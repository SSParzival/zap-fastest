//! Historical same-account merge. Imported transport work is retained without activation.

use std::{fs, path::Path};

use anyhow::{Result, ensure};
use rusqlite::params;

use super::{Archive, backup::PreparedRestore, encryption};
use crate::model::{Content, Delivery, Message};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergePreview {
    pub added: u64,
    pub edited: u64,
    pub conflicts: u64,
    pub protected: u64,
    pub removed: u64,
    pub imported_pending: u64,
}

pub struct PreparedMerge {
    restore: PreparedRestore,
    pub preview: MergePreview,
    revision: (u64, i64),
}

impl std::fmt::Debug for PreparedMerge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedMerge")
            .field("preview", &self.preview)
            .finish_non_exhaustive()
    }
}

fn identity(archive: &Archive) -> Result<String> {
    let own = archive.meta("me_pn")?.unwrap_or_default();
    let number = own.strip_suffix("@s.whatsapp.net").unwrap_or_default();
    ensure!(
        !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit()),
        "This archive has no verified account identity. Restore it separately instead."
    );
    Ok(own)
}

fn revision(archive: &Archive) -> Result<(u64, i64)> {
    Ok((
        archive.connection.total_changes(),
        archive
            .connection
            .pragma_query_value(None, "data_version", |row| row.get(0))?,
    ))
}

fn logically_equal(a: &Content, b: &Content) -> Result<bool> {
    fn normalize(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                map.remove("path");
                map.remove("state");
                for value in map.values_mut() {
                    normalize(value);
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    normalize(value);
                }
            }
            _ => {}
        }
    }
    let mut a = serde_json::to_value(a)?;
    let mut b = serde_json::to_value(b)?;
    normalize(&mut a);
    normalize(&mut b);
    Ok(a == b)
}

fn protected(target: &Archive, source: &Archive, row: &Message) -> Result<bool> {
    for archive in [target, source] {
        if removed(archive, &row.chat, &row.id)?
            || archive.export_deletion_pending(&row.chat, &row.id)?
            || archive
                .removal_point(&row.chat)?
                .is_some_and(|through| row.timestamp <= through)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn removed(archive: &Archive, chat: &str, id: &str) -> Result<bool> {
    Ok(archive.message_removed(chat, id)?
        || archive.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM message_removals WHERE account='' AND chat=?1 AND id=?2)",
            params![chat, id],
            |row| row.get::<_, bool>(0),
        )?)
}

enum Decision {
    Add,
    Edit,
    Enrich,
    Conflict,
    Protect,
}

fn decision(target: &Archive, source: &Archive, row: &Message) -> Result<Decision> {
    if protected(target, source, row)? {
        return Ok(Decision::Protect);
    }
    let Some(current) = target.message(&row.chat, &row.id)? else {
        return Ok(Decision::Add);
    };
    if current.sender != row.sender || current.from_me != row.from_me {
        return Ok(Decision::Conflict);
    }
    if current.status.is_local() {
        return Ok(Decision::Conflict);
    }
    if matches!(current.content, Content::Revoked) {
        return Ok(Decision::Enrich);
    }
    if matches!(row.content, Content::Revoked) {
        return Ok(Decision::Edit);
    }
    if row.edited && !current.edited {
        return Ok(Decision::Edit);
    }
    if logically_equal(&current.content, &row.content)? || (current.edited && !row.edited) {
        return Ok(Decision::Enrich);
    }
    // Live edits persist a flag but no revision timestamp. Two different edited snapshots
    // cannot be ordered honestly from original-message time or history position.
    Ok(Decision::Conflict)
}

impl PreparedRestore {
    fn merge_source(&self) -> Result<Archive> {
        Ok(Archive {
            connection: encryption::keyed(
                &self.staging.path().join("archive.db"),
                &self.database_key,
            )?,
        })
    }

    pub fn preview_merge(self, target: &Archive) -> Result<PreparedMerge> {
        let source = self.merge_source()?;
        ensure!(
            identity(target)? == identity(&source)?,
            "This backup belongs to a different WhatsApp account."
        );
        source.connection.execute(
            "UPDATE message_removals SET account=?1 WHERE account=''",
            [identity(&source)?],
        )?;
        ensure!(
            source.connection.query_row(
                "SELECT COUNT(*) FROM messages WHERE NOT json_valid(content)",
                [],
                |row| row.get::<_, i64>(0)
            )? == 0,
            "The backup contains invalid message snapshots. Restore it separately for repair."
        );
        for table in [
            "messages",
            "chats",
            "message_removals",
            "pending_message_removals",
            "chat_removals",
        ] {
            ensure!(
                columns(target, table)? == columns(&source, table)?,
                "Backup schema requires migration. Restore it separately before merging."
            );
        }
        let mut mappings: std::collections::BTreeMap<String, String> =
            source.lids()?.into_iter().collect();
        for (lid, pn) in target.lids()? {
            ensure!(
                mappings.get(&lid).is_none_or(|known| known == &pn),
                "Privacy identity mappings conflict. Restore this backup separately."
            );
            mappings.insert(lid, pn);
        }
        for (lid, pn) in mappings {
            source.put_lid(&lid, &pn)?;
        }
        let mut preview = MergePreview::default();
        let mut statement = source
            .connection
            .prepare("SELECT chat,id FROM messages ORDER BY rowid")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (chat, id) = row?;
            let Some(message) = source.message(&chat, &id)? else {
                continue;
            };
            match decision(target, &source, &message)? {
                Decision::Add => preview.added += 1,
                Decision::Edit => preview.edited += 1,
                Decision::Conflict => preview.conflicts += 1,
                Decision::Protect => preview.protected += 1,
                Decision::Enrich => {}
            }
        }
        for table in PENDING_TABLES {
            if !columns(&source, table)?.is_empty() {
                preview.imported_pending += source.connection.query_row(
                    &format!("SELECT COUNT(*) FROM {table}{}", pending_filter(table)),
                    [],
                    |row| row.get::<_, i64>(0),
                )? as u64;
            }
        }
        drop(statement);
        let mut statement = target.connection.prepare("SELECT chat,id FROM messages")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (chat, id) = row?;
            if let Some(message) = target.message(&chat, &id)?
                && (removed(&source, &chat, &id)?
                    || source
                        .removal_point(&chat)?
                        .is_some_and(|through| message.timestamp <= through))
            {
                preview.removed += 1;
            }
        }
        drop(statement);
        Ok(PreparedMerge {
            revision: revision(target)?,
            restore: self,
            preview,
        })
    }
}

const PENDING_TABLES: &[&str] = &[
    "outgoing_queue",
    "pending_message_removals",
    "star_changes",
    "favorite_changes",
    "chats",
    "archive_changes",
];

fn pending_filter(table: &str) -> &'static str {
    if table == "chats" {
        " WHERE pending_read IS NOT NULL OR pending_unread IS NOT NULL"
    } else {
        ""
    }
}

fn columns(archive: &Archive, table: &str) -> Result<Vec<String>> {
    let mut statement = archive
        .connection
        .prepare(&format!("PRAGMA table_info({table})"))?;
    Ok(statement
        .query_map([], |row| row.get(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?)
}

fn quarantine(
    target: &Archive,
    merge_id: &str,
    kind: &str,
    key: &str,
    payload: &serde_json::Value,
) -> Result<()> {
    target.connection.execute(
        "INSERT OR IGNORE INTO merge_quarantine(merge_id,kind,item,payload) VALUES(?1,?2,?3,?4)",
        params![merge_id, kind, key, serde_json::to_string(payload)?],
    )?;
    Ok(())
}

fn enrich(current: &mut Message, imported: &Message) -> Result<()> {
    current.history_order = current.history_order.or(imported.history_order);
    fn paths(a: &mut serde_json::Value, b: &serde_json::Value) {
        match (a, b) {
            (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
                if a.get("path").is_some_and(serde_json::Value::is_null)
                    && let Some(path) = b.get("path").filter(|path| path.is_string())
                {
                    a.insert("path".into(), path.clone());
                }
                for (key, value) in a.iter_mut() {
                    if let Some(other) = b.get(key) {
                        paths(value, other);
                    }
                }
            }
            (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
                for (a, b) in a.iter_mut().zip(b) {
                    paths(a, b);
                }
            }
            _ => {}
        }
    }
    if logically_equal(&current.content, &imported.content)? {
        let mut content = serde_json::to_value(&current.content)?;
        paths(&mut content, &serde_json::to_value(&imported.content)?);
        current.content = serde_json::from_value(content)?;
    }
    Ok(())
}

impl PreparedMerge {
    /// Revalidates the exact preview, then commits the historical merge as one database change.
    /// Session credentials and imported work queues never replace or activate current ones.
    pub fn apply(self, target: &Archive, media_root: &Path) -> Result<MergePreview> {
        ensure!(
            revision(target)? == self.revision,
            "The archive changed after this preview. Prepare the merge again."
        );
        let source = self.restore.merge_source()?;
        ensure!(
            identity(target)? == identity(&source)?,
            "Account identity changed after the preview."
        );
        // Blocked snapshots must not leave attachment copies behind after the merge.
        let mut blocked = Vec::new();
        {
            let mut statement = source.connection.prepare("SELECT chat,id FROM messages")?;
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            for row in rows {
                let (chat, id) = row?;
                if let Some(message) = source.message(&chat, &id)?
                    && protected(target, &source, &message)?
                {
                    blocked.push((chat, id));
                }
            }
        }
        for (chat, id) in blocked {
            source.connection.execute(
                "DELETE FROM messages WHERE chat=?1 AND id=?2",
                params![chat, id],
            )?;
        }
        fs::create_dir_all(media_root)?;
        let media = tempfile::Builder::new()
            .prefix("merged-backup-")
            .tempdir_in(media_root)?;
        let media_path = media.path().to_owned();
        super::backup::rewrite(&source.connection, |relative| {
            ensure!(
                relative.starts_with("media/")
                    && Path::new(relative)
                        .components()
                        .all(|part| matches!(part, std::path::Component::Normal(_))),
                "Invalid merged attachment path"
            );
            let input = self.restore.staging.path().join(relative);
            let output = media_path.join(relative);
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
            if !output.exists() {
                fs::copy(input, &output)?;
            }
            Ok(output.to_string_lossy().into_owned())
        })?;
        let merge_id = media_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let transaction = target.connection.unchecked_transaction()?;
        target.connection.execute_batch("CREATE TABLE IF NOT EXISTS merge_quarantine(merge_id TEXT NOT NULL,kind TEXT NOT NULL,item TEXT NOT NULL,payload TEXT NOT NULL,PRIMARY KEY(merge_id,kind,item));")?;
        // The input database was authenticated and has exactly the checked schema. Parameterized
        // attachment paths and keys stay inside this process and never enter logs or the preview.
        target.connection.execute(
            "ATTACH DATABASE ?1 AS merge_input KEY ?2",
            params![
                self.restore
                    .staging
                    .path()
                    .join("archive.db")
                    .to_string_lossy(),
                &*encryption::key_literal(&self.restore.database_key)
            ],
        )?;
        let merged = (|| -> Result<()> {
            let own = identity(target)?;
            target.connection.execute(
                "UPDATE message_removals SET account=?1 WHERE account=''",
                [&own],
            )?;
            target.connection.execute("INSERT INTO message_removals(account,chat,id) SELECT account,chat,id FROM merge_input.message_removals WHERE account=?1 ON CONFLICT(chat,id) DO NOTHING", [&own])?;
            target.connection.execute("INSERT INTO message_removals(account,chat,id) SELECT account,chat,id FROM merge_input.pending_message_removals WHERE account=?1 AND confirmed=1 ON CONFLICT(chat,id) DO NOTHING", [&own])?;
            target.connection.execute_batch("INSERT INTO chat_removals(chat,through) SELECT chat,through FROM merge_input.chat_removals WHERE 1 ON CONFLICT(chat) DO UPDATE SET through=MAX(chat_removals.through,excluded.through);")?;
            let names = columns(target, "chats")?;
            let chat_columns = names.join(",");
            let values = names
                .iter()
                .map(|name| {
                    if matches!(name.as_str(), "pending_read" | "pending_unread") {
                        "NULL"
                    } else {
                        name.as_str()
                    }
                })
                .collect::<Vec<_>>()
                .join(",");
            target.connection.execute_batch(&format!("INSERT OR IGNORE INTO chats({chat_columns}) SELECT {values} FROM merge_input.chats; UPDATE chats SET locked=1 WHERE id IN(SELECT id FROM merge_input.chats WHERE locked=1);"))?;
            let mut statement = source
                .connection
                .prepare("SELECT chat,id FROM messages ORDER BY rowid")?;
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            for row in rows {
                let (chat, id) = row?;
                let Some(mut message) = source.message(&chat, &id)? else {
                    continue;
                };
                let raw = source.raw(&chat, &id)?;
                match decision(target, &source, &message)? {
                    Decision::Add => {
                        if message.status.is_local() {
                            message.status = Delivery::Unconfirmed;
                        }
                        target.insert_message(&message, raw.as_deref())?;
                    }
                    Decision::Edit => {
                        let mut current = target.message(&chat, &id)?.unwrap();
                        let order = current.history_order.or(message.history_order);
                        message.history_order = order;
                        message.timestamp = current.timestamp;
                        std::mem::swap(&mut current.content, &mut message.content);
                        current.edited = message.edited;
                        current.history_order = order;
                        current.mentions = message.mentions;
                        target.insert_message(&current, raw.as_deref())?;
                    }
                    Decision::Enrich => {
                        let mut current = target.message(&chat, &id)?.unwrap();
                        enrich(&mut current, &message)?;
                        let current_raw = target.raw(&chat, &id)?;
                        target
                            .insert_message(&current, current_raw.as_deref().or(raw.as_deref()))?;
                    }
                    Decision::Conflict => quarantine(
                        target,
                        &merge_id,
                        "message",
                        &format!("{chat}:{id}"),
                        &serde_json::json!({"message": message,"raw":raw}),
                    )?,
                    Decision::Protect => {}
                }
            }
            // Retain imported pending state as encrypted JSON, without copying it into active queues.
            for table in PENDING_TABLES {
                let names = columns(&source, table)?;
                if names.is_empty() {
                    continue;
                }
                let mut statement = source.connection.prepare(&format!(
                    "SELECT rowid,* FROM {table}{}",
                    pending_filter(table)
                ))?;
                let mut rows = statement.query([])?;
                while let Some(row) = rows.next()? {
                    let key = row.get::<_, i64>(0)?.to_string();
                    let mut payload = serde_json::Map::new();
                    for (index, name) in names.iter().enumerate() {
                        use rusqlite::types::ValueRef;
                        let value = match row.get_ref(index + 1)? {
                            ValueRef::Null => serde_json::Value::Null,
                            ValueRef::Integer(value) => value.into(),
                            ValueRef::Real(value) => serde_json::json!(value),
                            ValueRef::Text(value) => {
                                String::from_utf8_lossy(value).into_owned().into()
                            }
                            ValueRef::Blob(value) => serde_json::json!(value),
                        };
                        payload.insert(name.clone(), value);
                    }
                    quarantine(target, &merge_id, table, &key, &payload.into())?;
                }
            }
            for table in [
                "contacts",
                "lids",
                "motion_clips",
                "group_receipts",
                "transcriptions",
                "polls",
                "poll_votes",
                "poll_history",
                "synced_call_log",
            ] {
                let names = columns(&source, table)?;
                if names.is_empty() || names != columns(target, table)? {
                    continue;
                }
                let names = names.join(",");
                target.connection.execute_batch(&format!("INSERT OR IGNORE INTO {table}({names}) SELECT {names} FROM merge_input.{table};"))?;
            }
            target.connection.execute("DELETE FROM messages WHERE EXISTS(SELECT 1 FROM message_removals r WHERE r.chat=messages.chat AND r.id=messages.id AND r.account=?1) OR EXISTS(SELECT 1 FROM chat_removals r WHERE r.chat=messages.chat AND messages.timestamp<=r.through)", [&own])?;
            Ok(())
        })();
        if let Err(error) = merged {
            drop(transaction);
            let _ = target
                .connection
                .execute_batch("DETACH DATABASE merge_input;");
            return Err(error);
        }
        if let Err(error) = transaction.commit() {
            let _ = target
                .connection
                .execute_batch("DETACH DATABASE merge_input;");
            return Err(error.into());
        }
        // Files are owned independently before publication; rollback automatically removes them.
        let _retained = media.keep();
        // A cleanup failure cannot turn an already committed merge into a failed operation.
        let _ = target
            .connection
            .execute_batch("DETACH DATABASE merge_input;");
        Ok(self.preview)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archive::{backup::prepare_restore, tests::message};

    fn fixture() -> Archive {
        let archive = Archive::in_memory().unwrap();
        archive.set_meta("me_pn", "123456@s.whatsapp.net").unwrap();
        archive.ensure_chat("fixture", "Synthetic").unwrap();
        archive
    }

    fn prepare(source: &Archive, target: &Archive, root: &Path) -> PreparedMerge {
        let package = root.join("synthetic.zfbackup");
        source
            .export_backup(&package, "synthetic passphrase", "fixture", root)
            .unwrap();
        prepare_restore(&package, "synthetic passphrase", root)
            .unwrap()
            .preview_merge(target)
            .unwrap()
    }

    #[test]
    fn merge_unites_history_preserves_revisions_and_quarantines_pending_work() {
        let root = tempfile::tempdir().unwrap();
        let target = fixture();
        let source = fixture();
        target
            .insert_message(&message("fixture", "current", 10, false), None)
            .unwrap();
        let mut old = message("fixture", "edit", 20, false);
        target.insert_message(&old, None).unwrap();
        old.content = Content::text("Edited synthetic snapshot");
        old.edited = true;
        source.insert_message(&old, None).unwrap();
        source
            .insert_message(&message("fixture", "imported", 30, true), None)
            .unwrap();
        source
            .connection
            .execute("UPDATE chats SET pending_read=30,pending_unread=1", [])
            .unwrap();
        let prepared = prepare(&source, &target, root.path());
        assert_eq!(prepared.preview.added, 1);
        assert_eq!(prepared.preview.edited, 1);
        assert_eq!(prepared.preview.imported_pending, 1);
        prepared.apply(&target, &root.path().join("cache")).unwrap();
        assert!(target.message("fixture", "current").unwrap().is_some());
        assert!(target.message("fixture", "edit").unwrap().unwrap().edited);
        assert_eq!(
            target
                .message("fixture", "imported")
                .unwrap()
                .unwrap()
                .status,
            Delivery::Unconfirmed
        );
        assert!(target.pending_reads().unwrap().is_empty());
        assert_eq!(
            target
                .connection
                .query_row(
                    "SELECT COUNT(*) FROM merge_quarantine WHERE kind='chats'",
                    [],
                    |row| row.get::<_, i64>(0)
                )
                .unwrap(),
            1
        );
    }

    #[test]
    fn incoming_deletions_and_existing_revocations_prevent_resurrection() {
        let root = tempfile::tempdir().unwrap();
        let target = fixture();
        let source = fixture();
        target
            .insert_message(&message("fixture", "removed", 10, false), None)
            .unwrap();
        source.delete_message("fixture", "removed").unwrap();
        target
            .insert_message(&message("fixture", "legacy", 10, false), None)
            .unwrap();
        source
            .connection
            .execute(
                "INSERT INTO message_removals(account,chat,id) VALUES('','fixture','legacy')",
                [],
            )
            .unwrap();
        target
            .insert_message(&message("fixture", "previous", 10, false), None)
            .unwrap();
        source.connection.execute("INSERT INTO message_removals(account,chat,id) VALUES('999999@s.whatsapp.net','fixture','previous')",[]).unwrap();
        let mut revoked = message("fixture", "revoked", 20, false);
        revoked.content = Content::Revoked;
        target.insert_message(&revoked, None).unwrap();
        let mut stale = message("fixture", "revoked", 20, false);
        stale.edited = true;
        source.insert_message(&stale, None).unwrap();
        let prepared = prepare(&source, &target, root.path());
        assert_eq!(prepared.preview.removed, 2);
        prepared.apply(&target, &root.path().join("cache")).unwrap();
        assert!(target.message("fixture", "removed").unwrap().is_none());
        assert!(target.message("fixture", "legacy").unwrap().is_none());
        assert!(target.message("fixture", "previous").unwrap().is_some());
        assert!(matches!(
            target
                .message("fixture", "revoked")
                .unwrap()
                .unwrap()
                .content,
            Content::Revoked
        ));
    }

    #[test]
    fn preview_rejects_account_mismatch_and_current_archive_changes() {
        let root = tempfile::tempdir().unwrap();
        let target = fixture();
        let source = fixture();
        source
            .insert_message(&message("fixture", "imported", 10, false), None)
            .unwrap();
        let prepared = prepare(&source, &target, root.path());
        target.set_meta("changed", "yes").unwrap();
        assert!(prepared.apply(&target, &root.path().join("cache")).is_err());
        assert!(target.message("fixture", "imported").unwrap().is_none());
        source.set_meta("me_pn", "999999@s.whatsapp.net").unwrap();
        let package = root.path().join("different.zfbackup");
        source
            .export_backup(&package, "synthetic passphrase", "fixture", root.path())
            .unwrap();
        assert!(
            prepare_restore(&package, "synthetic passphrase", root.path())
                .unwrap()
                .preview_merge(&target)
                .is_err()
        );
    }
}

//! Compressed history staged in the encrypted account archive before indexing.
//! Capture precedes the protocol receipt. Unindexed chunks replay after a crash.

use super::Archive;
use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, OptionalExtension, params};
use sha2::{Digest, Sha256};
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use whatsapp_rust::{Client, HistorySyncCaptureHook, prelude::wa};

pub(super) const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS captured_history (
    id TEXT PRIMARY KEY, digest BLOB NOT NULL, kind INTEGER,
    compressed BLOB NOT NULL, indexed INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS history_capture_identity (
    singleton INTEGER PRIMARY KEY CHECK(singleton=1), generation INTEGER NOT NULL
);
INSERT OR IGNORE INTO history_capture_identity(singleton,generation) VALUES (1,0);
CREATE TABLE IF NOT EXISTS account_cleanup (
    singleton INTEGER PRIMARY KEY CHECK(singleton=1), required INTEGER NOT NULL
);
INSERT OR IGNORE INTO account_cleanup(singleton,required) VALUES (1,0);";

/// Separate connection, with no startup migrations or outgoing-message recovery.
pub struct HistoryCapture(Arc<Mutex<Connection>>, i64);

pub type CapturedHistory = (i64, Option<i32>, Vec<u8>);

impl HistoryCapture {
    pub fn open(path: &Path) -> Result<Self> {
        let key = super::encryption::key_for(path)?;
        let connection = super::encryption::keyed(path, &key)?;
        connection.busy_timeout(Duration::from_secs(10))?;
        connection.execute_batch("PRAGMA synchronous=FULL;")?;
        connection.execute_batch(SCHEMA)?;
        let generation = Self::generation(&connection)?;
        Ok(Self(Arc::new(Mutex::new(connection)), generation))
    }

    fn generation(connection: &Connection) -> rusqlite::Result<i64> {
        connection.query_row(
            "SELECT generation FROM history_capture_identity WHERE singleton=1",
            [],
            |row| row.get(0),
        )
    }

    fn save(
        connection: &Connection,
        generation: i64,
        id: &str,
        kind: Option<i32>,
        bytes: &[u8],
    ) -> Result<()> {
        // Check the identity and write under one transaction. An old owner can
        // either commit before logout (which clears it) or fail after logout,
        // but cannot publish into the next identity's archive.
        let transaction = connection.unchecked_transaction()?;
        ensure!(
            Self::generation(&transaction)? == generation,
            "History capture belongs to a previous account session"
        );
        ensure!(
            !transaction.query_row(
                "SELECT required FROM account_cleanup WHERE singleton=1",
                [],
                |row| row.get::<_, bool>(0)
            )?,
            "Account cleanup must finish before history capture"
        );
        let digest = Sha256::digest(bytes);
        let prior: Option<Vec<u8>> = transaction
            .query_row(
                "SELECT digest FROM captured_history WHERE id=?1",
                [id],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(prior) = prior {
            ensure!(
                prior.as_slice() == digest.as_slice(),
                "History notification changed its payload"
            );
            transaction.commit()?;
            return Ok(());
        }
        transaction.execute(
            "INSERT INTO captured_history(id,digest,kind,compressed) VALUES (?1,?2,?3,?4)",
            params![id, digest.as_slice(), kind, bytes],
        )?;
        transaction.commit()?;
        Ok(())
    }
}

#[whatsapp_rust::async_trait]
impl HistorySyncCaptureHook for HistoryCapture {
    async fn on_history_sync(
        &self,
        _client: Arc<Client>,
        message_id: &str,
        sync_type: Option<wa::message::HistorySyncType>,
        compressed: &[u8],
    ) -> Result<()> {
        let connection = self.0.clone();
        let id = message_id.to_owned();
        let bytes = compressed.to_vec();
        let kind = sync_type.map(|kind| kind as i32);
        let generation = self.1;
        tokio::task::spawn_blocking(move || {
            let connection = connection
                .lock()
                .map_err(|_| anyhow::anyhow!("History capture lock failed"))?;
            Self::save(&connection, generation, &id, kind, &bytes)
        })
        .await
        .context("History capture task failed")?
    }
}

impl Archive {
    pub fn has_account_data(&self) -> super::Result<bool> {
        self.connection.query_row("SELECT EXISTS(SELECT 1 FROM chats UNION ALL SELECT 1 FROM messages UNION ALL SELECT 1 FROM contacts UNION ALL SELECT 1 FROM captured_history UNION ALL SELECT 1 FROM stickers UNION ALL SELECT 1 FROM lids UNION ALL SELECT 1 FROM motion_clips UNION ALL SELECT 1 FROM call_log UNION ALL SELECT 1 FROM synced_call_log UNION ALL SELECT 1 FROM polls UNION ALL SELECT 1 FROM poll_votes UNION ALL SELECT 1 FROM favorites UNION ALL SELECT 1 FROM favorite_stickers UNION ALL SELECT 1 FROM meta WHERE key='me_pn')", [], |row| row.get(0))
    }
    /// Keep this intent until both the archive and account-private files are gone.
    pub fn require_account_cleanup(&self) -> super::Result<()> {
        let transaction = self.connection.unchecked_transaction()?;
        transaction.execute(
            "UPDATE account_cleanup SET required=1 WHERE singleton=1",
            [],
        )?;
        transaction.execute(
            "UPDATE history_capture_identity SET generation=generation+1 WHERE singleton=1",
            [],
        )?;
        transaction.commit()
    }

    pub fn account_cleanup_required(&self) -> super::Result<bool> {
        self.connection.query_row(
            "SELECT required FROM account_cleanup WHERE singleton=1",
            [],
            |row| row.get(0),
        )
    }

    pub fn finish_account_cleanup(&self) -> super::Result<()> {
        self.connection.execute(
            "UPDATE account_cleanup SET required=0 WHERE singleton=1",
            [],
        )?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn reject_account_cleanup_fixture(&self, reject: bool) {
        self.connection.execute_batch(if reject {
            "CREATE TRIGGER reject_account_cleanup BEFORE DELETE ON captured_history BEGIN SELECT RAISE(ABORT,'fixture failure'); END;"
        } else { "DROP TRIGGER reject_account_cleanup" }).unwrap();
    }

    #[cfg(test)]
    pub(crate) fn motion_clip_fixture_count(&self) -> i64 {
        self.connection
            .query_row("SELECT COUNT(*) FROM motion_clips", [], |row| row.get(0))
            .unwrap()
    }
    #[cfg(test)]
    pub(crate) fn stage_history_fixture(&self, id: &str, compressed: &[u8]) {
        HistoryCapture::save(
            &self.connection,
            HistoryCapture::generation(&self.connection).unwrap(),
            id,
            None,
            compressed,
        )
        .unwrap();
    }

    #[cfg(test)]
    pub(crate) fn reject_history_fixture_writes(&self, table: &str, reject: bool) {
        assert!(matches!(
            table,
            "captured_history" | "motion_clips" | "stickers" | "messages"
        ));
        let operation = if matches!(table, "captured_history" | "messages") {
            "UPDATE"
        } else {
            "INSERT"
        };
        self.connection.execute_batch(&if reject {
            format!("CREATE TRIGGER reject_history_fixture BEFORE {operation} ON {table} BEGIN SELECT RAISE(ABORT,'fixture failure'); END;")
        } else {
            "DROP TRIGGER reject_history_fixture".into()
        }).unwrap();
    }

    pub fn next_captured_history(&self, after: i64) -> super::Result<Option<CapturedHistory>> {
        self.connection.query_row("SELECT rowid,kind,compressed FROM captured_history WHERE indexed=0 AND rowid>?1 ORDER BY rowid LIMIT 1",
            [after], |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?))).optional()
    }

    /// Release the staging payload after message rows have been read back.
    /// Keep only the digest to deduplicate future receipts, avoiding a second
    /// retained copy of messages the person may subsequently delete.
    pub fn history_indexed(&self, compressed: &[u8]) -> super::Result<()> {
        self.connection.execute(
            "UPDATE captured_history SET indexed=1,compressed=X'' WHERE digest=?1",
            params![Sha256::digest(compressed).as_slice()],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn logout_cleanup_intent_survives_restart_and_fences_new_capture_owners() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("synthetic.db");
        let archive = Archive::prepare(Connection::open(&path).unwrap()).unwrap();
        archive.stage_history_fixture("old", b"synthetic");
        archive.require_account_cleanup().unwrap();
        archive.reject_account_cleanup_fixture(true);
        assert!(archive.clear().is_err());
        drop(archive);
        let reopened = Archive::prepare(Connection::open(&path).unwrap()).unwrap();
        assert!(reopened.account_cleanup_required().unwrap());
        let generation = HistoryCapture::generation(&reopened.connection).unwrap();
        assert!(
            HistoryCapture::save(
                &reopened.connection,
                generation,
                "new-owner",
                None,
                b"synthetic"
            )
            .is_err()
        );
        reopened.reject_account_cleanup_fixture(false);
        reopened.clear().unwrap();
        assert!(
            reopened.account_cleanup_required().unwrap(),
            "file cleanup still needs to finish"
        );
        reopened.finish_account_cleanup().unwrap();
        assert!(!reopened.account_cleanup_required().unwrap());
        assert!(!reopened.has_account_data().unwrap());
    }

    #[test]
    fn logout_fences_late_capture_writers_and_preserves_failed_clear_atomically() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("synthetic.db");
        let archive = Archive::prepare(Connection::open(&path).unwrap()).unwrap();
        let old_writer = Connection::open(&path).unwrap();
        let old_generation = HistoryCapture::generation(&old_writer).unwrap();
        HistoryCapture::save(
            &old_writer,
            old_generation,
            "old",
            None,
            b"old synthetic history",
        )
        .unwrap();
        archive.connection.execute_batch("CREATE TRIGGER reject_capture_clear BEFORE DELETE ON captured_history BEGIN SELECT RAISE(ABORT,'fixture failure'); END;").unwrap();
        assert!(archive.clear().is_err());
        assert_eq!(
            HistoryCapture::generation(&old_writer).unwrap(),
            old_generation
        );
        assert!(archive.next_captured_history(0).unwrap().is_some());
        archive
            .connection
            .execute_batch("DROP TRIGGER reject_capture_clear")
            .unwrap();
        archive.clear().unwrap();
        assert!(archive.next_captured_history(0).unwrap().is_none());
        assert!(
            HistoryCapture::save(
                &old_writer,
                old_generation,
                "late",
                None,
                b"late old synthetic history"
            )
            .is_err()
        );
        let new_generation = HistoryCapture::generation(&archive.connection).unwrap();
        assert_ne!(new_generation, old_generation);
        HistoryCapture::save(
            &archive.connection,
            new_generation,
            "new",
            None,
            b"new synthetic history",
        )
        .unwrap();
        assert_eq!(
            archive.next_captured_history(0).unwrap().unwrap().2,
            b"new synthetic history"
        );
    }

    #[test]
    fn durable_source_is_idempotent_and_compacts_after_indexing() {
        let archive = Archive::in_memory().unwrap();
        HistoryCapture::save(
            &archive.connection,
            0,
            "fixture",
            Some(6),
            b"compressed-fixture",
        )
        .unwrap();
        HistoryCapture::save(
            &archive.connection,
            0,
            "fixture",
            Some(6),
            b"compressed-fixture",
        )
        .unwrap();
        assert!(
            HistoryCapture::save(&archive.connection, 0, "fixture", Some(6), b"changed").is_err()
        );
        assert!(archive.next_captured_history(0).unwrap().is_some());
        archive.history_indexed(b"compressed-fixture").unwrap();
        assert!(archive.next_captured_history(0).unwrap().is_none());
        assert_eq!(
            archive
                .connection
                .query_row("SELECT COUNT(*) FROM captured_history", [], |row| row
                    .get::<_, u32>(0))
                .unwrap(),
            1
        );
    }
}

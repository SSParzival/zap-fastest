//! Message-specific expiry and timestamp-ordered keep decisions.
use super::{Archive, Result};
use rusqlite::params;

pub(super) const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS disappearing_messages (
    chat TEXT NOT NULL,id TEXT NOT NULL,expires_at INTEGER,kept INTEGER NOT NULL DEFAULT 0,
    keep_at INTEGER NOT NULL DEFAULT 0,PRIMARY KEY(chat,id));
    CREATE INDEX IF NOT EXISTS disappearing_deadline ON disappearing_messages(expires_at);";

impl Archive {
    pub fn disappearing_cached_media(
        &self,
        chat: &str,
        id: &str,
    ) -> Result<Vec<std::path::PathBuf>> {
        self.cached_media("m.chat=?1 AND m.id=?2", params![chat, id])
    }
    pub fn remember_message_expiry(&self, chat: &str, id: &str, expires_at: i64) -> Result<()> {
        self.connection.execute("INSERT INTO disappearing_messages(chat,id,expires_at) VALUES (?1,?2,?3)
            ON CONFLICT(chat,id) DO UPDATE SET expires_at=COALESCE(disappearing_messages.expires_at,excluded.expires_at)", params![chat,id,expires_at])?;
        Ok(())
    }

    pub fn remember_message_keep(
        &self,
        chat: &str,
        id: &str,
        kept: bool,
        at_ms: i64,
    ) -> Result<()> {
        self.connection.execute(
            "INSERT INTO disappearing_messages(chat,id,kept,keep_at) VALUES (?1,?2,?3,?4)
            ON CONFLICT(chat,id) DO UPDATE SET kept=excluded.kept,keep_at=excluded.keep_at
            WHERE excluded.keep_at > disappearing_messages.keep_at OR
            (excluded.keep_at=disappearing_messages.keep_at AND excluded.kept=1)",
            params![chat, id, kept, at_ms],
        )?;
        Ok(())
    }

    pub fn due_disappearing_messages(&self, now: i64) -> Result<Vec<(String, String)>> {
        self.connection.prepare("SELECT d.chat,d.id FROM disappearing_messages d JOIN messages m ON m.chat=d.chat AND m.id=d.id
            WHERE d.expires_at<=?1 AND d.kept=0 AND m.status NOT IN (-3,-1,1,6)
            ORDER BY d.expires_at LIMIT 128")?.query_map([now], |row| Ok((row.get(0)?,row.get(1)?)))?.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keep_replay_and_expiry_preserve_later_decisions_and_deletion_barriers() {
        let archive = Archive::in_memory().unwrap();
        let row = super::super::tests::message("15550000001@s.whatsapp.net", "fixture", 100, false);
        archive.insert_message(&row, None).unwrap();
        archive
            .remember_message_expiry(&row.chat, &row.id, 200)
            .unwrap();
        assert!(archive.due_disappearing_messages(199).unwrap().is_empty());
        assert_eq!(archive.due_disappearing_messages(200).unwrap().len(), 1);
        archive
            .remember_message_keep(&row.chat, &row.id, true, 300)
            .unwrap();
        archive
            .remember_message_keep(&row.chat, &row.id, false, 299)
            .unwrap();
        assert!(archive.due_disappearing_messages(999).unwrap().is_empty());
        archive
            .remember_message_keep(&row.chat, &row.id, false, 301)
            .unwrap();
        assert_eq!(archive.due_disappearing_messages(999).unwrap().len(), 1);
        archive.delete_message(&row.chat, &row.id).unwrap();
        archive.insert_message(&row, None).unwrap();
        assert!(archive.message(&row.chat, &row.id).unwrap().is_none());
    }
}

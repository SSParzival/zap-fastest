//! Durable automation request identities prevent repeated CLI commands from sending twice.
use super::*;
use sha2::{Digest, Sha256};
pub type CliWatchPage = (Vec<Message>, Option<(i64, String)>);
impl Archive {
    fn cli_schema(&self) -> Result<()> {
        self.connection.execute_batch("CREATE TABLE IF NOT EXISTS automation_sends(request_id TEXT PRIMARY KEY,chat TEXT NOT NULL,digest TEXT NOT NULL,message_id TEXT NOT NULL);")
    }
    pub fn cli_existing_send(
        &self,
        request: &str,
        chat: &str,
        text: Option<&str>,
    ) -> Result<Option<String>> {
        self.cli_schema()?;
        let row: Option<(String, String, String)> = self
            .connection
            .query_row(
                "SELECT chat,digest,message_id FROM automation_sends WHERE request_id=?1",
                [request],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;
        if let Some((old_chat, digest, id)) = row {
            if old_chat != chat
                || text.is_some_and(|text| {
                    Sha256::digest(text.as_bytes())
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect::<String>()
                        != digest
                })
            {
                return Err(rusqlite::Error::InvalidQuery);
            }
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }
    pub fn cli_reserve_send(&self, request: &str, chat: &str, text: &str, id: &str) -> Result<()> {
        self.cli_schema()?;
        self.connection.execute(
            "INSERT INTO automation_sends(request_id,chat,digest,message_id) VALUES (?1,?2,?3,?4)",
            params![
                request,
                chat,
                Sha256::digest(text.as_bytes())
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>(),
                id
            ],
        )?;
        Ok(())
    }
    pub fn cli_watch(
        &self,
        chat: &str,
        cursor: Option<&(i64, String)>,
        limit: usize,
    ) -> Result<CliWatchPage> {
        let after = if let Some((rowid, id)) = cursor {
            let valid: bool = self.connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM messages WHERE chat=?1 AND id=?2 AND rowid=?3)",
                params![chat, id, rowid],
                |r| r.get(0),
            )?;
            if !valid {
                return Err(rusqlite::Error::InvalidQuery);
            }
            *rowid
        } else {
            0
        };
        let entries: Vec<(i64, String)> = self
            .connection
            .prepare(
                "SELECT rowid,id FROM messages WHERE chat=?1 AND rowid>?2 ORDER BY rowid LIMIT ?3",
            )?
            .query_map(params![chat, after, limit as i64], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })?
            .collect::<Result<_>>()?;
        let next = entries.last().cloned().or_else(|| cursor.cloned());
        let rows = entries
            .into_iter()
            .map(|(_, id)| self.message(chat, &id))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok((rows, next))
    }
    pub fn cli_exportable(&self, chat: &str, id: &str) -> Result<bool> {
        let disappearing:bool=self.connection.query_row("SELECT EXISTS(SELECT 1 FROM disappearing_messages WHERE chat=?1 AND id=?2 AND expires_at IS NOT NULL)",params![chat,id],|r|r.get(0))?;
        Ok(!disappearing)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_request_ids_preserve_send_identity_and_reject_changed_payload() {
        let a = Archive::in_memory().unwrap();
        a.cli_reserve_send("fixture", "chat", "synthetic body", "original")
            .unwrap();
        assert_eq!(
            a.cli_existing_send("fixture", "chat", Some("synthetic body"))
                .unwrap(),
            Some("original".into())
        );
        assert!(
            a.cli_existing_send("fixture", "chat", Some("changed synthetic body"))
                .is_err()
        );
        assert!(a.cli_existing_send("fixture", "other-chat", None).is_err());
    }
}

//! Encrypted account-local call outcomes; unknown duration stays absent.

use super::{Archive, Result};
use crate::calls::CallOutcome;
use crate::model::CallDirection;
use rusqlite::params;

pub(super) const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS call_log (
    id TEXT PRIMARY KEY, chat TEXT NOT NULL, direction TEXT NOT NULL,
    outcome TEXT NOT NULL, at INTEGER NOT NULL, duration INTEGER
); CREATE INDEX IF NOT EXISTS call_log_time ON call_log(at DESC);
CREATE TABLE IF NOT EXISTS synced_call_log (
    id TEXT PRIMARY KEY, creator TEXT, from_me INTEGER, raw BLOB NOT NULL
);";

#[derive(Clone, Debug)]
pub struct CallRecord {
    pub id: String,
    pub chat: String,
    pub direction: CallDirection,
    pub outcome: CallOutcome,
    pub at: i64,
    pub duration: Option<u64>,
}

type CallBackfillRow = (String, String, Vec<u8>);

impl Archive {
    pub fn unsupported_call_rows(&self) -> Result<Vec<CallBackfillRow>> {
        self.connection
            .prepare(
                "SELECT chat,id,raw FROM messages WHERE raw IS NOT NULL AND edited=0
            AND json_valid(content) AND json_extract(content,'$.kind')='unsupported'
            AND json_extract(content,'$.what')='call'",
            )?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect()
    }

    pub fn backfill_call_content(
        &self,
        chat: &str,
        id: &str,
        content: &crate::model::Content,
    ) -> Result<()> {
        let content = serde_json::to_string(content)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        self.connection.execute(
            "UPDATE messages SET content=?3 WHERE chat=?1 AND id=?2 AND edited=0
            AND json_valid(content) AND json_extract(content,'$.kind')='unsupported'
            AND json_extract(content,'$.what')='call'",
            params![chat, id, content],
        )?;
        Ok(())
    }
    pub fn has_local_call(&self, id: &str) -> Result<bool> {
        self.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM call_log WHERE id=?1)",
            [id],
            |row| row.get(0),
        )
    }
    /// Retain unresolved protocol metadata without inventing a peer or a timestamp.
    pub fn save_synced_call_raw(
        &self,
        id: &str,
        creator: Option<&str>,
        from_me: Option<bool>,
        raw: &[u8],
    ) -> Result<()> {
        self.connection.execute(
            "INSERT INTO synced_call_log(id,creator,from_me,raw) VALUES (?1,?2,?3,?4)
            ON CONFLICT(id) DO UPDATE SET creator=COALESCE(excluded.creator,creator),
            from_me=COALESCE(excluded.from_me,from_me),raw=excluded.raw",
            params![id, creator, from_me, raw],
        )?;
        Ok(())
    }

    pub fn save_call_record(&self, record: &CallRecord) -> Result<()> {
        let direction = serde_json::to_string(&record.direction)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        let outcome = serde_json::to_string(&record.outcome)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        let duration = record
            .duration
            .map(i64::try_from)
            .transpose()
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        self.connection.execute(
            "INSERT INTO call_log(id,chat,direction,outcome,at,duration)
             VALUES (?1,?2,?3,?4,?5,?6) ON CONFLICT(id) DO NOTHING",
            params![
                record.id,
                record.chat,
                direction,
                outcome,
                record.at,
                duration
            ],
        )?;
        Ok(())
    }

    pub fn call_records(&self) -> Result<Vec<CallRecord>> {
        self.connection
            .prepare(
                "SELECT q.id,q.chat,q.direction,q.outcome,q.at,q.duration FROM call_log q
             WHERE NOT EXISTS(SELECT 1 FROM chats c WHERE c.id=q.chat AND c.locked=1)
             ORDER BY q.at DESC, q.id DESC LIMIT 100",
            )?
            .query_map([], |row| {
                let direction: String = row.get(2)?;
                let outcome: String = row.get(3)?;
                let duration: Option<i64> = row.get(5)?;
                Ok(CallRecord {
                    id: row.get(0)?,
                    chat: row.get(1)?,
                    direction: serde_json::from_str(&direction).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    outcome: serde_json::from_str(&outcome).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            3,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    at: row.get(4)?,
                    duration: duration.map(u64::try_from).transpose().map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Integer,
                            Box::new(error),
                        )
                    })?,
                })
            })?
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, chat: &str, duration: Option<u64>) -> CallRecord {
        CallRecord {
            id: id.into(),
            chat: chat.into(),
            direction: CallDirection::Outgoing,
            outcome: CallOutcome::Answered,
            at: 100,
            duration,
        }
    }

    #[test]
    fn duration_round_trips_without_inventing_zero_for_unanswered_calls() {
        let archive = Archive::in_memory().unwrap();
        archive
            .save_call_record(&record("answered", "fixture@s.whatsapp.net", Some(42)))
            .unwrap();
        archive
            .save_call_record(&record("unknown", "fixture@s.whatsapp.net", None))
            .unwrap();
        let records = archive.call_records().unwrap();
        assert_eq!(
            records
                .iter()
                .find(|record| record.id == "answered")
                .unwrap()
                .duration,
            Some(42)
        );
        assert_eq!(
            records
                .iter()
                .find(|record| record.id == "unknown")
                .unwrap()
                .duration,
            None
        );
        assert!(
            archive
                .save_call_record(&record(
                    "overflow",
                    "fixture@s.whatsapp.net",
                    Some(u64::MAX)
                ))
                .is_err()
        );
    }

    #[test]
    fn duplicate_outcomes_do_not_replace_history_and_locked_chats_are_hidden() {
        let archive = Archive::in_memory().unwrap();
        let chat = "fixture@s.whatsapp.net";
        archive.ensure_chat(chat, "Fixture").unwrap();
        archive
            .save_call_record(&record("call", chat, Some(42)))
            .unwrap();
        archive
            .save_call_record(&record("call", chat, Some(0)))
            .unwrap();
        assert_eq!(archive.call_records().unwrap()[0].duration, Some(42));
        archive.set_locked(chat, true).unwrap();
        assert!(archive.call_records().unwrap().is_empty());
        archive.set_locked(chat, false).unwrap();
        assert_eq!(archive.call_records().unwrap().len(), 1);
    }

    #[test]
    fn logout_clears_unresolved_call_sources_as_well_as_visible_records() {
        let archive = Archive::in_memory().unwrap();
        assert!(!archive.has_account_data().unwrap());
        archive
            .save_synced_call_raw("unresolved", None, None, b"synthetic")
            .unwrap();
        assert!(archive.has_account_data().unwrap());
        archive
            .save_call_record(&record("visible", "fixture@s.whatsapp.net", None))
            .unwrap();
        archive.clear().unwrap();
        assert!(!archive.has_account_data().unwrap());
        assert!(archive.call_records().unwrap().is_empty());
        let remaining: i64 = archive
            .connection
            .query_row("SELECT COUNT(*) FROM synced_call_log", [], |row| row.get(0))
            .unwrap();
        assert_eq!(remaining, 0);
    }

    #[test]
    fn corrupt_negative_duration_is_reported() {
        let archive = Archive::in_memory().unwrap();
        archive
            .save_call_record(&record("call", "fixture@s.whatsapp.net", Some(42)))
            .unwrap();
        archive
            .connection
            .execute("UPDATE call_log SET duration=-1", [])
            .unwrap();
        assert!(archive.call_records().is_err());
    }
}

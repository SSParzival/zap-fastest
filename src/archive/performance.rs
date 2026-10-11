//! Opt-in repeatable synthetic archive measurements, never the user's archive.
use super::*;
use std::time::Instant;

#[test]
#[ignore = "manual synthetic performance measurement"]
fn measure_synthetic_archive() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("synthetic.db");
    let key = [0x51; 32];
    let start = Instant::now();
    let archive = Archive::open_with_key(&path, &key).unwrap();
    let new_ms = start.elapsed().as_secs_f64() * 1000.0;
    eprintln!("SYNTHETIC_MEASUREMENT_PID={}", std::process::id());
    let start = Instant::now();
    for index in 0..100 {
        archive
            .upsert_chat(&Chat::new(
                format!("synthetic-{index}@s.whatsapp.net"),
                format!("Synthetic {index}"),
            ))
            .unwrap();
    }
    let raw = vec![0x42; 512];
    for index in 0..10_000 {
        let chat = format!("synthetic-{}@s.whatsapp.net", index % 100);
        let message = Message {
            id: format!("fixture-{index}"),
            chat: chat.clone(),
            sender: chat,
            sender_name: None,
            from_me: false,
            timestamp: index,
            history_order: Some(index),
            content: Content::text(
                "Synthetic archive measurement. No personal data. العربية 中文 עברית 🟢",
            ),
            status: Delivery::None,
            delivered_at: None,
            read_at: None,
            quoted: None,
            reactions: Vec::new(),
            edited: false,
            mentions: Vec::new(),
            forwarded: false,
            thumbnail: None,
        };
        archive.insert_message(&message, Some(&raw)).unwrap();
    }
    let ingest_ms = start.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(
        archive
            .connection
            .query_row("SELECT count(*) FROM messages", [], |row| row
                .get::<_, u32>(0))
            .unwrap(),
        10_000
    );
    let start = Instant::now();
    for index in 0..100 {
        assert_eq!(
            archive
                .messages(&format!("synthetic-{index}@s.whatsapp.net"), None, 50)
                .unwrap()
                .len(),
            50
        );
    }
    let pages_ms = start.elapsed().as_secs_f64() * 1000.0;
    let start = Instant::now();
    assert_eq!(
        archive.search_messages("Synthetic", 100).unwrap().len(),
        100
    );
    let search_ms = start.elapsed().as_secs_f64() * 1000.0;
    drop(archive);
    let start = Instant::now();
    let archive = Archive::open_with_key(&path, &key).unwrap();
    assert_eq!(archive.chats().unwrap().len(), 100);
    let reopen_ms = start.elapsed().as_secs_f64() * 1000.0;
    let database_bytes = std::fs::metadata(&path).unwrap().len();
    eprintln!(
        "SYNTHETIC_ARCHIVE_MEASUREMENT={}",
        serde_json::json!({
            "chats":100,"messages":10000,"raw_bytes_per_message":512,"new_ms":new_ms,"ingest_ms":ingest_ms,
            "pages_100_ms":pages_ms,"search_100_ms":search_ms,"reopen_and_chats_ms":reopen_ms,"database_bytes":database_bytes,
            "profile": if cfg!(debug_assertions) {"debug"} else {"release"},
            "platform":std::env::consts::OS,
        })
    );
}

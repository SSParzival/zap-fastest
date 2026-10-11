//! Bounded archive reads and idempotent text sends. No raw protocol/key/media export.
use super::*;
use crate::automation::{Pending, Request, error};
impl Worker {
    pub(super) fn automation(&mut self, pending: Pending) {
        if !pending.request.valid()
            || pending.request.account() != self.dirs.id.as_str()
            || pending.blocked()
        {
            pending.answer(error("Application locked or request expired"));
            return;
        }
        let chat = pending.request.chat().to_owned();
        let known = match self.archive.chat(&chat) {
            Ok(Some(known)) => known,
            _ => {
                pending.answer(error("Unknown conversation"));
                return;
            }
        };
        if known.locked || known.disappearing_timer().is_some() {
            pending.answer(error(
                "Locked or disappearing conversations are unavailable to automation",
            ));
            return;
        }
        let reply = match &pending.request {
            Request::Read { before, limit, .. } => self
                .archive
                .messages(
                    &chat,
                    before.as_ref().map(|(time, id)| (*time, id.as_str())),
                    *limit,
                )
                .map(|rows| self.cli_rows(&chat, rows))
                .unwrap_or_else(|_| error("Could not read archive")),
            Request::Watch { after, limit, .. } => {
                // Polls archived arrival order, including late history. The cursor is archive rowid,
                // carried as the first tuple value; the second is the message id for validation.
                self.archive.cli_watch(&chat,after.as_ref(),*limit)
                    .map(|(rows,cursor)|serde_json::json!({"page":serde_json::from_str::<serde_json::Value>(&self.cli_rows(&chat,rows)).unwrap_or_default(),"cursor":cursor}).to_string())
                    .unwrap_or_else(|_|error("Invalid cursor or archive read failed"))
            }
            Request::Send {
                request_id, text, ..
            } => {
                if self.archive_only || !self.link_up() || !self.destination_writable(&chat) {
                    error("Account offline or conversation read-only")
                } else if let Ok(Some(id)) =
                    self.archive
                        .cli_existing_send(request_id, &chat, Some(text))
                {
                    self.cli_status(&chat, request_id, &id)
                } else if self
                    .archive
                    .cli_existing_send(request_id, &chat, Some(text))
                    .is_err()
                {
                    error("Request id already belongs to different content")
                } else if pending.blocked() {
                    error("Application locked or request expired")
                } else if let Some(Some(id)) = pending.permit.execute(|| {
                    if pending.expired() {
                        return None;
                    }
                    self.send_text_inner(
                        chat.clone(),
                        text.clone(),
                        None,
                        Vec::new(),
                        Some(request_id.clone()),
                    )
                }) {
                    self.cli_status(&chat, request_id, &id)
                } else {
                    error("Send was not confirmed. Query request_id; never repeat with a new id")
                }
            }
            Request::Status { request_id, .. } => {
                match self.archive.cli_existing_send(request_id, &chat, None) {
                    Ok(Some(id)) => self.cli_status(&chat, request_id, &id),
                    Ok(None) => serde_json::json!({"request_id":request_id,"status":"not_found"})
                        .to_string(),
                    Err(_) => error("Request id belongs to another conversation"),
                }
            }
        };
        pending.answer(if reply.len() > 12 * 1024 {
            error("Response too large; use a smaller page")
        } else {
            reply
        });
    }
    fn cli_rows(&self, chat: &str, rows: Vec<Message>) -> String {
        let before = rows.first().map(|row| (row.timestamp, row.id.clone()));
        let rows:Vec<_>=rows.into_iter().filter(|row| !matches!(row.content,Content::PhoneOnly{..}) && self.archive.cli_exportable(chat,&row.id).unwrap_or(false))
            .map(|row|serde_json::json!({"id":row.id,"timestamp":row.timestamp,"from_me":row.from_me,"text":row.content.full_summary(),"status":row.status})).collect();
        serde_json::json!({"messages":rows,"next_before":before}).to_string()
    }
    fn cli_status(&self, chat: &str, request: &str, id: &str) -> String {
        let status = self
            .archive
            .message(chat, id)
            .ok()
            .flatten()
            .map(|row| row.status)
            .unwrap_or(Delivery::Unconfirmed);
        serde_json::json!({"request_id":request,"message_id":id,"status":status}).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(request: Request) -> (Pending, std::sync::mpsc::Receiver<String>) {
        let (reply, received) = std::sync::mpsc::channel();
        (
            Pending {
                request,
                permit: std::sync::Arc::new(crate::automation::Permit::allowed()),
                reply,
                cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                locked: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            },
            received,
        )
    }
    #[tokio::test]
    async fn sdk_text_send_is_queued_once_and_uncertain_completion_never_retries() {
        let directory = tempfile::tempdir().unwrap();
        let store = whatsapp_rust::store::SqliteStore::open(
            &directory
                .path()
                .join("synthetic-session.db")
                .to_string_lossy(),
        )
        .await
        .unwrap();
        let bot = Bot::builder().with_backend(store).build().await.unwrap();
        let (mut worker, _events, _commands, _wa) =
            super::super::offline_worker("9@s.whatsapp.net");
        worker.client = Some(bot.client());
        // The official SDK is built but never connected. This existing queue hook intercepts all transport starts.
        worker.outgoing.synthetic = Some(Vec::new());
        let chat = "1@s.whatsapp.net";
        worker
            .archive
            .ensure_chat(chat, "Synthetic contact")
            .unwrap();
        let token = "0123456789abcdef0123456789abcdef";
        let original = Request::Send {
            account: "1".into(),
            chat: chat.into(),
            request_id: token.into(),
            text: "Synthetic CLI send".into(),
        };
        let (pending, received) = request(original.clone());
        worker.handle_command(Command::Automation(pending)).await;
        let reply: serde_json::Value = serde_json::from_str(&received.recv().unwrap()).unwrap();
        let id = reply["message_id"].as_str().unwrap().to_owned();
        assert_eq!(worker.outgoing.synthetic.as_ref().unwrap().len(), 1);
        let row = worker.archive.message(chat, &id).unwrap().unwrap();
        assert_eq!(row.content, Content::text("Synthetic CLI send"));
        assert_eq!(
            worker
                .archive
                .cli_existing_send(token, chat, Some("Synthetic CLI send"))
                .unwrap(),
            Some(id.clone())
        );
        let (pending, received) = request(original.clone());
        worker.handle_command(Command::Automation(pending)).await;
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&received.recv().unwrap()).unwrap()["message_id"],
            id
        );
        assert_eq!(worker.outgoing.synthetic.as_ref().unwrap().len(), 1);
        worker
            .handle_command(Command::OutgoingFinished {
                chat: chat.into(),
                id: id.clone(),
                result: Err(crate::backend::SendFailure::Failed {
                    kind: "iq timeout",
                    code: None,
                }),
            })
            .await;
        assert_eq!(
            worker.archive.message(chat, &id).unwrap().unwrap().status,
            Delivery::Unconfirmed
        );
        let (pending, received) = request(original);
        worker.handle_command(Command::Automation(pending)).await;
        assert!(received.recv().unwrap().contains("unconfirmed"));
        assert_eq!(worker.outgoing.synthetic.as_ref().unwrap().len(), 1);
        let altered = Request::Send {
            account: "1".into(),
            chat: chat.into(),
            request_id: token.into(),
            text: "Altered synthetic content".into(),
        };
        let (pending, received) = request(altered);
        worker.handle_command(Command::Automation(pending)).await;
        assert!(received.recv().unwrap().contains("error"));
        assert_eq!(worker.outgoing.synthetic.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn misrouted_account_and_revoked_read_are_refused_without_content() {
        let (mut worker, _events, _commands, _wa) =
            super::super::offline_worker("9@s.whatsapp.net");
        let chat = "1@s.whatsapp.net";
        worker.archive.ensure_chat(chat, "Synthetic").unwrap();
        worker
            .archive
            .insert_message(
                &crate::archive::tests::message(chat, "private-visible", 10, false),
                None,
            )
            .unwrap();
        let (pending, received) = request(Request::Read {
            account: "2".into(),
            chat: chat.into(),
            before: None,
            limit: 1,
        });
        worker.automation(pending);
        assert!(!received.recv().unwrap().contains("private-visible"));
        let (pending, received) = request(Request::Read {
            account: "1".into(),
            chat: chat.into(),
            before: None,
            limit: 1,
        });
        pending.permit.revoke();
        worker.automation(pending);
        assert!(!received.recv().unwrap().contains("private-visible"));
    }

    #[test]
    fn worker_read_and_watch_preserve_lock_expiry_and_phone_only_privacy() {
        let (mut worker, _events, _commands, _wa) =
            super::super::offline_worker("fixture@s.whatsapp.net");
        let chat = "1@s.whatsapp.net";
        worker.archive.ensure_chat(chat, "Synthetic chat").unwrap();
        let row = crate::archive::tests::message(chat, "visible", 10, false);
        worker.archive.insert_message(&row, None).unwrap();
        let mut private = row.clone();
        private.id = "disappearing".into();
        private.content = Content::text("synthetic disappearing secret");
        worker.archive.insert_message(&private, None).unwrap();
        worker
            .archive
            .remember_message_expiry(chat, &private.id, 999999)
            .unwrap();
        let (pending, received) = request(Request::Read {
            account: "1".into(),
            chat: chat.into(),
            before: None,
            limit: 10,
        });
        worker.automation(pending);
        let reply = received.recv().unwrap();
        assert!(reply.contains("visible"));
        assert!(!reply.contains("synthetic disappearing secret"));
        worker.archive.set_locked(chat, true).unwrap();
        let (pending, received) = request(Request::Watch {
            account: "1".into(),
            chat: chat.into(),
            after: None,
            limit: 10,
        });
        worker.automation(pending);
        let reply = received.recv().unwrap();
        assert!(reply.contains("error"));
        assert!(!reply.contains("visible"));
    }
}

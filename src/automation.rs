//! Explicit local automation over a separate authenticated fastframe-instance slot.
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum Request {
    Read {
        account: String,
        chat: String,
        before: Option<(i64, String)>,
        limit: usize,
    },
    Watch {
        account: String,
        chat: String,
        after: Option<(i64, String)>,
        limit: usize,
    },
    Send {
        account: String,
        chat: String,
        request_id: String,
        text: String,
    },
    Status {
        account: String,
        chat: String,
        request_id: String,
    },
}
impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AutomationRequest([private])")
    }
}
impl Request {
    pub fn account(&self) -> &str {
        match self {
            Self::Read { account, .. }
            | Self::Watch { account, .. }
            | Self::Send { account, .. }
            | Self::Status { account, .. } => account,
        }
    }
    pub fn chat(&self) -> &str {
        match self {
            Self::Read { chat, .. }
            | Self::Watch { chat, .. }
            | Self::Send { chat, .. }
            | Self::Status { chat, .. } => chat,
        }
    }
    pub fn valid(&self) -> bool {
        if !crate::model::AccountId::is_safe(self.account())
            || self.chat().len() > 256
            || self.chat().contains(['\n', '\r'])
        {
            return false;
        }
        match self {
            Self::Read { limit, .. } | Self::Watch { limit, .. } => (1..=25).contains(limit),
            Self::Send {
                request_id, text, ..
            } => valid_id(request_id) && !text.trim().is_empty() && text.len() <= 4000,
            Self::Status { request_id, .. } => valid_id(request_id),
        }
    }
}
pub fn valid_id(id: &str) -> bool {
    id.len() == 32
        && id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn error(message: &str) -> String {
    serde_json::json!({"error":message}).to_string()
}

/// A request may be authorized once; revocation is permanent even after unlocking.
#[derive(Debug, Default)]
pub struct Permit(Mutex<u8>);
impl Permit {
    fn authorize(&self) -> bool {
        let mut state = self.0.lock().unwrap_or_else(|p| p.into_inner());
        if *state != 0 {
            return false;
        }
        *state = 1;
        true
    }
    pub fn revoke(&self) {
        *self.0.lock().unwrap_or_else(|p| p.into_inner()) = 2;
    }
    pub fn blocked(&self) -> bool {
        *self.0.lock().unwrap_or_else(|p| p.into_inner()) != 1
    }
    /// Enqueue and revocation share one fence: a send is accepted before revocation or refused.
    pub fn execute<T>(&self, action: impl FnOnce() -> T) -> Option<T> {
        let state = self.0.lock().unwrap_or_else(|p| p.into_inner());
        if *state != 1 {
            return None;
        }
        Some(action())
    }
    #[cfg(test)]
    pub fn allowed() -> Self {
        Self(Mutex::new(1))
    }
}
type TrackedPermit = (String, String, Weak<Permit>);
#[derive(Default)]
pub struct Permits(Mutex<Vec<TrackedPermit>>);
impl Permits {
    pub fn authorize(&self, pending: &Pending) -> bool {
        let mut requests = self.0.lock().unwrap_or_else(|p| p.into_inner());
        if pending.expired() || !pending.permit.authorize() {
            return false;
        }
        requests.push((
            pending.request.account().into(),
            pending.request.chat().into(),
            Arc::downgrade(&pending.permit),
        ));
        true
    }
    pub fn sync(&self, allowed: impl Fn(&str, &str) -> bool) {
        self.0
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .retain(|(account, chat, weak)| {
                let Some(permit) = weak.upgrade() else {
                    return false;
                };
                if !allowed(account, chat) {
                    permit.revoke();
                }
                !permit.blocked()
            });
    }
    pub fn revoke_all(&self) {
        self.sync(|_, _| false);
    }
}

#[derive(Debug)]
pub struct Pending {
    pub request: Request,
    pub permit: Arc<Permit>,
    pub reply: mpsc::Sender<String>,
    pub cancelled: Arc<AtomicBool>,
    pub locked: Arc<AtomicBool>,
}
impl Pending {
    pub fn blocked(&self) -> bool {
        self.expired() || self.permit.blocked()
    }
    pub fn expired(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst) || self.locked.load(Ordering::SeqCst)
    }
    pub fn answer(&self, reply: String) {
        let _ = self.reply.send(if self.blocked() {
            error("Request expired or application locked")
        } else {
            reply
        });
    }
}
pub type Queue = Arc<Mutex<Vec<Pending>>>;
pub struct Server {
    pub queue: Queue,
    pub locked: Arc<AtomicBool>,
    pub permits: Permits,
    _guard: fastframe_instance::Guard,
}
fn slot(runtime: &Path) -> fastframe_instance::Slot {
    fastframe_instance::Slot::at(runtime.join("automation-v1"), "zap-faster-automation-v1")
}
impl Server {
    pub fn start(runtime: &Path, waker: crate::backend::Waker) -> Result<Self, String> {
        let queue = Queue::default();
        let locked = Arc::new(AtomicBool::new(true));
        let pending = queue.clone();
        let privacy = locked.clone();
        let claim=slot(runtime).claim("probe",move|wire|{
            if wire=="probe" { return Some(error("Automation requires account opt-in")); }
            if privacy.load(Ordering::SeqCst) { return Some(error("Application locked")); }
            let Ok(request)=serde_json::from_str::<Request>(wire) else {return Some(error("Invalid request"));};
            if !request.valid(){return Some(error("Invalid request"));}
            let (reply,received)=mpsc::channel(); let cancelled=Arc::new(AtomicBool::new(false));
            let permit=Arc::new(Permit::default());
            let item=Pending{request,permit:permit.clone(),reply,cancelled:cancelled.clone(),locked:privacy.clone()};
            let mut requests=pending.lock().unwrap_or_else(|p|p.into_inner());
            if requests.len()>=32 { return Some(error("Request queue is full")); }
            requests.push(item); drop(requests); waker.wake();
            let answer=received.recv_timeout(Duration::from_secs(3));
            let authorized=!privacy.load(Ordering::SeqCst) && !permit.blocked();
            cancelled.store(true,Ordering::SeqCst);
            if answer.is_ok() && !authorized { return Some(error("Request authorization was revoked")); }
            Some(answer.unwrap_or_else(|_|error("Request timed out; a send may be Unconfirmed. Query its request_id, never resubmit with a new id")))
        });
        match claim {
            fastframe_instance::Claim::First(guard) => Ok(Self {
                queue,
                locked,
                permits: Permits::default(),
                _guard: guard,
            }),
            _ => Err("Automation slot is already occupied".into()),
        }
    }
}
pub fn send(runtime: &Path, request: &Request) -> std::io::Result<String> {
    if !request.valid() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid automation request",
        ));
    }
    slot(runtime).send(&serde_json::to_string(request).map_err(std::io::Error::other)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opt_out_and_chat_lock_revoke_delayed_replies_permanently() {
        for cause in ["account opt-out", "chat lock", "account removal"] {
            let registry = Permits::default();
            let (reply, received) = mpsc::channel();
            let pending = Pending {
                request: Request::Read {
                    account: "2".into(),
                    chat: "fixture".into(),
                    before: None,
                    limit: 1,
                },
                permit: Arc::new(Permit::default()),
                reply,
                cancelled: Arc::new(AtomicBool::new(false)),
                locked: Arc::new(AtomicBool::new(false)),
            };
            assert!(registry.authorize(&pending));
            assert!(!pending.blocked());
            registry.sync(|_, _| false);
            registry.sync(|_, _| true);
            assert!(pending.blocked(), "{cause} cannot revive after re-enable");
            assert!(!registry.authorize(&pending));
            assert!(
                pending
                    .permit
                    .execute(|| panic!("revoked send must never enqueue"))
                    .is_none()
            );
            pending.answer("synthetic private body".into());
            assert!(!received.recv().unwrap().contains("synthetic private body"));
        }
    }

    #[test]
    fn app_lock_and_unlock_do_not_revive_previous_request_and_final_handler_scrubs_buffered_reply()
    {
        let registry = Permits::default();
        let (reply, received) = mpsc::channel();
        let pending = Pending {
            request: Request::Read {
                account: "2".into(),
                chat: "fixture".into(),
                before: None,
                limit: 1,
            },
            permit: Arc::new(Permit::default()),
            reply,
            cancelled: Arc::new(AtomicBool::new(false)),
            locked: Arc::new(AtomicBool::new(false)),
        };
        assert!(registry.authorize(&pending));
        pending.answer("synthetic private body".into());
        registry.revoke_all();
        pending.locked.store(true, Ordering::SeqCst);
        pending.locked.store(false, Ordering::SeqCst);
        // Even a reply buffered before the lock is withheld by the transport's final authorization check.
        let buffered = received.recv().unwrap();
        let output = if pending.blocked() {
            error("Request authorization was revoked")
        } else {
            buffered
        };
        assert!(!output.contains("synthetic private body"));
    }

    #[test]
    fn requests_are_bounded_and_debug_never_contains_private_body() {
        let request = Request::Send {
            account: "2".into(),
            chat: "fixture".into(),
            request_id: "0123456789abcdef0123456789abcdef".into(),
            text: "synthetic private body".into(),
        };
        assert!(request.valid());
        assert!(!format!("{request:?}").contains("synthetic private body"));
        let oversized = Request::Read {
            account: "2".into(),
            chat: "fixture".into(),
            before: None,
            limit: 26,
        };
        assert!(!oversized.valid());
        assert!(!valid_id("arbitrary"));
    }
    #[test]
    fn cancellation_or_lock_scrubs_reply() {
        let (reply, received) = mpsc::channel();
        let pending = Pending {
            permit: Arc::new(Permit::allowed()),
            request: Request::Status {
                account: "2".into(),
                chat: "fixture".into(),
                request_id: "0123456789abcdef0123456789abcdef".into(),
            },
            reply,
            cancelled: Arc::new(AtomicBool::new(false)),
            locked: Arc::new(AtomicBool::new(true)),
        };
        pending.answer("synthetic private body".into());
        assert!(!received.recv().unwrap().contains("synthetic private body"));
    }
}

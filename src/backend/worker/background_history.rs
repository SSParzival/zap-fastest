//! Bounded account-local history recovery and ownership of unsolicited pages.
//!
//! Match the phone's optional PDO session id to the library's returned id.
//! Keep a conservative chat owner for responses that omit that id.

use crate::model::ChatId;
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

const PACE: Duration = Duration::from_secs(20);
const MAX_PAGES: u8 = 5;
const MAX_REQUESTS: u8 = 50;

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Checkpoint {
    cursor: usize,
    day: i64,
    requests: u32,
    next_request: i64,
    retries: HashMap<ChatId, (u8, i64)>,
}

#[derive(Default)]
pub(super) struct Recovery {
    pub response: crate::model::HistoryResponseState,
    pub full: bool,
    pub daily_budget: u32,
    checkpoint: Checkpoint,
    pub enabled: bool,
    pub paused: bool,
    pub focused: Option<ChatId>,
    due: Option<Instant>,
    pages: HashMap<ChatId, u8>,
    requests: u8,
    cursor: usize,
    /// Automatic requests, including timed-out ones still owned by the phone.
    silent: HashSet<ChatId>,
    stopped: HashSet<ChatId>,
    polls: HashSet<ChatId>,
    started: HashMap<ChatId, Instant>,
    sessions: HashMap<String, (ChatId, Instant)>,
    early: HashMap<String, Vec<(ChatId, usize, Option<bool>)>>,
}

impl Recovery {
    pub fn restore(&mut self, value: &str) -> Result<(), serde_json::Error> {
        self.checkpoint = serde_json::from_str(value)?;
        self.cursor = self.checkpoint.cursor;
        Ok(())
    }
    pub fn checkpoint(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.checkpoint)
    }
    pub fn canonicalize(&mut self, old: &str, new: &str) {
        if let Some(retry) = self.checkpoint.retries.remove(old) {
            let target = self.checkpoint.retries.entry(new.to_owned()).or_default();
            target.0 = target.0.max(retry.0);
            target.1 = target.1.max(retry.1);
        }
        if let Some(pages) = self.pages.remove(old) {
            let total = self.pages.entry(new.to_owned()).or_default();
            *total = total.saturating_add(pages);
        }
        if self.silent.remove(old) {
            self.silent.insert(new.to_owned());
        }
        if self.polls.remove(old) {
            self.polls.insert(new.to_owned());
        }
        if self.stopped.remove(old) {
            self.stopped.insert(new.to_owned());
        }
        if let Some(started) = self.started.remove(old) {
            let target = self.started.entry(new.to_owned()).or_insert(started);
            *target = (*target).max(started);
        }
        for (chat, _) in self.sessions.values_mut() {
            if chat == old {
                *chat = new.to_owned();
            }
        }
        for filed in self.early.values_mut() {
            for (chat, _, _) in filed {
                if chat == old {
                    *chat = new.to_owned();
                }
            }
        }
        if self.focused.as_deref() == Some(old) {
            self.focused = Some(new.to_owned());
        }
    }
    pub fn started(&mut self, chat: &str, requested: Instant) {
        self.response = crate::model::HistoryResponseState::Waiting;
        self.started.insert(chat.to_owned(), requested);
    }
    pub fn session(&self, id: &str) -> Option<&(ChatId, Instant)> {
        self.sessions.get(id)
    }
    pub fn sent(
        &mut self,
        chat: ChatId,
        requested: Instant,
        id: String,
    ) -> Option<Vec<(ChatId, usize, Option<bool>)>> {
        // Dropping old identities is safe: unrecognized pages never own UI.
        if self.sessions.len() >= 128
            && let Some(oldest) = self
                .sessions
                .iter()
                .min_by_key(|(_, (_, at))| *at)
                .map(|(id, _)| id.clone())
        {
            self.sessions.remove(&oldest);
        }
        let early = self.early.remove(&id);
        self.sessions.insert(id, (chat, requested));
        early
    }
    pub fn early(&mut self, id: String, filed: Vec<(ChatId, usize, Option<bool>)>) {
        if self.early.len() < 8 {
            self.early.insert(id, filed);
        }
    }
    pub fn owns_attempt(&self, chat: &str, requested: Instant) -> bool {
        self.started.get(chat) == Some(&requested)
    }
    pub fn attempted(&self, chat: &str) -> bool {
        self.started.contains_key(chat)
    }
    pub fn correlated(&self, chat: &str) -> bool {
        self.started.get(chat).is_some_and(|at| {
            self.sessions
                .values()
                .any(|(owner, started)| owner == chat && started == at)
        })
    }
    pub fn owns(&self, chat: &str) -> bool {
        self.silent.contains(chat)
    }
    pub fn claim(&mut self, chat: ChatId) {
        self.silent.insert(chat);
    }
    pub fn claim_poll(&mut self, chat: ChatId) {
        self.polls.insert(chat.clone());
        self.claim(chat);
    }
    pub fn poll_owned(&self, chat: &str) -> bool {
        self.polls.contains(chat)
    }
    pub fn answered(&mut self, chat: &str, progress: bool) -> bool {
        self.response = crate::model::HistoryResponseState::Answered;
        self.polls.remove(chat);
        let owned = self.silent.remove(chat);
        if owned && !progress {
            self.stop(chat);
        } else if owned {
            self.checkpoint.retries.remove(chat);
        }
        owned
    }
    pub fn stop(&mut self, chat: &str) {
        if self.started.contains_key(chat) {
            self.response = crate::model::HistoryResponseState::NoResponse;
        }
        if self.full {
            let retry = self.checkpoint.retries.entry(chat.to_owned()).or_default();
            retry.0 = retry.0.saturating_add(1).min(10);
            retry.1 = crate::util::now() + (60_i64 * (1 << retry.0)).min(3600);
            // An exact old session identity still protects late responses.
            // Untagged sends retain their owner until reconnect.
            if self.correlated(chat) {
                self.silent.remove(chat);
            }
        } else {
            self.stopped.insert(chat.to_owned());
        }
    }
    /// Reconnect releases uncorrelated leases but never renews session budgets.
    pub fn reconnect(&mut self) {
        self.response = crate::model::HistoryResponseState::Unknown;
        self.silent.clear();
        self.polls.clear();
        self.started.clear();
        self.early.clear();
    }
    pub fn next(&mut self, candidates: &[ChatId], now: Instant) -> Option<ChatId> {
        let timestamp = crate::util::now();
        let day = timestamp.div_euclid(86_400);
        if day != self.checkpoint.day {
            self.checkpoint.day = day;
            self.checkpoint.requests = 0;
        }
        if !self.enabled
            || self.paused
            || (!self.full && self.requests >= MAX_REQUESTS)
            || (self.full
                && (self.checkpoint.requests >= self.daily_budget.max(1)
                    || timestamp < self.checkpoint.next_request))
            || self.due.is_some_and(|due| now < due)
        {
            return None;
        }
        let len = candidates.len();
        for step in 0..len {
            let index = (self.cursor + step) % len;
            let chat = &candidates[index];
            if self.owns(chat)
                || self.stopped.contains(chat)
                || (self.full
                    && self
                        .checkpoint
                        .retries
                        .get(chat)
                        .is_some_and(|(_, due)| timestamp < *due))
                || (!self.full && self.pages.get(chat).copied().unwrap_or_default() >= MAX_PAGES)
            {
                continue;
            }
            self.cursor = (index + 1) % len;
            let pages = self.pages.entry(chat.clone()).or_default();
            *pages = pages.saturating_add(1);
            self.requests = self.requests.saturating_add(1);
            self.checkpoint.cursor = self.cursor;
            self.checkpoint.requests = self.checkpoint.requests.saturating_add(1);
            self.checkpoint.next_request = timestamp + PACE.as_secs() as i64;
            self.due = Some(now + PACE);
            self.claim(chat.clone());
            return Some(chat.clone());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn history_response_reports_observations_without_inferring_phone_availability() {
        use crate::model::HistoryResponseState as State;
        let mut recovery = Recovery::default();
        assert_eq!(recovery.response, State::Unknown);
        recovery.stop("unsent");
        assert_eq!(recovery.response, State::Unknown);
        recovery.started("fixture", Instant::now());
        assert_eq!(recovery.response, State::Waiting);
        recovery.stop("fixture");
        assert_eq!(recovery.response, State::NoResponse);
        recovery.answered("fixture", false);
        assert_eq!(recovery.response, State::Answered);
        recovery.reconnect();
        assert_eq!(recovery.response, State::Unknown);
    }

    #[test]
    fn canonical_identity_merge_keeps_the_stronger_persisted_backoff() {
        let mut recovery = Recovery::default();
        recovery
            .checkpoint
            .retries
            .insert("old@lid".into(), (4, 900));
        recovery
            .checkpoint
            .retries
            .insert("new@s.whatsapp.net".into(), (1, 100));
        recovery.canonicalize("old@lid", "new@s.whatsapp.net");
        assert_eq!(
            recovery.checkpoint.retries.get("new@s.whatsapp.net"),
            Some(&(4, 900))
        );
        assert!(!recovery.checkpoint.retries.contains_key("old@lid"));
        let saved = recovery.checkpoint().unwrap();
        let mut restarted = Recovery::default();
        restarted.restore(&saved).unwrap();
        assert_eq!(
            restarted.checkpoint.retries.get("new@s.whatsapp.net"),
            Some(&(4, 900))
        );
    }
    #[test]
    fn full_mode_rotates_beyond_ten_chats_and_preserves_daily_budget() {
        let mut recovery = Recovery {
            enabled: true,
            full: true,
            daily_budget: 60,
            ..Default::default()
        };
        let candidates: Vec<_> = (0..12).map(|index| format!("fixture-{index}")).collect();
        let now = Instant::now();
        for attempt in 0..60 {
            // Advance the synthetic wall-clock gate without a real sleep.
            recovery.checkpoint.next_request = 0;
            let chat = recovery.next(&candidates, now + PACE * attempt).unwrap();
            assert_eq!(chat, candidates[attempt as usize % candidates.len()]);
            recovery.answered(&chat, true);
        }
        let saved = recovery.checkpoint().unwrap();
        let mut restarted = Recovery {
            enabled: true,
            full: true,
            daily_budget: 60,
            ..Default::default()
        };
        restarted.restore(&saved).unwrap();
        assert!(restarted.next(&candidates, now + PACE * 61).is_none());
    }

    #[test]
    fn full_mode_no_progress_backoff_and_pacing_survive_restart() {
        let mut recovery = Recovery {
            enabled: true,
            full: true,
            daily_budget: 100,
            ..Default::default()
        };
        let candidates = vec!["fixture-chat".into()];
        let now = Instant::now();
        let chat = recovery.next(&candidates, now).unwrap();
        recovery.answered(&chat, false);
        let saved = recovery.checkpoint().unwrap();
        let mut restarted = Recovery {
            enabled: true,
            full: true,
            daily_budget: 100,
            ..Default::default()
        };
        restarted.restore(&saved).unwrap();
        restarted.checkpoint.next_request = 0;
        assert!(restarted.next(&candidates, now + PACE * 10).is_none());
        restarted.checkpoint.retries.get_mut(&chat).unwrap().1 = 0;
        assert_eq!(
            restarted.next(&candidates, now + PACE * 11).as_deref(),
            Some(chat.as_str())
        );
    }

    #[test]
    fn late_privacy_mapping_preserves_budget_and_correlated_ownership() {
        let now = Instant::now();
        let mut recovery = Recovery {
            enabled: true,
            ..Default::default()
        };
        assert_eq!(
            recovery.next(&["9@lid".into()], now).as_deref(),
            Some("9@lid")
        );
        recovery.started("9@lid", now);
        recovery.sent("9@lid".into(), now, "session".into());
        recovery.canonicalize("9@lid", "1@s.whatsapp.net");
        assert_eq!(
            recovery.session("session"),
            Some(&("1@s.whatsapp.net".into(), now))
        );
        assert!(recovery.owns("1@s.whatsapp.net"));
        assert!(!recovery.owns("9@lid"));
        assert!(recovery.answered("1@s.whatsapp.net", true));
        for attempt in 1..5 {
            let chat = recovery
                .next(&["1@s.whatsapp.net".into()], now + PACE * attempt)
                .unwrap();
            recovery.answered(&chat, true);
        }
        assert!(
            recovery
                .next(&["1@s.whatsapp.net".into()], now + PACE * 5)
                .is_none()
        );
    }
    #[test]
    fn changing_chat_candidates_cannot_exceed_the_account_session_budget() {
        let mut recovery = Recovery {
            enabled: true,
            ..Default::default()
        };
        let now = Instant::now();
        for attempt in 0..50 {
            let candidate = vec![format!("chat-{attempt}")];
            let chat = recovery.next(&candidate, now + PACE * attempt).unwrap();
            recovery.answered(&chat, true);
        }
        recovery.reconnect();
        assert!(
            recovery
                .next(&["another".into()], now + PACE * 51)
                .is_none()
        );
    }
    #[test]
    fn early_response_registration_is_bounded_and_correlated_to_its_attempt() {
        let now = Instant::now();
        let mut recovery = Recovery::default();
        recovery.early("early".into(), vec![("chat".into(), 1, Some(true))]);
        assert_eq!(
            recovery.sent("chat".into(), now, "early".into()),
            Some(vec![("chat".into(), 1, Some(true))])
        );
        recovery.claim("chat".into());
        recovery.started("chat", now);
        assert!(recovery.correlated("chat"));
        assert!(recovery.owns_attempt("chat", now));
        assert!(!recovery.owns_attempt("chat", now + PACE));
        recovery.reconnect();
        assert!(!recovery.owns_attempt("chat", now));
        assert_eq!(recovery.session("early"), Some(&("chat".into(), now)));
        for index in 0..20 {
            recovery.early(index.to_string(), Vec::new());
        }
        assert_eq!(recovery.early.len(), 8);
    }
    #[test]
    fn opt_in_pacing_fairness_and_budget_survive_reconnect_and_cancellation() {
        let now = Instant::now();
        let chats = vec!["a".into(), "b".into()];
        let mut recovery = Recovery::default();
        assert!(recovery.next(&chats, now).is_none());
        recovery.enabled = true;
        assert_eq!(recovery.next(&chats, now).as_deref(), Some("a"));
        recovery.answered("a", true);
        assert!(
            recovery
                .next(&chats, now + Duration::from_secs(19))
                .is_none()
        );
        assert_eq!(recovery.next(&chats, now + PACE).as_deref(), Some("b"));
        recovery.enabled = false;
        assert!(
            recovery.owns("b"),
            "cancel keeps ownership of transmitted requests"
        );
        assert!(recovery.next(&chats, now + PACE * 2).is_none());
        recovery.enabled = true;
        recovery.reconnect();
        for attempt in 2..10 {
            let chat = recovery.next(&chats, now + PACE * attempt).unwrap();
            recovery.answered(&chat, true);
        }
        assert!(recovery.next(&chats, now + PACE * 10).is_none());
        recovery.reconnect();
        assert!(recovery.next(&chats, now + PACE * 11).is_none());
    }
    #[test]
    fn timeout_late_response_no_progress_lock_pause_and_account_isolation() {
        let now = Instant::now();
        let chats = vec!["a".into()];
        let mut first = Recovery {
            enabled: true,
            ..Default::default()
        };
        let mut second = Recovery {
            enabled: true,
            ..Default::default()
        };
        first.paused = true;
        assert!(first.next(&chats, now).is_none());
        first.paused = false;
        assert!(first.next(&chats, now).is_some());
        assert!(first.next(&chats, now + PACE * 10).is_none());
        assert!(second.next(&chats, now).is_some());
        assert!(first.answered("a", false));
        assert!(
            !first.answered("a", true),
            "stale duplicate cannot release another owner"
        );
        assert!(second.owns("a"));
        assert!(first.next(&chats, now + PACE * 11).is_none());
        assert!(
            !Recovery::default().enabled,
            "restart retains opt-in policy via account settings"
        );
    }
}

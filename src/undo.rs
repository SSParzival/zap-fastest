//! Short local grace periods. A cancelled or interrupted request never reaches WhatsApp.

use std::time::{Duration, Instant};

use crate::model::{AccountId, ChatId};

pub const GRACE: Duration = Duration::from_secs(5);

#[derive(Clone, PartialEq, Eq)]
pub enum Operation {
    DeleteMessage {
        chat: ChatId,
        id: String,
    },
    ArchiveChat {
        chat: ChatId,
        before: bool,
        after: bool,
    },
}

impl Operation {
    fn same_target(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DeleteMessage { chat: a, id: x }, Self::DeleteMessage { chat: b, id: y }) => {
                a == b && x == y
            }
            (Self::ArchiveChat { chat: a, .. }, Self::ArchiveChat { chat: b, .. }) => a == b,
            _ => false,
        }
    }
}

pub struct Pending {
    pub token: u64,
    pub account: AccountId,
    pub operation: Operation,
    deadline: Instant,
}

#[derive(Default)]
pub struct Queue {
    sequence: u64,
    pending: Vec<Pending>,
}

impl Queue {
    pub fn schedule(
        &mut self,
        account: AccountId,
        mut operation: Operation,
        now: Instant,
    ) -> (u64, Option<u64>) {
        let previous = self
            .pending
            .iter()
            .position(|pending| {
                pending.account == account && pending.operation.same_target(&operation)
            })
            .map(|index| self.pending.remove(index));
        if let Some(Pending {
            operation: Operation::ArchiveChat { before, .. },
            ..
        }) = &previous
            && let Operation::ArchiveChat {
                before: initial, ..
            } = &mut operation
        {
            *initial = *before;
        }
        self.sequence = self.sequence.wrapping_add(1);
        let token = self.sequence;
        self.pending.push(Pending {
            token,
            account,
            operation,
            deadline: now + GRACE,
        });
        (token, previous.map(|pending| pending.token))
    }

    pub fn cancel(&mut self, token: u64) -> Option<Pending> {
        self.pending
            .iter()
            .position(|pending| pending.token == token)
            .map(|index| self.pending.remove(index))
    }

    pub fn forget_account(&mut self, account: &AccountId) {
        self.pending.retain(|pending| &pending.account != account);
    }

    pub fn due(&mut self, now: Instant) -> Vec<Pending> {
        let mut due = Vec::new();
        self.pending.retain_mut(|pending| {
            if pending.deadline <= now {
                due.push(Pending {
                    token: pending.token,
                    account: pending.account.clone(),
                    operation: pending.operation.clone(),
                    deadline: pending.deadline,
                });
                false
            } else {
                true
            }
        });
        due
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        self.pending.iter().map(|pending| pending.deadline).min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_and_expiry_are_account_bound_and_send_each_request_once() {
        let now = Instant::now();
        let mut queue = Queue::default();
        let operation = Operation::DeleteMessage {
            chat: "same-chat".into(),
            id: "same-message".into(),
        };
        let (first, _) = queue.schedule(AccountId("1".into()), operation.clone(), now);
        queue.schedule(AccountId("2".into()), operation, now);
        assert!(queue.due(now + GRACE - Duration::from_millis(1)).is_empty());
        assert_eq!(queue.cancel(first).unwrap().account, AccountId("1".into()));
        let due = queue.due(now + GRACE);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].account, AccountId("2".into()));
        assert!(queue.due(now + GRACE).is_empty());
    }

    #[test]
    fn repeated_archive_changes_keep_the_original_state_for_undo() {
        let now = Instant::now();
        let mut queue = Queue::default();
        let account = AccountId("fixture".into());
        let (old, _) = queue.schedule(
            account.clone(),
            Operation::ArchiveChat {
                chat: "chat".into(),
                before: false,
                after: true,
            },
            now,
        );
        let (new, previous) = queue.schedule(
            account,
            Operation::ArchiveChat {
                chat: "chat".into(),
                before: true,
                after: false,
            },
            now,
        );
        assert_eq!(previous, Some(old));
        assert!(matches!(
            queue.cancel(new).unwrap().operation,
            Operation::ArchiveChat {
                before: false,
                after: false,
                ..
            }
        ));
    }
}

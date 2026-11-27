//! The app lock, like WhatsApp Web's screen lock: a password that hides the
//! whole window after a while without input, at start, or on request.
//!
//! It keeps people using this computer out of the chats. It encrypts nothing
//! beyond what the archive already is, and a forgotten password means
//! unlinking. The password is never stored: settings keep a salted PBKDF2
//! verifier (the same scheme as the locked-chats code, with more rounds), and
//! checking or making one runs on its own thread because it takes tens of
//! milliseconds.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::{Duration, Instant, SystemTime};

use crate::settings::{hex, unhex};

/// The shortest password Settings accepts, in characters.
pub const MIN_PASSWORD_CHARS: usize = 6;

/// The shortest locked-chats code accepted, in characters. Six, as a
/// password: a shorter one falls to guessing even with the wait between tries.
pub const MIN_CHAT_LOCK_CHARS: usize = MIN_PASSWORD_CHARS;

/// A wall-clock instant as Unix milliseconds, the form the saved wait takes:
/// it must survive a restart, and a monotonic instant does not.
pub fn unix_millis(at: SystemTime) -> i64 {
    at.duration_since(SystemTime::UNIX_EPOCH)
        .map(|since| since.as_millis() as i64)
        .unwrap_or(0)
}

/// Wrong tries against one local code, kept on the wall clock so the wait
/// after repeated failures survives a restart of the app.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Attempts {
    failures: u32,
    /// When the next try may be made, as Unix milliseconds.
    retry_at_ms: Option<i64>,
}

impl Attempts {
    pub fn new(failures: u32, retry_at_ms: Option<i64>) -> Self {
        Self {
            failures,
            retry_at_ms,
        }
    }

    pub fn failures(&self) -> u32 {
        self.failures
    }

    pub fn retry_at_ms(&self) -> Option<i64> {
        self.retry_at_ms
    }

    /// How long until another try may be made, if it must wait.
    pub fn wait_left(&self, now_ms: i64) -> Option<Duration> {
        let at = self.retry_at_ms?;
        let left = at - now_ms;
        (left > 0).then(|| Duration::from_millis(left as u64))
    }

    /// Records a wrong try at `now_ms`, starting a wait once too many have
    /// come in a row.
    pub fn failed(&mut self, now_ms: i64) {
        self.failures = self.failures.saturating_add(1);
        let wait = wait_after(self.failures);
        self.retry_at_ms = (!wait.is_zero()).then(|| now_ms + wait.as_millis() as i64);
    }

    /// A right try, or a new code: the count and the wait start over.
    pub fn passed(&mut self) {
        *self = Self::default();
    }
}

/// PBKDF2-HMAC-SHA256 rounds for new verifiers, OWASP's current advice for
/// this function. About 60 ms here, paid once per unlock attempt.
const ROUNDS: u32 = 600_000;

/// The scheme name leading a stored verifier, so a later scheme can be told
/// apart and the rounds can change without breaking stored ones.
const SCHEME: &str = "pbkdf2-sha256";

/// Consecutive wrong passwords allowed before each try has to wait.
const FREE_TRIES: u32 = 3;

/// The longest wait between tries after repeated wrong passwords.
const MAX_WAIT: Duration = Duration::from_secs(30);

/// How far the wall clock must run ahead of the monotonic one before the
/// difference counts as the machine having slept, rather than as the two
/// clocks drifting apart by a few microseconds of ordinary use.
const SLEEP_SLACK: Duration = Duration::from_secs(1);

/// A stored verifier for `password`: `pbkdf2-sha256$rounds$salt$hash`, with
/// salt and hash in hex. Slow on purpose; call it off the interface thread.
pub fn verifier(password: &str) -> String {
    verifier_with_rounds(password, ROUNDS)
}

fn verifier_with_rounds(password: &str, rounds: u32) -> String {
    let rounds = std::num::NonZeroU32::new(rounds.max(1)).expect("at least one round");
    let salt: [u8; 16] = ring::rand::generate(&ring::rand::SystemRandom::new())
        .expect("the system random generator is unavailable")
        .expose();
    let mut hash = [0u8; 32];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        rounds,
        &salt,
        password.as_bytes(),
        &mut hash,
    );
    format!("{SCHEME}${rounds}${}${}", hex(&salt), hex(&hash))
}

/// Whether `password` matches a stored verifier. ring compares the derived
/// hash in constant time. Slow on purpose; call it off the interface thread.
pub fn verifies(stored: &str, password: &str) -> bool {
    let mut parts = stored.split('$');
    let (Some(SCHEME), Some(rounds), Some(salt), Some(expected), None) = (
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
    ) else {
        return false;
    };
    // A hand-edited round count must not hang the checking thread for hours.
    let Some(rounds) = rounds
        .parse::<u32>()
        .ok()
        .filter(|rounds| *rounds <= 10 * ROUNDS)
        .and_then(std::num::NonZeroU32::new)
    else {
        return false;
    };
    let (Some(salt), Some(expected)) = (unhex(salt), unhex(expected)) else {
        return false;
    };
    ring::pbkdf2::verify(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        rounds,
        &salt,
        password.as_bytes(),
        &expected,
    )
    .is_ok()
}

/// How long the next try waits after `failures` wrong passwords in a row:
/// nothing for the first few, then one second doubling up to half a minute.
pub fn wait_after(failures: u32) -> Duration {
    match failures.checked_sub(FREE_TRIES) {
        None => Duration::ZERO,
        Some(extra) => Duration::from_secs(1u64 << extra.min(5)).min(MAX_WAIT),
    }
}

/// Where the time comes from; tests replace it to move time forward.
pub type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;

/// Where the wall-clock time comes from. Unlike [`Instant`], it keeps moving
/// while the machine sleeps, which is how a suspend counts as inactivity:
/// [`AppLock::idle_left`] takes whichever clock moved further.
pub type WallClock = Arc<dyn Fn() -> SystemTime + Send + Sync>;

/// What a Settings form does with the password.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormMode {
    /// Turns the app lock on with a new password.
    Set,
    /// Replaces the password; the current one is asked first.
    Change,
    /// Turns the app lock off; the current password is asked first.
    TurnOff,
}

/// Why a Settings form was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormError {
    TooShort,
    Mismatch,
    WrongCurrent,
}

/// The password form on the Settings page. Its fields belong to the view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub mode: FormMode,
    pub current: String,
    pub new: String,
    pub confirm: String,
    pub error: Option<FormError>,
    /// A check or a new verifier is being worked out.
    pub busy: bool,
}

impl Form {
    pub fn new(mode: FormMode) -> Self {
        Self {
            mode,
            current: String::new(),
            new: String::new(),
            confirm: String::new(),
            error: None,
            busy: false,
        }
    }

    /// Checks what can be checked without the stored verifier.
    pub fn check(&self) -> Result<(), FormError> {
        if self.mode == FormMode::TurnOff {
            return Ok(());
        }
        if self.new.chars().count() < MIN_PASSWORD_CHARS {
            Err(FormError::TooShort)
        } else if self.new != self.confirm {
            Err(FormError::Mismatch)
        } else {
            Ok(())
        }
    }
}

/// Where "Forgot password?" on the lock screen has got to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Forgetting {
    #[default]
    No,
    /// Asking whether to unlink.
    Confirming,
    /// Unlinking was asked for; the lock lifts once WhatsApp confirms it.
    Unlinking,
}

/// What the checking thread worked out.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The lock screen's password matched, or did not.
    Unlock(bool),
    /// The form's current password did not match.
    WrongCurrent,
    /// A new verifier for the form's new password.
    Set(String),
    /// The current password matched: turn the lock off.
    TurnOff,
}

/// Work for the checking thread. It carries passwords, so it is never logged.
enum Work {
    Unlock {
        stored: String,
        password: String,
    },
    Form {
        mode: FormMode,
        stored: Option<String>,
        current: String,
        new: String,
    },
}

fn run(work: Work) -> Outcome {
    match work {
        Work::Unlock { stored, password } => Outcome::Unlock(verifies(&stored, &password)),
        Work::Form {
            mode,
            stored,
            current,
            new,
        } => {
            // Changing or turning off needs the current password first.
            if let Some(stored) = stored
                && mode != FormMode::Set
                && !verifies(&stored, &current)
            {
                return Outcome::WrongCurrent;
            }
            match mode {
                FormMode::TurnOff => Outcome::TurnOff,
                FormMode::Set | FormMode::Change => Outcome::Set(verifier(&new)),
            }
        }
    }
}

/// The lock's state while the app runs.
pub struct AppLock {
    clock: Clock,
    /// Keeps moving while the machine sleeps, where `clock` stops.
    wall: WallClock,
    locked: bool,
    last_input: Instant,
    /// When `last_input` happened on the wall clock.
    last_input_wall: SystemTime,
    failures: u32,
    retry_at: Option<Instant>,
    job: Option<Receiver<Outcome>>,
    /// The lock screen's password field.
    pub entry: String,
    /// The last try was wrong.
    pub wrong: bool,
    pub forgetting: Forgetting,
    /// A notification clicked while locked, opened once unlocked.
    pub deferred: Option<crate::notify::NotificationTarget>,
    /// The Settings password form, when open.
    pub form: Option<Form>,
}

impl std::fmt::Debug for AppLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppLock")
            .field("locked", &self.locked)
            .field("failures", &self.failures)
            .finish_non_exhaustive()
    }
}

impl AppLock {
    /// Starts locked when a password is set, as the app does at start.
    pub fn new(locked: bool) -> Self {
        let clock: Clock = Arc::new(Instant::now);
        let wall: WallClock = Arc::new(SystemTime::now);
        Self {
            last_input: clock(),
            last_input_wall: wall(),
            clock,
            wall,
            locked,
            failures: 0,
            retry_at: None,
            job: None,
            entry: String::new(),
            wrong: false,
            forgetting: Forgetting::No,
            deferred: None,
            form: None,
        }
    }

    /// Replaces the clock, and counts inactivity from its present.
    pub fn set_clock(&mut self, clock: Clock) {
        self.last_input = clock();
        self.last_input_wall = (self.wall)();
        self.clock = clock;
    }

    /// Replaces the wall clock, and counts inactivity from its present. Tests
    /// use it to sleep the machine: the monotonic clock stays put while this
    /// one moves on.
    pub fn set_wall_clock(&mut self, wall: WallClock) {
        self.last_input_wall = wall();
        self.wall = wall;
    }

    /// Picks up the wait saved by an earlier run, so restarting the app does
    /// not offer three fresh tries.
    pub fn restore_wait(&mut self, failures: u32, retry_at: Option<SystemTime>) {
        self.failures = failures;
        let wall_now = (self.wall)();
        self.retry_at = retry_at
            .and_then(|at| at.duration_since(wall_now).ok())
            .filter(|left| !left.is_zero())
            .map(|left| self.now() + left);
    }

    /// The wait to save: how many wrong tries in a row, and when the next may
    /// be made on the wall clock.
    pub fn wait_record(&self) -> (u32, Option<SystemTime>) {
        let now = self.now();
        let retry_at = self
            .retry_at
            .map(|at| (self.wall)() + at.saturating_duration_since(now));
        (self.failures, retry_at)
    }

    pub fn now(&self) -> Instant {
        (self.clock)()
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Someone used the window: the inactivity count starts again.
    pub fn note_input(&mut self) {
        self.last_input = self.now();
        self.last_input_wall = (self.wall)();
    }

    /// How long until `after` of inactivity has passed; zero once it has.
    ///
    /// A machine that slept has been idle too, but its monotonic clock stops
    /// while it sleeps, so this takes whichever clock moved further: the wall
    /// clock catches the suspend, and the monotonic one keeps a clock set back
    /// by hand from reviving an idle countdown (or ending it early).
    pub fn idle_left(&self, after: Duration) -> Duration {
        let monotonic = self.now().saturating_duration_since(self.last_input);
        let wall = (self.wall)()
            .duration_since(self.last_input_wall)
            .unwrap_or(Duration::ZERO);
        let elapsed = if wall > monotonic + SLEEP_SLACK {
            wall
        } else {
            monotonic
        };
        after.saturating_sub(elapsed)
    }

    /// Locks, forgetting anything half typed on a previous lock screen.
    pub fn lock(&mut self) {
        self.locked = true;
        self.entry.clear();
        self.wrong = false;
        self.forgetting = Forgetting::No;
        self.form = None;
    }

    /// Lifts the lock without a password: when the password itself is gone,
    /// after unlinking or when it was turned off.
    pub fn release(&mut self) {
        self.locked = false;
        self.entry.clear();
        self.wrong = false;
        self.forgetting = Forgetting::No;
        self.failures = 0;
        self.retry_at = None;
        self.last_input = self.now();
        self.last_input_wall = (self.wall)();
    }

    /// How long until another password may be tried, if it must wait.
    pub fn wait_left(&self) -> Option<Duration> {
        let at = self.retry_at?;
        let left = at.saturating_duration_since(self.now());
        (!left.is_zero()).then_some(left)
    }

    /// Whether a check is running.
    pub fn checking(&self) -> bool {
        self.job.is_some()
    }

    /// Whether the lock screen may try its password now.
    pub fn can_try(&self) -> bool {
        self.locked && !self.checking() && self.wait_left().is_none() && !self.entry.is_empty()
    }

    /// Checks the lock screen's password against `stored` on a thread.
    pub fn try_unlock(&mut self, stored: &str, wake: impl Fn() + Send + 'static) {
        if !self.can_try() {
            return;
        }
        let password = std::mem::take(&mut self.entry);
        self.wrong = false;
        self.spawn(
            Work::Unlock {
                stored: stored.to_owned(),
                password,
            },
            wake,
        );
    }

    /// Submits the Settings form: checks it, then checks the current
    /// password and makes the new verifier on a thread.
    pub fn submit_form(&mut self, stored: Option<&str>, wake: impl Fn() + Send + 'static) {
        if self.checking() {
            return;
        }
        let Some(form) = self.form.as_mut() else {
            return;
        };
        if let Err(error) = form.check() {
            form.error = Some(error);
            return;
        }
        form.error = None;
        form.busy = true;
        let work = Work::Form {
            mode: form.mode,
            stored: stored.map(str::to_owned),
            current: std::mem::take(&mut form.current),
            new: std::mem::take(&mut form.new),
        };
        form.confirm.clear();
        self.spawn(work, wake);
    }

    fn spawn(&mut self, work: Work, wake: impl Fn() + Send + 'static) {
        let (sender, receiver) = std::sync::mpsc::channel();
        let spawned = std::thread::Builder::new()
            .name("app-lock".into())
            .spawn(move || {
                let _ = sender.send(run(work));
                wake();
            });
        match spawned {
            Ok(_) => self.job = Some(receiver),
            Err(error) => log::warn!("could not check the app lock password: {error}"),
        }
    }

    /// The finished check, if there is one.
    pub fn poll(&mut self) -> Option<Outcome> {
        let receiver = self.job.as_ref()?;
        match receiver.try_recv() {
            Ok(outcome) => {
                self.job = None;
                Some(outcome)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.job = None;
                if let Some(form) = &mut self.form {
                    form.busy = false;
                }
                None
            }
        }
    }

    /// Waits for the running check; for tests.
    #[cfg(test)]
    pub fn wait(&mut self) -> Option<Outcome> {
        let outcome = self
            .job
            .as_ref()?
            .recv_timeout(Duration::from_secs(30))
            .ok();
        self.job = None;
        outcome
    }

    /// Applies the lock screen's answer.
    pub fn unlocked(&mut self, matched: bool) {
        if matched {
            self.release();
            return;
        }
        self.failures = self.failures.saturating_add(1);
        self.wrong = true;
        let wait = wait_after(self.failures);
        self.retry_at = (!wait.is_zero()).then(|| self.now() + wait);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// A clock that moves only when told to.
    fn manual_clock() -> (Clock, Arc<Mutex<Instant>>) {
        let now = Arc::new(Mutex::new(Instant::now()));
        let read = Arc::clone(&now);
        (Arc::new(move || *read.lock().unwrap()), now)
    }

    fn advance(now: &Mutex<Instant>, by: Duration) {
        *now.lock().unwrap() += by;
    }

    #[test]
    fn a_verifier_is_salted_slow_and_accepts_only_its_password() {
        let first = verifier_with_rounds("correct horse", 1_000);
        let second = verifier_with_rounds("correct horse", 1_000);
        assert_ne!(first, second, "each verifier has its own salt");
        assert!(first.starts_with("pbkdf2-sha256$1000$"));
        assert!(!first.contains("correct horse"));
        assert!(verifies(&first, "correct horse"));
        assert!(verifies(&second, "correct horse"));
        assert!(!verifies(&first, "correct hors"));
        assert!(!verifies(&first, "correct horse "));
        assert!(!verifies(&first, ""));
    }

    #[test]
    fn new_verifiers_use_the_full_round_count() {
        let stored = verifier("secret-password");
        assert!(stored.starts_with(&format!("pbkdf2-sha256${ROUNDS}$")));
        assert!(verifies(&stored, "secret-password"));
    }

    #[test]
    fn malformed_or_foreign_verifiers_match_nothing() {
        let good = verifier_with_rounds("password", 10);
        let parts: Vec<&str> = good.split('$').collect();
        for stored in [
            String::new(),
            "password".to_owned(),
            format!("scrypt${}${}${}", parts[1], parts[2], parts[3]),
            format!("pbkdf2-sha256$0${}${}", parts[2], parts[3]),
            format!("pbkdf2-sha256$99999999${}${}", parts[2], parts[3]),
            format!("pbkdf2-sha256$10$zz${}", parts[3]),
            format!("{good}$extra"),
        ] {
            assert!(!verifies(&stored, "password"), "{stored}");
        }
    }

    #[test]
    fn repeated_failures_wait_longer_up_to_a_limit() {
        assert_eq!(wait_after(1), Duration::ZERO);
        assert_eq!(wait_after(2), Duration::ZERO);
        assert_eq!(wait_after(3), Duration::from_secs(1));
        assert_eq!(wait_after(4), Duration::from_secs(2));
        assert_eq!(wait_after(5), Duration::from_secs(4));
        assert_eq!(wait_after(8), MAX_WAIT);
        assert_eq!(wait_after(u32::MAX), MAX_WAIT);
    }

    #[test]
    fn inactivity_is_counted_on_the_injected_clock() {
        let (clock, now) = manual_clock();
        let mut lock = AppLock::new(false);
        lock.set_clock(clock);
        let after = Duration::from_secs(60);
        assert_eq!(lock.idle_left(after), after);
        advance(&now, Duration::from_secs(45));
        assert_eq!(lock.idle_left(after), Duration::from_secs(15));
        lock.note_input();
        assert_eq!(lock.idle_left(after), after);
        advance(&now, Duration::from_secs(61));
        assert!(lock.idle_left(after).is_zero());
    }

    fn manual_wall_clock() -> (WallClock, Arc<Mutex<SystemTime>>) {
        let now = Arc::new(Mutex::new(
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
        ));
        let read = Arc::clone(&now);
        (Arc::new(move || *read.lock().unwrap()), now)
    }

    fn advance_wall(now: &Mutex<SystemTime>, by: Duration) {
        *now.lock().unwrap() += by;
    }

    #[test]
    fn a_suspend_counts_as_inactivity_though_the_monotonic_clock_stops() {
        let (clock, _now) = manual_clock();
        let (wall, wall_now) = manual_wall_clock();
        let mut lock = AppLock::new(false);
        lock.set_clock(clock);
        lock.set_wall_clock(wall);
        let after = Duration::from_secs(60);
        assert_eq!(lock.idle_left(after), after);
        advance_wall(&wall_now, Duration::from_secs(3600));
        assert!(
            lock.idle_left(after).is_zero(),
            "a suspend is inactivity too"
        );
        lock.note_input();
        assert_eq!(lock.idle_left(after), after);
    }

    #[test]
    fn a_wall_clock_set_back_does_not_revive_an_idle_countdown() {
        let (clock, now) = manual_clock();
        let (wall, wall_now) = manual_wall_clock();
        let mut lock = AppLock::new(false);
        lock.set_clock(clock);
        lock.set_wall_clock(wall);
        let after = Duration::from_secs(60);
        advance(&now, Duration::from_secs(45));
        *wall_now.lock().unwrap() -= Duration::from_secs(3600);
        assert_eq!(lock.idle_left(after), Duration::from_secs(15));
    }

    #[test]
    fn a_saved_wait_survives_a_restart() {
        let (clock, _now) = manual_clock();
        let (wall, _wall_now) = manual_wall_clock();
        let mut lock = AppLock::new(true);
        lock.set_clock(clock);
        lock.set_wall_clock(wall);
        let stored = verifier_with_rounds("right-password", 10);
        for _ in 0..3 {
            lock.entry = "wrong".into();
            lock.try_unlock(&stored, || {});
            lock.wait();
            lock.unlocked(false);
        }
        let (failures, retry_at) = lock.wait_record();
        assert_eq!(failures, 3);
        let retry_at = retry_at.expect("the third wrong password starts a wait");
        let (clock, now) = manual_clock();
        let (wall, _wall_now) = manual_wall_clock();
        let mut restarted = AppLock::new(true);
        restarted.set_clock(clock);
        restarted.set_wall_clock(wall);
        restarted.restore_wait(failures, Some(retry_at));
        assert_eq!(restarted.wait_left(), Some(Duration::from_secs(1)));
        advance(&now, Duration::from_secs(2));
        assert_eq!(restarted.wait_left(), None);
    }

    #[test]
    fn attempts_count_on_the_wall_clock_and_reset() {
        let now = 1_700_000_000_000;
        let mut attempts = Attempts::default();
        attempts.failed(now);
        attempts.failed(now);
        assert_eq!(attempts.wait_left(now), None);
        attempts.failed(now);
        assert_eq!(attempts.failures(), 3);
        assert_eq!(attempts.wait_left(now), Some(Duration::from_secs(1)));
        assert_eq!(attempts.wait_left(now + 1_000), None);
        attempts.passed();
        assert_eq!(attempts.failures(), 0);
        assert_eq!(attempts.retry_at_ms(), None);
    }

    #[test]
    fn wrong_passwords_back_off_and_a_right_one_resets() {
        let (clock, now) = manual_clock();
        let mut lock = AppLock::new(true);
        lock.set_clock(clock);
        let stored = verifier_with_rounds("right-password", 10);
        for attempt in 1..=3 {
            lock.entry = "wrong-password".into();
            assert!(lock.can_try(), "attempt {attempt}");
            lock.try_unlock(&stored, || {});
            assert!(lock.entry.is_empty(), "the field empties once submitted");
            let outcome = lock.wait();
            assert_eq!(outcome, Some(Outcome::Unlock(false)));
            lock.unlocked(false);
            assert!(lock.is_locked() && lock.wrong);
        }
        // The third wrong password makes the next try wait a second.
        lock.entry = "right-password".into();
        assert_eq!(lock.wait_left(), Some(Duration::from_secs(1)));
        assert!(!lock.can_try());
        lock.try_unlock(&stored, || {});
        assert!(!lock.checking(), "a try during the wait is not made");
        advance(&now, Duration::from_secs(1));
        assert!(lock.can_try());
        lock.try_unlock(&stored, || {});
        assert_eq!(lock.wait(), Some(Outcome::Unlock(true)));
        lock.unlocked(true);
        assert!(!lock.is_locked());
        assert!(!lock.wrong);
        assert_eq!(lock.wait_left(), None);
        // The count starts over for the next lock.
        lock.lock();
        lock.unlocked(false);
        assert_eq!(lock.wait_left(), None);
    }

    #[test]
    fn a_new_password_needs_six_characters_typed_twice() {
        let mut form = Form::new(FormMode::Set);
        form.new = "five5".into();
        form.confirm = "five5".into();
        assert_eq!(form.check(), Err(FormError::TooShort));
        form.new = "sixsix".into();
        assert_eq!(form.check(), Err(FormError::Mismatch));
        form.confirm = "sixsix".into();
        assert_eq!(form.check(), Ok(()));
        // Characters, not bytes.
        form.new = "ééééé".into();
        form.confirm = "ééééé".into();
        assert_eq!(form.check(), Err(FormError::TooShort));
    }

    #[test]
    fn changing_or_turning_off_requires_the_current_password() {
        let stored = verifier_with_rounds("old-password", 10);
        let mut lock = AppLock::new(false);
        lock.form = Some(Form::new(FormMode::TurnOff));
        lock.form.as_mut().unwrap().current = "not-it".into();
        lock.submit_form(Some(&stored), || {});
        assert!(lock.form.as_ref().unwrap().busy);
        assert_eq!(lock.wait(), Some(Outcome::WrongCurrent));
        lock.form.as_mut().unwrap().current = "old-password".into();
        lock.submit_form(Some(&stored), || {});
        assert_eq!(lock.wait(), Some(Outcome::TurnOff));

        let mut form = Form::new(FormMode::Change);
        form.current = "not-it".into();
        form.new = "new-password".into();
        form.confirm = "new-password".into();
        lock.form = Some(form.clone());
        lock.submit_form(Some(&stored), || {});
        assert_eq!(lock.wait(), Some(Outcome::WrongCurrent));
        form.current = "old-password".into();
        lock.form = Some(form);
        lock.submit_form(Some(&stored), || {});
        let Some(Outcome::Set(new)) = lock.wait() else {
            panic!("expected a new verifier");
        };
        assert!(verifies(&new, "new-password"));
        assert!(!verifies(&new, "old-password"));
        let form = lock.form.as_ref().unwrap();
        assert!(form.current.is_empty() && form.new.is_empty() && form.confirm.is_empty());
    }

    #[test]
    fn an_invalid_form_is_refused_without_a_check() {
        let mut lock = AppLock::new(false);
        let mut form = Form::new(FormMode::Set);
        form.new = "short".into();
        form.confirm = "short".into();
        lock.form = Some(form);
        lock.submit_form(None, || {});
        assert!(!lock.checking());
        assert_eq!(lock.form.as_ref().unwrap().error, Some(FormError::TooShort));
    }
}

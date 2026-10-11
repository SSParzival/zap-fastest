//! Native Notification Center delivery, cancellation and account-scoped clicks.
//!
//! The framework requires a bundle identifier and a running main run loop.
//! The window pumps it while shown and fastframe_tray::idle while hidden.
//! Dropping a response future unregisters its observer without removing the
//! delivered notification, so the shared waiting limit also bounds macOS.

use super::{NotificationTarget, Stop, before_showing};
use mac_usernotifications::Notification;
use std::sync::{Arc, Mutex, OnceLock};

fn authorized() -> bool {
    static BUNDLED: OnceLock<bool> = OnceLock::new();
    if !*BUNDLED.get_or_init(|| mac_usernotifications::check_bundle().is_ok()) {
        log::debug!("native notifications require an application bundle");
        return false;
    }
    // Ask only once at OS level; later settings changes apply without restart.
    match mac_usernotifications::blocking::request_auth() {
        Ok(granted) => granted,
        Err(_) => {
            log::warn!("native notification authorization failed");
            false
        }
    }
}

#[expect(clippy::too_many_arguments)]
pub(super) fn deliver(
    title: &str,
    body: &str,
    native_sound: Option<&str>,
    target: NotificationTarget,
    opened: Arc<Mutex<Vec<NotificationTarget>>>,
    wake: impl Fn() + Send + 'static,
    mut cancelled: tokio::sync::oneshot::Receiver<Stop>,
    may_wait: bool,
    failed: impl FnOnce(),
) {
    if !authorized() {
        failed();
        return;
    }
    // Permission UI can outlive a read/clear request. Recheck before delivery.
    let may_wait = if may_wait {
        let Some(still_waiting) = before_showing(&mut cancelled) else {
            return;
        };
        still_waiting
    } else {
        // Release was already consumed by the caller: the sender is now gone,
        // but that means leave this notification in Notification Center.
        false
    };
    let mut notification = Notification::new().title(title).message(body);
    if let Some(sound) = native_sound {
        notification = if sound == "NSUserNotificationDefaultSoundName" {
            notification.default_sound()
        } else {
            notification.sound(sound)
        };
    }
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(_) => {
            log::warn!("native notification runtime creation failed");
            failed();
            return;
        }
    };
    runtime.block_on(async {
        let handle = match notification.send().await {
            Ok(handle) => handle,
            Err(_) => {
                log::warn!("native notification submission failed");
                failed();
                return;
            }
        };
        if !may_wait {
            return;
        }
        let id = handle.notification_id().to_owned();
        tokio::select! {
            biased;
            stop = &mut cancelled => {
                if stop != Ok(Stop::Release) {
                    mac_usernotifications::close_delivered(&id).await;
                }
            }
            response = handle.response() => {
                if response.is_ok_and(|response| response.is_default_action()) {
                    // App applies its own account and screen-lock gate when
                    // consuming this queue; callbacks never mutate app state.
                    opened.lock().unwrap_or_else(|p| p.into_inner()).push(target);
                    wake();
                }
            }
        }
    });
}

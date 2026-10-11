//! Optional local notification cards, using desktop notification eligibility.

use std::time::{Duration, Instant};

use egui::{Align2, CornerRadius, Frame, Margin, vec2};

use super::focus::{Stop, TabStop};
use crate::app::App;
use crate::model::Action;
use crate::notify::NotificationTarget;
use crate::settings::ScreenPrivacyWhat;
use crate::theme::{self, Icon};

pub(crate) const LIFETIME: Duration = Duration::from_secs(8);
pub(crate) const LIMIT: usize = 3;

#[derive(Clone)]
pub(crate) struct Notification {
    pub id: u64,
    pub target: NotificationTarget,
    pub title: String,
    pub body: String,
    pub expires: Instant,
}

pub(crate) fn show(app: &mut App, ctx: &egui::Context) {
    let now = Instant::now();
    app.in_app_notifications
        .retain(|notification| notification.expires > now);
    if app.app_lock.is_locked()
        || app.window_hidden
        || app.dialog.is_some()
        || app.show_update
        || app.image_preview.is_some()
        || app.video_expanded
        || app.picture_edit.is_some()
        || egui::Popup::is_any_open(ctx)
    {
        return;
    }
    let notifications: Vec<_> = app
        .in_app_notifications
        .iter()
        .filter_map(|notification| {
            let account = app
                .accounts
                .iter()
                .find(|account| account.id == notification.target.account)?;
            let chat = account
                .chats
                .iter()
                .find(|chat| chat.id == notification.target.chat)?;
            if !account.settings.notifications
                || !account.settings.in_app_notifications
                || chat.locked
                || chat.archived
                || chat.muted(crate::util::now())
            {
                return None;
            }
            let sender = account.settings.notification_sender
                && !app
                    .settings
                    .screen_privacy
                    .hides(ScreenPrivacyWhat::Name, false);
            let content = sender
                && account.settings.notification_content
                && !app
                    .settings
                    .screen_privacy
                    .hides(ScreenPrivacyWhat::Preview, false);
            let (generic_title, generic_body) = crate::notify::locked_lines(app.locale);
            Some((
                notification.clone(),
                if sender {
                    notification.title.clone()
                } else {
                    generic_title
                },
                if content {
                    notification.body.clone()
                } else {
                    generic_body
                },
            ))
        })
        .collect();
    if notifications.is_empty() {
        return;
    }
    let palette = app.palette;
    let width = (ctx.content_rect().width() - 32.0).clamp(160.0, 320.0);
    egui::Area::new(egui::Id::new("in-app-notifications"))
        .anchor(Align2::RIGHT_TOP, vec2(-16.0, 60.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.set_max_width(width);
            for (notification, title, body) in &notifications {
                Frame::new()
                    .fill(palette.panel)
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.set_width(width - 24.0);
                        ui.horizontal(|ui| {
                            super::widgets::rich_text(ui, title, theme::medium(14.0), palette.text);
                            let dismiss = crate::i18n::gettext(app.locale, "Dismiss notification");
                            if theme::icon_button(
                                ui,
                                Icon::X,
                                18.0,
                                palette.secondary,
                                palette.text,
                                &dismiss,
                            )
                            .tab_stop(Stop::NotificationDismiss(notification.id))
                            .clicked()
                            {
                                app.actions
                                    .push(Action::DismissInAppNotification(notification.id));
                            }
                        });
                        super::widgets::rich_text(
                            ui,
                            body,
                            theme::regular(13.0),
                            palette.secondary,
                        );
                        let open = crate::i18n::gettext(app.locale, "Open message");
                        if theme::link(ui, &*open, theme::medium(13.0), palette.link)
                            .tab_stop(Stop::NotificationOpen(notification.id))
                            .clicked()
                        {
                            app.actions
                                .push(Action::OpenInAppNotification(notification.id));
                        }
                    });
                ui.add_space(6.0);
            }
        });
    // One wake at expiry, with no animation or idle repaint loop.
    if let Some(expires) = notifications
        .iter()
        .map(|(notification, _, _)| notification.expires)
        .min()
    {
        ctx.request_repaint_after(expires.saturating_duration_since(now));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(shape: &egui::Shape, output: &mut String) {
        match shape {
            egui::Shape::Text(shape) => output.push_str(&shape.galley.job.text),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    text(shape, output);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn cards_recheck_current_privacy_and_expose_keyboard_controls() {
        let directory = tempfile::tempdir().unwrap();
        let (mut app, _) = App::headless(
            crate::paths::AppDirs::under(directory.path()),
            crate::settings::Settings::default(),
        );
        app.window_hidden = false;
        app.account_mut().settings.in_app_notifications = true;
        let chat =
            crate::model::Chat::new("fixture@s.whatsapp.net".into(), "Synthetic Sender".into());
        app.chats.push(chat.clone());
        app.in_app_notifications.push(Notification {
            id: 1,
            target: NotificationTarget {
                account: app.account().id.clone(),
                chat: chat.id,
                message: "fixture".into(),
            },
            title: "Synthetic Sender".into(),
            body: "Synthetic private preview".into(),
            expires: Instant::now() + LIFETIME,
        });
        let ctx = egui::Context::default();
        theme::install(&ctx);
        theme::apply(&ctx, &crate::theme::Palette::dark());
        let frame = |app: &mut App| {
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(640.0, 500.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    super::super::focus::begin(ui.ctx(), true);
                    show(app, ui.ctx());
                    super::super::focus::finish(ui.ctx(), true);
                },
            );
            output.textures_delta.clear();
            let mut painted = String::new();
            for shape in &output.shapes {
                text(&shape.shape, &mut painted);
            }
            painted
        };
        frame(&mut app);
        assert!(frame(&mut app).contains("Synthetic private preview"));
        assert!(super::super::focus::control(&ctx, Stop::NotificationOpen(1)).is_some());
        assert!(super::super::focus::control(&ctx, Stop::NotificationDismiss(1)).is_some());
        app.account_mut().settings.notification_sender = false;
        let hidden = frame(&mut app);
        assert!(!hidden.contains("Synthetic Sender"));
        assert!(!hidden.contains("Synthetic private preview"));
        app.account_mut().settings.notification_sender = true;
        app.settings.screen_privacy.enabled = true;
        app.settings.screen_privacy.blur_names = true;
        let hidden = frame(&mut app);
        assert!(!hidden.contains("Synthetic Sender"));
        assert!(!hidden.contains("Synthetic private preview"));
        app.chats[0].locked = true;
        assert!(!frame(&mut app).contains("Open message"));
        app.in_app_notifications[0].expires = Instant::now() - Duration::from_secs(1);
        frame(&mut app);
        assert!(app.in_app_notifications.is_empty());
    }
}

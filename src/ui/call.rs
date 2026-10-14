//! Account-local native voice controls. Views queue actions only.

use crate::app::App;
use crate::calls::{CallOutcome, CallPhase, DeviceKind, PeerAudio, SilenceReason};
use crate::i18n::{Locale, gettext};
use crate::model::{Action, CallDirection};
use crate::theme::{self, Icon};

pub fn show(app: &mut App, ctx: &egui::Context) {
    let locale = app.locale;
    if app
        .call
        .as_ref()
        .is_none_or(|call| call.video_view.is_none())
    {
        let id = egui::Id::new(("call-video-textures", app.account().id.as_str()));
        ctx.data_mut(|data| data.remove::<VideoTextures>(id));
    }
    if app.call_preparing {
        egui::Window::new(gettext(locale, "Starting call"))
            .id(egui::Id::new(("preparing-call", app.account().id.as_str())))
            .collapsible(false)
            .auto_sized()
            .default_width(320.0)
            .default_pos(ctx.content_rect().center() - egui::vec2(160.0, 64.0))
            .constrain_to(ctx.content_rect().shrink(16.0))
            .max_size(ctx.content_rect().shrink(16.0).size())
            .vscroll(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    theme::icon(ui, Icon::Phone, 32.0, app.palette.accent);
                    ui.add_space(12.0);
                    ui.label(gettext(locale, "Preparing call…"));
                    ui.add_space(16.0);
                    if control(
                        ui,
                        &app.palette,
                        128.0_f32.min(ui.available_width()),
                        Icon::PhoneOff,
                        &gettext(locale, "Cancel call"),
                        ControlTone::Danger,
                    )
                    .clicked()
                    {
                        app.actions.push(Action::HangupCall);
                    }
                    ui.add_space(12.0);
                });
            });
    }
    if let Some(call) = app
        .call
        .clone()
        .filter(|call| call.phase.is_live() && !app.call_preparing)
    {
        if app.chat(&call.chat).is_some_and(|chat| chat.locked) {
            return;
        }
        let minimized_id = minimized_id(app.account().id.as_str(), call.generation);
        let minimized = ctx
            .data(|data| data.get_temp::<bool>(minimized_id))
            .unwrap_or(false);
        if minimized {
            minimized_call(app, ctx, &call, minimized_id);
        } else {
            egui::Window::new(if call.video_active || call.video_requested {
            gettext(locale, "Video call")
        } else {
            gettext(locale, "Voice call")
        })
        .id(egui::Id::new(("compact-call", app.account().id.as_str())))
        .collapsible(false)
        .resizable(false)
        .title_bar(ctx.content_rect().height() >= 500.0)
        .constrain_to(ctx.content_rect().shrink(16.0))
        .max_size(ctx.content_rect().shrink(16.0).size())
        .auto_sized()
        .default_width(360.0)
        .default_pos(egui::pos2(ctx.content_rect().right() - 392.0, ctx.content_rect().top() + 80.0))
        .vscroll(false)
        .show(ctx, |ui| {
            let mut controls = call_controls(locale, &call);
            let compact = true;
            let split = compact && call.phase == CallPhase::Incoming && call.video_requested;
            let width = if split { (ui.available_width() - 8.0) / 2.0 } else { ((ui.available_width() - 16.0) / 3.0).min(128.0) };
            let heights: Vec<_> = controls.iter().map(|(_, label, _, _)| {
                let height = ui.painter().layout(label.to_string(), egui::TextStyle::Body.resolve(ui.style()), app.palette.text, if compact { width - 32.0 } else { width }).size().y;
                if compact { height.max(28.0) + 16.0 } else { height + 56.0 }
            }).collect();
            let controls_height = if split { heights[0].max(heights[1]) + heights[2] + 8.0 } else { heights.into_iter().fold(0.0_f32, f32::max) };
            // Reserve the actual footer plus title/margins. The body alone scrolls;
            // it must never claim the space needed by wrapped call-control labels.
            let height = (ui.available_height().min(ctx.content_rect().height() - if compact { 96.0 } else { 160.0 + ui.text_style_height(&egui::TextStyle::Heading) }) - controls_height - if compact { 4.0 } else { 36.0 })
                .clamp(16.0, 280.0);
            let body = egui::ScrollArea::vertical()
                .id_salt(("call-body", app.account().id.as_str(), call.generation))
                .max_height(height)
                .min_scrolled_height(height)
                .auto_shrink([false, true])
                .show(ui, |ui| {
            let short = ctx.content_rect().height() < 500.0;
            if short {
                minimize_row(app, ui, ctx, minimized_id, &gettext(locale, if call.video_active || call.video_requested { "Video call" } else { "Voice call" }));
            }
            ui.add_space(if compact { 0.0 } else { 12.0 });
            ui.vertical_centered(|ui| {
                if call.video_view.is_none() && !compact {
                    // An anonymous mark avoids leaking initials or photos under screen privacy.
                    let size = if ctx.content_rect().height() < 500.0 { 48.0 } else { 72.0 };
                    let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(size), egui::Sense::hover());
                    let hidden = app.settings.screen_privacy.hides(
                        crate::settings::ScreenPrivacyWhat::Name, ui.rect_contains_pointer(rect),
                    ) || app.settings.screen_privacy.hides(
                        crate::settings::ScreenPrivacyWhat::Media, ui.rect_contains_pointer(rect),
                    );
                    if hidden {
                        ui.painter().circle_filled(rect.center(), size / 2.0, app.palette.surface);
                        theme::paint_icon(ui, Icon::User, rect, size * 0.45, app.palette.secondary);
                    } else if let Some(chat) = app.chat(&call.chat) {
                        let name = app.chat_title(chat);
                        let picture = app.avatar(&call.chat);
                        super::widgets::paint_avatar(ui, &app.palette, rect, &name, &call.chat, picture.as_deref());
                    }
                    ui.add_space(8.0);
                }
                let name_width = if short { ui.available_width() } else { (ui.available_width() - theme::soft_button_width(ui, &gettext(locale, "Minimize call"), true) - ui.spacing().item_spacing.x).max(1.0) };
                let name = display_name_width(app, ui, &call.chat, egui::TextStyle::Heading, name_width);
                if !short {
                    minimize_row(app, ui, ctx, minimized_id, &name);
                } else if compact {
                    ui.label(egui::RichText::new(name).strong());
                } else {
                    ui.heading(name);
                }
                ui.label(egui::RichText::new(match call.phase {
                CallPhase::Dialing => gettext(locale, "Calling…"),
                CallPhase::Ringing => gettext(locale, "Ringing…"),
                CallPhase::Incoming if call.video_requested => {
                    gettext(locale, "Incoming video call")
                }
                CallPhase::Incoming => gettext(locale, "Incoming voice call"),
                CallPhase::Connecting | CallPhase::Accepted => gettext(locale, "Connecting…"),
                CallPhase::Active => gettext(locale, "Connected"),
                CallPhase::Ended => gettext(locale, "Call ended"),
                CallPhase::Failed => gettext(locale, "Call failed"),
                }).color(app.palette.secondary));
                if let Some(start) = call.started {
                ui.label(format!(
                    "{}:{:02}",
                    start.elapsed().as_secs() / 60,
                    start.elapsed().as_secs() % 60
                ));
                ctx.request_repaint_after(std::time::Duration::from_secs(1));
                }
            });
            ui.add_space(12.0);
            if let Some(audio) = call.peer_audio {
                ui.label(peer_audio_label(locale, audio));
            }
            if call.audio_processing_failed {
                ui.label(gettext(locale, "Microphone cleanup is unavailable. Audio is being sent without echo cancellation."));
            }
            for lost in &call.lost_devices {
                ui.label(
                    gettext(locale, "{device} is unavailable; using the system default")
                        .replace("{device}", &lost.name),
                );
            }
            if call.phase == CallPhase::Active {
                if call.video_starting {
                    ui.label(gettext(locale, "Opening the camera…"));
                } else if !call.video_active {
                    if call.video_requested {
                        ui.label(gettext(locale, "The other person requested video."));
                    }
                    if call.video_requested
                        && ui.button(gettext(locale, "Keep voice only")).clicked()
                    {
                        app.actions.push(Action::SetCallVideo(false));
                    }
                }
            }
            if let Some(view) = &call.video_view {
                let max_video_height = video_height_limit(ui, height);
                video(app, ui, ctx, call.generation, view, max_video_height);
            }
            ui.add_space(8.0);
            ui.separator();
            let id = devices_open_id(app.account().id.as_str());
            let mut expanded = ctx.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
            let toggle = theme::soft_button(ui, &app.palette, Some(if expanded { Icon::ChevronUp } else { Icon::Settings }), &gettext(locale, "Audio and camera devices"), expanded);
            #[cfg(test)]
            ctx.data_mut(|data| data.insert_temp(id.with("bounds"), toggle.rect));
            if toggle.clicked() {
                expanded = !expanded;
                ctx.data_mut(|data| data.insert_temp(id, expanded));
                if expanded {
                    app.actions.push(Action::RefreshAudioDevices);
                }
            }
            if expanded {
                ui.add_space(8.0);
                devices(app, ui);
            }
            ui.add_space(8.0);
                });
            #[cfg(test)]
            ctx.data_mut(|data| data.insert_temp(devices_open_id(app.account().id.as_str()).with("scroll"), body.inner_rect));
            let _ = body;
            ui.add_space(if compact { 0.0 } else { 12.0 });
            ui.separator();
            ui.add_space(if compact { 0.0 } else { 8.0 });
            if split {
                let decline = controls.pop().expect("incoming video has a decline control");
                control_row(app, ui, width, controls);
                ui.add_space(8.0);
                control_row(app, ui, width, vec![decline]);
            } else {
                control_row(app, ui, width, controls);
            }
            ui.add_space(if compact { 4.0 } else { 8.0 });
        });
        }
    }
    if app.call_log_open {
        let mut open = true;
        let popup_was_open = egui::Popup::is_any_open(ctx);
        let window = egui::Window::new(gettext(locale, "Calls"))
            .id(egui::Id::new(("call-log", app.account().id.as_str())))
            .open(&mut open)
            .collapsible(false)
            .default_width(400.0)
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 72.0))
            .vscroll(true)
            .constrain_to(ctx.content_rect().shrink(16.0))
            .max_size(ctx.content_rect().shrink(16.0).size())
            .show(ctx, |ui| {
                ui.heading(gettext(locale, "Recent calls"));
                ui.add_space(8.0);
                egui::ScrollArea::vertical()
                    .max_height(350.0)
                    .show(ui, |ui| {
                        for record in app.call_records.clone() {
                            if app.chat(&record.chat).is_some_and(|chat| chat.locked) {
                                continue;
                            }
                            let name = display_name(app, ui, &record.chat, egui::TextStyle::Body);
                            ui.vertical(|ui| {
                                if ui.link(name).clicked() {
                                    app.actions.push(Action::OpenChat(record.chat.clone()));
                                    app.actions.push(Action::ToggleCallLog);
                                }
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(crate::util::moment_stamp(locale, record.at));
                                    ui.label(format!(
                                        "{} · {}",
                                        direction_label(locale, record.direction),
                                        outcome_label(locale, record.outcome)
                                    ));
                                    if let Some(duration) = record.duration {
                                        ui.label(format!("{}:{:02}", duration / 60, duration % 60));
                                    }
                                });
                            });
                            ui.add_space(8.0);
                        }
                        if app.call_records.is_empty() {
                            ui.label(gettext(locale, "No calls recorded on this computer yet."));
                        }
                    });
                ui.separator();
                egui::CollapsingHeader::new(gettext(locale, "Audio and camera devices"))
                    .show(ui, |ui| devices(app, ui));
            });
        let popup_open = popup_was_open || egui::Popup::is_any_open(ctx);
        let clicked_outside = window.as_ref().is_some_and(|window| {
            ctx.input(|input| {
                input.pointer.any_pressed()
                    && input
                        .pointer
                        .interact_pos()
                        .is_some_and(|position| !window.response.rect.contains(position))
            })
        });
        let escape = !popup_open
            && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        if !open || (!popup_open && clicked_outside) || escape {
            app.actions.push(Action::ToggleCallLog);
        }
    }
}

#[derive(Clone, Default)]
struct VideoTextures {
    generation: u64,
    local: Option<egui::TextureHandle>,
    peer: Option<egui::TextureHandle>,
}

#[cfg(test)]
pub(crate) fn video_texture_ids(account: &str, ctx: &egui::Context) -> Vec<egui::TextureId> {
    ctx.data(|data| data.get_temp::<VideoTextures>(egui::Id::new(("call-video-textures", account))))
        .map(|textures| {
            [textures.peer, textures.local]
                .into_iter()
                .flatten()
                .map(|texture| texture.id())
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn devices_open_id(account: &str) -> egui::Id {
    egui::Id::new(("live-call-devices-open", account))
}

#[derive(Clone, Copy)]
enum ControlTone {
    Neutral,
    Selected,
    Primary,
    Danger,
}

type CallControl = (Icon, std::borrow::Cow<'static, str>, ControlTone, Action);

fn control_row(app: &mut App, ui: &mut egui::Ui, width: f32, controls: Vec<CallControl>) {
    ui.horizontal_top(|ui| {
        let count = controls.len() as f32;
        ui.add_space(
            ((ui.available_width() - (width * count + (count - 1.0) * 8.0)) / 2.0).max(0.0),
        );
        ui.spacing_mut().item_spacing.x = 8.0;
        let height = controls
            .iter()
            .map(|(_, label, _, _)| control_height(ui, width, label))
            .fold(44.0_f32, f32::max);
        for (icon, label, tone, action) in controls {
            if control_sized(
                ui,
                &app.palette,
                egui::vec2(width, height),
                icon,
                &label,
                tone,
            )
            .clicked()
            {
                app.actions.push(action);
            }
        }
    });
}

fn call_controls(locale: Locale, call: &crate::calls::CallUpdate) -> Vec<CallControl> {
    let mut controls = Vec::with_capacity(3);
    if call.phase == CallPhase::Incoming {
        if call.video_requested {
            controls.push((
                Icon::Video,
                gettext(locale, "Answer with video"),
                ControlTone::Primary,
                Action::AnswerVideoCall,
            ));
        }
        controls.push((
            Icon::Phone,
            if call.video_requested {
                gettext(locale, "Answer without camera")
            } else {
                gettext(locale, "Answer")
            },
            ControlTone::Primary,
            Action::AnswerCall,
        ));
        controls.push((
            Icon::PhoneOff,
            gettext(locale, "Decline"),
            ControlTone::Danger,
            Action::DeclineCall,
        ));
    } else {
        controls.push((
            if call.muted { Icon::MicOff } else { Icon::Mic },
            if call.muted {
                gettext(locale, "Unmute")
            } else {
                gettext(locale, "Mute")
            },
            if call.muted {
                ControlTone::Selected
            } else {
                ControlTone::Neutral
            },
            Action::SetCallMuted(!call.muted),
        ));
        if call.phase == CallPhase::Active {
            let (label, enable) = if call.video_starting {
                (gettext(locale, "Cancel video"), false)
            } else if call.video_active {
                (gettext(locale, "Turn camera off"), false)
            } else if call.video_requested {
                (gettext(locale, "Accept video and turn camera on"), true)
            } else {
                (gettext(locale, "Turn camera on"), true)
            };
            controls.push((
                if enable { Icon::VideoOff } else { Icon::Video },
                label,
                if enable {
                    ControlTone::Neutral
                } else {
                    ControlTone::Selected
                },
                Action::SetCallVideo(enable),
            ));
        }
        controls.push((
            Icon::PhoneOff,
            gettext(locale, "End call"),
            ControlTone::Danger,
            Action::HangupCall,
        ));
    }
    controls
}

/// Named call targets, with compact inline icons for very short windows.
fn control(
    ui: &mut egui::Ui,
    palette: &theme::Palette,
    width: f32,
    icon: Icon,
    label: &str,
    tone: ControlTone,
) -> egui::Response {
    let height = control_height(ui, width, label);
    control_sized(ui, palette, egui::vec2(width, height), icon, label, tone)
}

fn control_height(ui: &egui::Ui, width: f32, label: &str) -> f32 {
    ui.painter()
        .layout(
            label.to_owned(),
            egui::TextStyle::Body.resolve(ui.style()),
            egui::Color32::WHITE,
            (width - 32.0).max(1.0),
        )
        .size()
        .y
        .max(28.0)
        + 16.0
}

fn control_sized(
    ui: &mut egui::Ui,
    palette: &theme::Palette,
    size: egui::Vec2,
    icon: Icon,
    label: &str,
    tone: ControlTone,
) -> egui::Response {
    let width = size.x;
    let font = egui::TextStyle::Body.resolve(ui.style());
    let compact = true;
    let galley = ui.painter().layout(
        label.to_owned(),
        font,
        palette.text,
        (width - if compact { 32.0 } else { 0.0 }).max(1.0),
    );
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    theme::reveal_focus(&response);
    #[cfg(test)]
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("call-control-test", label)),
            (response.id, response.rect),
        )
    });
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            matches!(tone, ControlTone::Selected),
            label,
        )
    });
    let (fill, hover) = match tone {
        ControlTone::Primary | ControlTone::Selected => (palette.accent, palette.accent_hover),
        ControlTone::Danger => (
            palette.danger,
            theme::blend(palette.danger, palette.text, 0.12),
        ),
        ControlTone::Neutral => (palette.surface, palette.surface_hover),
    };
    let fill = if response.hovered() { hover } else { fill };
    let foreground = if matches!(tone, ControlTone::Neutral) {
        palette.text
    } else if theme::contrast(fill, egui::Color32::BLACK)
        > theme::contrast(fill, egui::Color32::WHITE)
    {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    };
    let circle = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.top() + 24.0),
        egui::Vec2::splat(48.0),
    );
    theme::focus_outline_on_fill(
        ui,
        response.id,
        if compact { rect } else { circle },
        if compact { 6.0 } else { 24.0 },
        fill,
    );
    if ui.is_rect_visible(rect) {
        if compact {
            ui.painter().rect_filled(rect, 6.0, fill);
            theme::paint_icon(
                ui,
                icon,
                egui::Rect::from_center_size(
                    egui::pos2(rect.left() + 16.0, rect.center().y),
                    egui::Vec2::splat(18.0),
                ),
                18.0,
                foreground,
            );
            ui.painter().galley(
                egui::pos2(rect.left() + 30.0, rect.center().y - galley.size().y / 2.0),
                galley,
                foreground,
            );
        } else {
            ui.painter().circle_filled(circle.center(), 24.0, fill);
            theme::paint_icon(
                ui,
                icon,
                circle,
                if response.is_pointer_button_down_on() {
                    21.0
                } else {
                    23.0
                },
                foreground,
            );
            ui.painter().galley(
                egui::pos2(rect.center().x - galley.size().x / 2.0, rect.top() + 56.0),
                galley,
                palette.text,
            );
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn display_name(app: &App, ui: &egui::Ui, chat: &str, style: egui::TextStyle) -> String {
    display_name_width(app, ui, chat, style, ui.available_width())
}

fn display_name_width(
    app: &App,
    ui: &egui::Ui,
    chat: &str,
    style: egui::TextStyle,
    width: f32,
) -> String {
    let region = egui::Rect::from_min_size(
        ui.cursor().min,
        egui::vec2(width, ui.text_style_height(&style)),
    );
    if app.settings.screen_privacy.hides(
        crate::settings::ScreenPrivacyWhat::Name,
        ui.rect_contains_pointer(region),
    ) {
        return gettext(app.locale, "WhatsApp contact").into_owned();
    }
    app.chat(chat)
        .map(|chat| app.chat_title(chat))
        .unwrap_or_else(|| gettext(app.locale, "WhatsApp contact").into_owned())
}

fn video(
    app: &App,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    generation: u64,
    view: &crate::camera::VideoView,
    max_height: f32,
) {
    let locale = app.locale;
    view.attach(ctx);
    let id = egui::Id::new(("call-video-textures", app.account().id.as_str()));
    let mut textures = ctx
        .data_mut(|data| data.get_temp::<VideoTextures>(id))
        .unwrap_or_default();
    if textures.generation != generation {
        textures = VideoTextures {
            generation,
            ..Default::default()
        };
    }
    for (slot, image, name) in [
        (&mut textures.peer, view.take_peer(), "call-peer"),
        (&mut textures.local, view.take_local(), "call-camera"),
    ] {
        if let Some(image) = image {
            if let Some(texture) = slot {
                texture.set(image, egui::TextureOptions::LINEAR);
            } else {
                *slot = Some(ctx.load_texture(name, image, egui::TextureOptions::LINEAR));
            }
        }
    }
    let width = ui.available_width();
    let height = if textures.peer.is_none() && textures.local.is_none() {
        56.0_f32.min(max_height)
    } else {
        (width * 9.0 / 16.0)
            .min(ctx.content_rect().height() * 0.22)
            .min(max_height)
    };
    let (region, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let hidden = app.settings.screen_privacy.hides(
        crate::settings::ScreenPrivacyWhat::Media,
        ui.rect_contains_pointer(region),
    );
    #[cfg(test)]
    ctx.data_mut(|data| {
        data.insert_temp(
            video_bounds_id(app.account().id.as_str()),
            (region, ui.clip_rect()),
        )
    });
    ui.painter().rect_filled(region, 8.0, app.palette.surface);
    let paint_frame = |ui: &egui::Ui, texture: &egui::TextureHandle, bounds: egui::Rect| {
        let size = texture.size_vec2();
        let fitted = size * (bounds.width() / size.x).min(bounds.height() / size.y);
        let image = egui::Rect::from_center_size(bounds.center(), fitted);
        egui::Image::new(texture)
            .corner_radius(6.0)
            .paint_at(ui, image);
    };
    if hidden {
        ui.put(
            region,
            egui::Label::new(gettext(locale, "Content hidden")).wrap(),
        );
    } else {
        if let Some(texture) = &textures.peer {
            paint_frame(ui, texture, region);
        } else {
            ui.put(
                region.shrink(16.0),
                egui::Label::new(gettext(locale, "Waiting for the other person's video…")).wrap(),
            );
        }
        if let Some(texture) = &textures.local {
            let size = egui::vec2((width * 0.25).min(104.0), (height * 0.4).min(78.0));
            let bounds = egui::Rect::from_min_size(
                region.right_bottom() - size - egui::vec2(8.0, 8.0),
                size,
            );
            ui.painter()
                .rect_filled(bounds.expand(2.0), 8.0, app.palette.panel);
            paint_frame(ui, texture, bounds);
            ui.label(
                egui::RichText::new(gettext(locale, "Your camera")).color(app.palette.secondary),
            );
        }
    }
    if let Some(error) = view.error() {
        ui.label(gettext(locale, error));
    }
    ctx.data_mut(|data| data.insert_temp(id, textures));
}

pub(crate) fn devices(app: &mut App, ui: &mut egui::Ui) {
    let locale = app.locale;
    for kind in [DeviceKind::Microphone, DeviceKind::Speaker] {
        let discovery_error = match kind {
            DeviceKind::Microphone => app.call_devices.microphone_error.clone(),
            DeviceKind::Speaker => app.call_devices.speaker_error.clone(),
        };
        let mut chosen = match kind {
            DeviceKind::Microphone => app.account().settings.call_microphone.clone(),
            DeviceKind::Speaker => app.account().settings.call_speaker.clone(),
        };
        let before = chosen.clone();
        let entries = match kind {
            DeviceKind::Microphone => app.call_devices.microphones.clone(),
            DeviceKind::Speaker => app.call_devices.speakers.clone(),
        };
        let field_label = match kind {
            DeviceKind::Microphone => gettext(locale, "Microphone"),
            DeviceKind::Speaker => gettext(locale, "Speaker"),
        };
        ui.label(field_label.as_ref());
        let label = entries
            .iter()
            .find(|entry| chosen.as_deref() == Some(entry.id.as_str()))
            .map(|entry| entry.label.as_str())
            .unwrap_or_else(|| chosen.as_deref().unwrap_or("System default"));
        let accessible_value = label.to_owned();
        let selector = egui::ComboBox::from_id_salt((
            "call-device",
            app.account().id.as_str(),
            kind == DeviceKind::Microphone,
        ))
        .selected_text(label)
        .width(ui.available_width().min(320.0))
        .truncate()
        .show_ui(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            ui.selectable_value(&mut chosen, None, gettext(locale, "System default"));
            for entry in entries {
                ui.selectable_value(&mut chosen, Some(entry.id), entry.label);
            }
        });
        ui.ctx()
            .accesskit_node_builder(selector.response.id, |node| {
                node.set_label(field_label.as_ref());
                node.set_value(accessible_value);
            });
        #[cfg(test)]
        ui.ctx().data_mut(|data| {
            data.insert_temp(
                device_bounds_id(app.account().id.as_str(), kind),
                (selector.response.rect, ui.clip_rect()),
            );
        });
        let _ = selector;
        if discovery_error.is_some() {
            ui.label(gettext(
                locale,
                match kind {
                    DeviceKind::Microphone => {
                        "Microphone discovery failed. Try refreshing devices."
                    }
                    DeviceKind::Speaker => "Speaker discovery failed. Try refreshing devices.",
                },
            ));
        }
        if chosen != before {
            app.actions.push(match kind {
                DeviceKind::Microphone => Action::SetCallMicrophone(chosen),
                DeviceKind::Speaker => Action::SetCallSpeaker(chosen),
            });
        }
    }
    ui.label(gettext(locale, "Camera"));
    let mut camera = app.account().settings.call_camera.clone();
    let before = camera.clone();
    let label = app
        .call_devices
        .cameras
        .iter()
        .find(|entry| camera.as_deref() == Some(entry.id.as_str()))
        .map(|entry| std::borrow::Cow::Borrowed(entry.label.as_str()))
        .unwrap_or_else(|| {
            if camera.is_some() {
                if app.call_devices.cameras_discovered {
                    gettext(locale, "Selected camera unavailable")
                } else {
                    gettext(locale, "Saved camera (refresh devices)")
                }
            } else {
                gettext(locale, "System default")
            }
        });
    let accessible_value = label.to_string();
    let camera_selector =
        egui::ComboBox::from_id_salt(("call-camera-device", app.account().id.as_str()))
            .selected_text(label)
            .width(ui.available_width().min(320.0))
            .truncate()
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.selectable_value(&mut camera, None, gettext(locale, "System default"));
                for entry in &app.call_devices.cameras {
                    ui.selectable_value(&mut camera, Some(entry.id.clone()), &entry.label);
                }
            });
    ui.ctx()
        .accesskit_node_builder(camera_selector.response.id, |node| {
            node.set_label(gettext(locale, "Camera"));
            node.set_value(accessible_value);
        });
    #[cfg(test)]
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            camera_bounds_id(app.account().id.as_str()),
            (camera_selector.response.rect, ui.clip_rect()),
        );
    });
    let _ = camera_selector;
    ui.label(gettext(locale, "Used next time the camera is turned on."));
    if app.call_devices.camera_error.is_some() {
        ui.label(gettext(
            locale,
            "Camera discovery failed. Try refreshing devices.",
        ));
    }
    if camera != before {
        app.actions.push(Action::SetCallCamera(camera));
    }
    if ui.button(gettext(locale, "Refresh devices")).clicked() {
        app.actions.push(Action::RefreshAudioDevices);
    }
}

#[cfg(test)]
pub(crate) fn camera_bounds_id(account: &str) -> egui::Id {
    egui::Id::new(("call-camera-bounds", account))
}

#[cfg(test)]
pub(crate) fn device_bounds_id(account: &str, kind: DeviceKind) -> egui::Id {
    egui::Id::new((
        "call-device-bounds",
        account,
        kind == DeviceKind::Microphone,
    ))
}

fn direction_label(locale: Locale, direction: CallDirection) -> std::borrow::Cow<'static, str> {
    match direction {
        CallDirection::Incoming => gettext(locale, "Incoming"),
        CallDirection::Outgoing => gettext(locale, "Outgoing"),
    }
}

fn outcome_label(locale: Locale, outcome: CallOutcome) -> std::borrow::Cow<'static, str> {
    match outcome {
        CallOutcome::Answered => gettext(locale, "Answered"),
        CallOutcome::Missed => gettext(locale, "Missed call"),
        CallOutcome::Declined => gettext(locale, "Declined"),
        CallOutcome::Busy => gettext(locale, "Busy"),
        CallOutcome::Failed => gettext(locale, "Call failed"),
        CallOutcome::NoAnswer => gettext(locale, "No answer"),
        CallOutcome::ConnectionLost => gettext(locale, "Connection lost"),
        CallOutcome::AnsweredElsewhere => gettext(locale, "Answered on another device"),
        CallOutcome::DeclinedElsewhere => gettext(locale, "Declined on another device"),
    }
}

fn peer_audio_label(locale: Locale, audio: PeerAudio) -> std::borrow::Cow<'static, str> {
    match audio {
        PeerAudio::Stalled => gettext(locale, "No audio is arriving. Check the connection."),
        PeerAudio::Silent(SilenceReason::NoDecoder) => {
            gettext(locale, "This audio format cannot be decoded by this build.")
        }
        PeerAudio::Silent(SilenceReason::AuthenticationFailing) => {
            gettext(locale, "The incoming audio could not be authenticated.")
        }
        PeerAudio::Silent(_) => gettext(
            locale,
            "Incoming audio cannot be played. Try reconnecting the call.",
        ),
    }
}

pub fn minimized_id(account: &str, generation: u64) -> egui::Id {
    egui::Id::new(("minimized-call", account, generation))
}

fn minimized_call(
    app: &mut App,
    ctx: &egui::Context,
    call: &crate::calls::CallUpdate,
    state: egui::Id,
) {
    let body_height = (ctx.content_rect().height() - 120.0).clamp(16.0, 180.0);
    egui::Window::new(gettext(app.locale, "Call"))
        .id(state.with("window"))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .auto_sized()
        .default_width(240.0)
        .default_pos(egui::pos2(
            ctx.content_rect().right() - 272.0,
            ctx.content_rect().top() + 80.0,
        ))
        .constrain_to(ctx.content_rect().shrink(16.0))
        .max_size(ctx.content_rect().shrink(16.0).size())
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(state.with("body"))
                .max_height(body_height)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let name = display_name(app, ui, &call.chat, egui::TextStyle::Body);
                        ui.add(egui::Label::new(name).truncate());
                    });
                    if let Some(view) = &call.video_view {
                        let max_video_height = video_height_limit(ui, body_height);
                        video(app, ui, ctx, call.generation, view, max_video_height);
                    }
                    if let Some(audio) = call.peer_audio {
                        ui.label(peer_audio_label(app.locale, audio));
                    }
                    if call.video_requested && !call.video_active {
                        ui.label(gettext(app.locale, "The other person requested video."));
                    }
                });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                for (icon, label, tone, action) in call_controls(app.locale, call) {
                    let color = if matches!(tone, ControlTone::Danger) {
                        app.palette.danger
                    } else if matches!(tone, ControlTone::Selected) {
                        app.palette.accent
                    } else {
                        app.palette.text
                    };
                    let response =
                        theme::icon_button(ui, icon, 32.0, color, app.palette.accent_hover, &label);
                    #[cfg(test)]
                    ctx.data_mut(|data| {
                        data.insert_temp(
                            egui::Id::new(("call-control-test", label.as_ref())),
                            (response.id, response.rect),
                        )
                    });
                    if response.clicked() {
                        app.actions.push(action);
                    }
                }
                let restore = theme::icon_button(
                    ui,
                    Icon::Maximize,
                    32.0,
                    app.palette.text,
                    app.palette.accent_hover,
                    &gettext(app.locale, "Restore call"),
                );
                #[cfg(test)]
                ctx.data_mut(|data| data.insert_temp(state.with("toggle"), restore.rect));
                if restore.clicked() {
                    ctx.data_mut(|data| data.insert_temp(state, false));
                }
            });
        });
}

fn minimize_row(app: &App, ui: &mut egui::Ui, ctx: &egui::Context, state: egui::Id, title: &str) {
    let label = gettext(app.locale, "Minimize call");
    let width = theme::soft_button_width(ui, &label, true);
    ui.horizontal(|ui| {
        let room = (ui.available_width() - width - ui.spacing().item_spacing.x).max(1.0);
        ui.add_sized(
            egui::vec2(room, 24.0),
            egui::Label::new(egui::RichText::new(title).strong()).truncate(),
        );
        let minimize = theme::soft_button(ui, &app.palette, Some(Icon::Minus), &label, false);
        #[cfg(test)]
        ctx.data_mut(|data| data.insert_temp(state.with("toggle"), minimize.rect));
        if minimize.clicked() {
            ctx.data_mut(|data| data.insert_temp(state, true));
        }
    });
}

fn video_height_limit(ui: &egui::Ui, body_height: f32) -> f32 {
    let above = (ui.cursor().top() - ui.min_rect().top()).max(0.0);
    (body_height
        - above
        - ui.text_style_height(&egui::TextStyle::Body)
        - ui.spacing().item_spacing.y
        - 4.0)
        .max(32.0)
}

#[cfg(test)]
pub(crate) fn video_bounds_id(account: &str) -> egui::Id {
    egui::Id::new(("call-video-bounds", account))
}

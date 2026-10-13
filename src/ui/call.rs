//! Account-local native voice controls. Views queue actions only.

use crate::app::App;
use crate::calls::{CallOutcome, CallPhase, DeviceKind, PeerAudio, SilenceReason};
use crate::i18n::{Locale, gettext};
use crate::model::{Action, CallDirection};

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
                ui.label(gettext(locale, "Preparing call…"));
                if ui.button(gettext(locale, "Cancel call")).clicked() {
                    app.actions.push(Action::HangupCall);
                }
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
        egui::Window::new(if call.video_active || call.video_requested {
            gettext(locale, "Video call")
        } else {
            gettext(locale, "Voice call")
        })
        .id(egui::Id::new(("voice-call", app.account().id.as_str())))
        .collapsible(false)
        .resizable(false)
        .constrain_to(ctx.content_rect().shrink(16.0))
        .max_size(ctx.content_rect().shrink(16.0).size())
        .default_width(400.0)
        .default_height(if call.video_requested || call.video_active {
            ctx.content_rect().shrink(16.0).height()
        } else {
            ctx.content_rect().shrink(16.0).height().min(520.0)
        })
        .default_pos(ctx.content_rect().center() - egui::vec2(200.0, 200.0))
        .vscroll(true)
        .show(ctx, |ui| {
            let name = display_name(app, ui, &call.chat, egui::TextStyle::Heading);
            ui.heading(name);
            ui.label(match call.phase {
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
            });
            if let Some(start) = call.started {
                ui.label(format!(
                    "{}:{:02}",
                    start.elapsed().as_secs() / 60,
                    start.elapsed().as_secs() % 60
                ));
                ctx.request_repaint_after(std::time::Duration::from_secs(1));
            }
            ui.horizontal_wrapped(|ui| {
                if call.phase == CallPhase::Incoming {
                    if call.video_requested
                        && ui.button(gettext(locale, "Answer with video")).clicked()
                    {
                        app.actions.push(Action::AnswerVideoCall);
                    }
                    if ui
                        .button(if call.video_requested {
                            gettext(locale, "Answer without camera")
                        } else {
                            gettext(locale, "Answer")
                        })
                        .clicked()
                    {
                        app.actions.push(Action::AnswerCall);
                    }
                    if ui.button(gettext(locale, "Decline")).clicked() {
                        app.actions.push(Action::DeclineCall);
                    }
                } else {
                    if ui
                        .button(if call.muted {
                            gettext(locale, "Unmute")
                        } else {
                            gettext(locale, "Mute")
                        })
                        .clicked()
                    {
                        app.actions.push(Action::SetCallMuted(!call.muted));
                    }
                    if ui.button(gettext(locale, "End call")).clicked() {
                        app.actions.push(Action::HangupCall);
                    }
                }
            });
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
                    if ui.button(gettext(locale, "Cancel video")).clicked() {
                        app.actions.push(Action::SetCallVideo(false));
                    }
                } else if call.video_active {
                    if ui.button(gettext(locale, "Turn camera off")).clicked() {
                        app.actions.push(Action::SetCallVideo(false));
                    }
                } else {
                    if call.video_requested {
                        ui.label(gettext(locale, "The other person requested video."));
                    }
                    if ui
                        .button(if call.video_requested {
                            gettext(locale, "Accept video and turn camera on")
                        } else {
                            gettext(locale, "Turn camera on")
                        })
                        .clicked()
                    {
                        app.actions.push(Action::SetCallVideo(true));
                    }
                    if call.video_requested
                        && ui.button(gettext(locale, "Keep voice only")).clicked()
                    {
                        app.actions.push(Action::SetCallVideo(false));
                    }
                }
            }
            if let Some(view) = &call.video_view {
                video(app, ui, ctx, call.generation, view);
            }
            ui.add_space(8.0);
            devices(app, ui);
        });
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

fn display_name(app: &App, ui: &egui::Ui, chat: &str, style: egui::TextStyle) -> String {
    let region = egui::Rect::from_min_size(
        ui.cursor().min,
        egui::vec2(ui.available_width(), ui.text_style_height(&style)),
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
    // Share a bounded preview budget so call controls remain visible in a short window.
    // The window scrolls longer device errors or larger text without losing any controls.
    let preview_height = (ctx.content_rect().height() * 0.3).min(480.0);
    let width = ui.available_width();
    let fit = |texture: &egui::TextureHandle, max: egui::Vec2| {
        let size = texture.size_vec2();
        size * (max.x / size.x).min(max.y / size.y).min(1.0)
    };
    let spacing = ui.spacing().item_spacing.y;
    let peer_size = textures
        .peer
        .as_ref()
        .map(|texture| fit(texture, egui::vec2(width.min(480.0), preview_height * 0.75)));
    let local_size = textures
        .local
        .as_ref()
        .map(|texture| fit(texture, egui::vec2(width.min(160.0), preview_height * 0.25)));
    let waiting = gettext(locale, "Waiting for the other person's video…");
    let height = peer_size.map_or_else(
        || {
            ui.painter()
                .layout(
                    waiting.to_string(),
                    egui::TextStyle::Body.resolve(ui.style()),
                    app.palette.text,
                    width,
                )
                .size()
                .y
        },
        |size| size.y,
    ) + local_size.map_or(0.0, |size| {
        size.y + ui.text_style_height(&egui::TextStyle::Body) + 2.0 * spacing
    });
    let (region, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let hidden = app.settings.screen_privacy.hides(
        crate::settings::ScreenPrivacyWhat::Media,
        ui.rect_contains_pointer(region),
    );
    let mut preview_ui = ui.new_child(egui::UiBuilder::new().max_rect(region));
    preview_ui.set_clip_rect(region.intersect(ui.clip_rect()));
    if hidden && (peer_size.is_some() || local_size.is_some()) {
        preview_ui
            .painter()
            .rect_filled(region, 6.0, app.palette.surface);
        let text = super::widgets::line(
            &preview_ui,
            &gettext(locale, "Content hidden"),
            crate::theme::regular(13.5),
            app.palette.secondary,
            width,
            2,
        );
        text.paint(
            &preview_ui,
            region.center() - text.size() / 2.0,
            app.palette.secondary,
        );
    } else {
        if let Some(texture) = &textures.peer {
            preview_ui.add(
                egui::Image::new(texture)
                    .max_size(egui::vec2(width.min(480.0), preview_height * 0.75)),
            );
        } else {
            preview_ui.label(waiting);
        }
        if let Some(texture) = &textures.local {
            preview_ui.label(gettext(locale, "Your camera"));
            preview_ui.add(
                egui::Image::new(texture)
                    .max_size(egui::vec2(width.min(160.0), preview_height * 0.25)),
            );
        }
    }
    if let Some(error) = view.error() {
        ui.label(error);
    }
    ctx.data_mut(|data| data.insert_temp(id, textures));
}

pub(super) fn devices(app: &mut App, ui: &mut egui::Ui) {
    let locale = app.locale;
    for kind in [DeviceKind::Microphone, DeviceKind::Speaker] {
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

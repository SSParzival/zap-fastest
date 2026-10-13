//! Operate regressions use synthetic accounts and never open a personal archive.

use super::{apply_flags, tests::app};

fn frames(app: &mut crate::app::App, ctx: &egui::Context, size: egui::Vec2) {
    for frame in 0..8 {
        let mut output = ctx.run_ui(
            egui::RawInput {
                time: Some(frame as f64),
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ui| {
                app.background_frame(ui.ctx());
                app.frame_ui(ui);
            },
        );
        output.textures_delta.clear();
    }
}

#[test]
fn forms_fit_narrow_short_windows_with_enlarged_text() {
    let mut failures = Vec::new();
    for page in [
        "archive-backup",
        "archive-backup-mismatch",
        "archive-restore",
        "archive-merge",
        "archive-resume",
        "about",
        "new-chat",
        "new-contact",
        "message-number",
        "poll-create",
        "location",
        "unlink",
        "delete-chat",
        "clear-chat",
        "delete-message",
        "locked-setup",
        "labels-dialog",
        "info",
        "group-info",
    ] {
        for scale in [1.0, 1.5] {
            let mut app = app();
            apply_flags(&mut app, Some(page));
            app.settings.text_scale = scale;
            let ctx = egui::Context::default();
            app.attach(&ctx);
            let size = egui::vec2(360.0, 440.0);
            frames(&mut app, &ctx, size);
            let rect = ctx
                .memory(|memory| memory.area_rect(egui::Id::new("dialog")))
                .expect("synthetic dialog is open");
            if !egui::Rect::from_min_size(egui::Pos2::ZERO, size)
                .shrink(8.0)
                .contains_rect(rect)
            {
                failures.push(format!("{page}, scale {scale}: {rect:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn composer_pickers_fit_narrow_short_windows() {
    for page in ["picker", "gifs", "stickers"] {
        let mut app = app();
        apply_flags(&mut app, Some(page));
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let size = egui::vec2(360.0, 440.0);
        frames(&mut app, &ctx, size);
        let rect = ctx
            .memory(|memory| memory.area_rect(egui::Id::new("picker")))
            .expect("synthetic picker is open");
        assert!(
            egui::Rect::from_min_size(egui::Pos2::ZERO, size)
                .shrink(4.0)
                .contains_rect(rect),
            "{page}: {rect:?}"
        );
    }
}

#[test]
fn backup_validation_explains_a_disabled_save_without_exposing_the_passphrase() {
    let mut app = app();
    apply_flags(&mut app, Some("archive-backup-mismatch"));
    let ctx = egui::Context::default();
    app.attach(&ctx);
    ctx.enable_accesskit();
    let mut labels = Vec::new();
    for _ in 0..3 {
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            app.background_frame(ui.ctx());
            app.frame_ui(ui);
        });
        output.textures_delta.clear();
        if let Some(tree) = output.platform_output.accesskit_update {
            for (_, node) in tree.nodes {
                if let Some(label) = node.label().or_else(|| node.value()) {
                    labels.push(label.to_owned());
                }
                assert!(
                    !node
                        .value()
                        .is_some_and(|value| value.contains("Synthetic passphrase"))
                );
            }
        }
    }
    assert!(
        labels
            .iter()
            .any(|label| label == "Passphrases do not match.")
    );
    assert!(
        !labels
            .iter()
            .any(|label| label.contains("Synthetic passphrase"))
    );
}

#[test]
fn a_starting_gallery_download_can_be_canceled_for_its_own_message() {
    let mut app = app();
    apply_flags(&mut app, Some("gallery-docs"));
    let listing = app.gallery.listing.as_mut().expect("synthetic gallery");
    let chat = listing.chat.clone();
    let message = listing.docs[0].id.clone();
    listing.docs[0].content.media_at_mut(None).unwrap().state =
        crate::model::MediaState::Downloading;
    let ctx = egui::Context::default();
    app.attach(&ctx);
    ctx.enable_accesskit();
    let draw = |app: &mut crate::app::App, events| {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(600.0, 600.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                crate::ui::gallery::show(app, ui, &chat);
            },
        );
        output.textures_delta.clear();
        output
    };
    let mut cancel = None;
    for _ in 0..3 {
        if let Some(tree) = draw(&mut app, Vec::new()).platform_output.accesskit_update {
            for (_, node) in tree.nodes {
                if node.label() == Some("Cancel")
                    && let Some(bounds) = node.bounds()
                {
                    cancel = Some(
                        egui::pos2(
                            ((bounds.x0 + bounds.x1) / 2.0) as f32,
                            ((bounds.y0 + bounds.y1) / 2.0) as f32,
                        ) / ctx.pixels_per_point(),
                    );
                }
            }
        }
    }
    let at = cancel.expect("starting downloads show an accessible Cancel action");
    draw(&mut app, vec![egui::Event::PointerMoved(at)]);
    for pressed in [true, false] {
        draw(
            &mut app,
            vec![egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
    assert!(app.actions.iter().any(|action| matches!(action, crate::model::Action::CancelDownload { chat: target, message: id, card: None } if target == &chat && id == &message)));
}

#[test]
fn normal_dialogs_paint_their_later_controls_without_an_initial_scroll() {
    fn collect(shape: &egui::Shape, clip: egui::Rect, texts: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text)
                if clip.expand(1.0).contains_rect(text.visual_bounding_rect()) =>
            {
                texts.push(text.galley.text().to_owned())
            }
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, clip, texts);
                }
            }
            _ => {}
        }
    }
    for (page, expected) in [
        ("new-contact", vec!["Save contact", "Message", "Cancel"]),
        ("poll-create", vec!["Cancel", "Send poll"]),
        ("labels-dialog", vec!["Work", "Family", "Follow up"]),
    ] {
        let mut app = app();
        apply_flags(&mut app, Some(page));
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut texts = Vec::new();
        for _ in 0..8 {
            texts.clear();
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1180.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    app.background_frame(ui.ctx());
                    app.frame_ui(ui);
                },
            );
            output.textures_delta.clear();
            for shape in output.shapes {
                collect(&shape.shape, shape.clip_rect, &mut texts);
            }
        }
        for label in expected {
            assert!(
                texts.iter().any(|text| text == label),
                "{page}: {label} must be painted inside its viewport"
            );
        }
    }
}

#[test]
fn long_action_menus_fit_short_windows_at_large_text_sizes() {
    for page in ["chat-menu", "react-picker"] {
        let mut app = app();
        apply_flags(&mut app, Some(page));
        if page == "chat-menu" {
            app.open_chat = None;
        }
        app.settings.text_scale = 1.5;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let size = egui::vec2(400.0, 440.0);
        frames(&mut app, &ctx, egui::vec2(1180.0, 780.0));
        if page == "chat-menu" {
            app.open_chat = None;
        }
        frames(&mut app, &ctx, size);
        let popup = if page == "chat-menu" {
            crate::ui::chats::chat_row_id(super::SAMPLES[0].id)
                .with("focus")
                .with("popup")
        } else {
            crate::ui::conversation::bubble_id(super::SAMPLES[0].id, "ada-link").with("popup")
        };
        let rect = if page == "react-picker" {
            ctx.data(|data| {
                data.get_temp::<egui::Rect>(
                    crate::ui::conversation::bubble_id(super::SAMPLES[0].id, "ada-link")
                        .with("menu-rect"),
                )
            })
        } else {
            ctx.memory(|memory| memory.area_rect(popup))
        }
        .unwrap_or_else(|| panic!("{page}: synthetic action menu is open"));
        assert!(
            egui::Rect::from_min_size(egui::Pos2::ZERO, size).contains_rect(rect),
            "{page}: {rect:?}"
        );
    }
}

#[test]
fn screen_privacy_hides_call_identity_and_video_without_hiding_end_call() {
    for hidden in [false, true] {
        let mut app = app();
        apply_flags(&mut app, Some("call-video"));
        app.settings.screen_privacy.enabled = hidden;
        app.settings.screen_privacy.blur_names = true;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        ctx.enable_accesskit();
        let mut labels = Vec::new();
        let mut image_meshes = 0;
        for _ in 0..3 {
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1180.0, 780.0),
                    )),
                    ..Default::default()
                },
                |ui| crate::ui::call::show(&mut app, ui.ctx()),
            );
            output.textures_delta.clear();
            image_meshes = ctx
                .tessellate(output.shapes, output.pixels_per_point)
                .iter()
                .filter(|primitive| {
                    matches!(&primitive.primitive, egui::epaint::Primitive::Mesh(mesh)
                        if mesh.texture_id != egui::TextureId::default())
                })
                .count();
            if let Some(tree) = output.platform_output.accesskit_update {
                for (_, node) in tree.nodes {
                    if let Some(label) = node.label().or_else(|| node.value()) {
                        labels.push(label.to_owned());
                    }
                }
            }
        }
        assert!(labels.iter().any(|label| label == "End call"));
        assert_eq!(labels.iter().any(|label| label == "Ada Lovelace"), !hidden);
        if hidden {
            assert_eq!(image_meshes, 0, "hidden video frames must not be painted");
        } else {
            assert!(
                image_meshes >= 2,
                "both synthetic video previews are painted"
            );
        }
    }
}

#[test]
fn device_selectors_have_programmatic_names_and_full_values() {
    let mut app = app();
    apply_flags(&mut app, Some("call-active"));
    let ctx = egui::Context::default();
    app.attach(&ctx);
    ctx.enable_accesskit();
    let mut fields = std::collections::BTreeMap::new();
    for _ in 0..3 {
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            crate::ui::call::show(&mut app, ui.ctx())
        });
        output.textures_delta.clear();
        if let Some(tree) = output.platform_output.accesskit_update {
            for (_, node) in tree.nodes {
                if node.role() == egui::accesskit::Role::ComboBox
                    && let Some(name) = node.label()
                {
                    fields.insert(name.to_owned(), node.value().map(str::to_owned));
                }
            }
        }
    }
    for name in ["Microphone", "Speaker", "Camera"] {
        assert_eq!(
            fields.get(name),
            Some(&Some("System default".to_owned())),
            "{name}"
        );
    }
}

#[test]
fn call_errors_explain_unprocessed_audio_without_pushing_end_call_below_the_window() {
    let mut app = app();
    apply_flags(&mut app, Some("call-active,large-text"));
    let call = app.call.as_mut().unwrap();
    call.audio_processing_failed = true;
    call.peer_audio = Some(crate::calls::PeerAudio::Stalled);
    call.lost_devices = vec![crate::calls::LostDevice {
        kind: crate::calls::DeviceKind::Microphone,
        name: "Synthetic microphone with a deliberately long device name".into(),
    }];
    let ctx = egui::Context::default();
    app.attach(&ctx);
    ctx.enable_accesskit();
    let _scale = crate::theme::text_scale(&ctx, app.settings.text_scale);
    let mut end_call = None;
    let mut warning = false;
    for _ in 0..3 {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(400.0, 440.0),
                )),
                ..Default::default()
            },
            |ui| crate::ui::call::show(&mut app, ui.ctx()),
        );
        output.textures_delta.clear();
        if let Some(tree) = output.platform_output.accesskit_update {
            for (_, node) in tree.nodes {
                if node.label() == Some("End call") {
                    end_call = node.bounds();
                }
                if node.label().or_else(|| node.value())
                    == Some(
                        "Microphone cleanup is unavailable. Audio is being sent without echo cancellation.",
                    )
                {
                    warning = true;
                }
            }
        }
    }
    let end_call = end_call.expect("End call remains accessible");
    assert!(end_call.y0 >= 0.0 && end_call.y1 <= 440.0, "{end_call:?}");
    assert!(warning, "unprocessed capture is disclosed");
}

#[test]
fn the_account_switcher_masks_identity_when_screen_privacy_is_enabled() {
    let mut app = app();
    apply_flags(&mut app, Some("accounts,screen-private"));
    let ctx = egui::Context::default();
    app.attach(&ctx);
    ctx.enable_accesskit();
    let mut labels = Vec::new();
    for _ in 0..3 {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1180.0, 780.0),
                )),
                ..Default::default()
            },
            |ui| {
                app.background_frame(ui.ctx());
                app.frame_ui(ui);
            },
        );
        output.textures_delta.clear();
        if let Some(tree) = output.platform_output.accesskit_update {
            for (_, node) in tree.nodes {
                if node.role() == egui::accesskit::Role::RadioButton
                    && let Some(label) = node.label()
                {
                    labels.push(label.to_owned());
                }
            }
        }
    }
    assert!(labels.iter().any(|label| label.starts_with("Account 1")));
    assert!(labels.iter().any(|label| label.starts_with("Account 2")));
    assert!(
        !labels
            .iter()
            .any(|label| label.contains("Carmine") || label.contains("1555"))
    );
}

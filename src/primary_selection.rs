//! Linux PRIMARY integration for the pinned egui version's selectable labels.
//!
//! Keep the clipboard owner alive: dropping arboard can lose the selection on X11.
use arboard::{GetExtLinux, LinuxClipboardKind, SetExtLinux};

#[derive(Default)]
pub(crate) struct PrimarySelection {
    clipboard: Option<arboard::Clipboard>,
    last_text: Option<String>,
}

impl PrimarySelection {
    pub(crate) fn new_selection(&mut self) {
        self.last_text = None;
    }

    pub(crate) fn publish(&mut self, text: String) {
        if text.is_empty() || self.last_text.as_ref() == Some(&text) {
            return;
        }
        // Headless UI fixtures must never replace the desktop's selection.
        if cfg!(test) {
            self.last_text = Some(text);
            return;
        }
        if self.clipboard.is_none() {
            self.clipboard = arboard::Clipboard::new().ok();
        }
        if let Some(clipboard) = &mut self.clipboard
            && clipboard
                .set()
                .clipboard(LinuxClipboardKind::Primary)
                .text(text.clone())
                .is_ok()
        {
            self.last_text = Some(text);
        }
    }
}

fn synthetic_id() -> egui::Id {
    egui::Id::new("primary-label-copy")
}

/// Scope a synthetic Copy event to labels only. TextEdit and application copy
/// shortcuts never see it. CopyAnnotator consumes the built-in label plugin's
/// resulting command before egui-winit can write the ordinary clipboard.
pub(crate) fn with_label_copy(ctx: &egui::Context, draw: impl FnOnce()) {
    let synthetic = ctx.input(|i| {
        (i.pointer.primary_down() || i.pointer.button_released(egui::PointerButton::Primary))
            // Wait for drag/release: the press may be clearing an old selection.
            && !i.pointer.button_pressed(egui::PointerButton::Primary)
            && !i
                .events
                .iter()
                .any(|e| matches!(e, egui::Event::Copy | egui::Event::Cut))
            && !i.any_touches()
    });
    if synthetic {
        ctx.input_mut(|i| i.events.push(egui::Event::Copy));
    }
    draw();
    if synthetic {
        // Snapshot the public plugin state so its exact pending output can be
        // identified, including when a context menu also copies in this pass.
        let snapshot = ctx
            .plugin::<egui::text_selection::LabelSelectionState>()
            .lock()
            .clone();
        ctx.data_mut(|d| d.insert_temp(synthetic_id(), snapshot));
        ctx.input_mut(|i| {
            let index = i
                .events
                .iter()
                .rposition(|e| matches!(e, egui::Event::Copy))
                .unwrap();
            i.events.remove(index);
        });
    }
}

pub(crate) fn take_label_copy(
    ctx: &egui::Context,
    output: &mut egui::FullOutput,
) -> Option<String> {
    use egui::plugin::Plugin;
    let mut snapshot = ctx
        .data_mut(|d| d.remove_temp::<egui::text_selection::LabelSelectionState>(synthetic_id()))?;
    // Flush the clone in an isolated context, leaving the real selection and
    // its painted shapes untouched. No synthetic command reaches a backend.
    let scratch = egui::Context::default();
    let extracted = scratch.run_ui(egui::RawInput::default(), |ui| snapshot.on_end_pass(ui));
    let expected = extracted.platform_output.commands.last().cloned();
    extracted.drop_without_applying_deltas();
    let Some(egui::OutputCommand::CopyText(expected)) = expected else {
        return None;
    };
    let index = output.platform_output.commands.iter().rposition(
        |command| matches!(command, egui::OutputCommand::CopyText(text) if text == &expected),
    )?;
    output.platform_output.commands.remove(index);
    Some(expected)
}

/// Returns PRIMARY text and places the editor cursor at the middle press.
pub(crate) fn middle_paste(
    ui: &egui::Ui,
    output: &egui::text_edit::TextEditOutput,
) -> Option<String> {
    middle_paste_with(ui, output, || {
        arboard::Clipboard::new()
            .ok()?
            .get()
            .clipboard(LinuxClipboardKind::Primary)
            .text()
            .ok()
    })
}

fn middle_paste_with(
    ui: &egui::Ui,
    output: &egui::text_edit::TextEditOutput,
    read: impl FnOnce() -> Option<String>,
) -> Option<String> {
    let response = &output.response;
    let pos = ui.input(|i| {
        i.pointer
            .button_pressed(egui::PointerButton::Middle)
            .then(|| i.pointer.interact_pos())
            .flatten()
    })?;
    if !response.contains_pointer() || !ui.clip_rect().contains(pos) || !ui.is_enabled() {
        return None;
    }
    let frame = ui.ctx().cumulative_frame_nr();
    let paste_id = response.id.with("primary-paste-frame");
    if ui.ctx().data(|d| d.get_temp::<u64>(paste_id)) == Some(frame) {
        return None;
    }
    let text = read()?;
    if text.is_empty() {
        return None;
    }
    let cursor = output.galley.cursor_from_pos(pos - output.galley_pos);
    let mut state = output.state.clone();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(cursor)));
    state.store(ui.ctx(), response.id);
    response.request_focus();
    ui.ctx().data_mut(|d| d.insert_temp(paste_id, frame));
    Some(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct Capture(Arc<Mutex<Vec<String>>>);
    impl egui::plugin::Plugin for Capture {
        fn debug_name(&self) -> &'static str {
            "primary-test"
        }
        fn output_hook(&mut self, ctx: &egui::Context, output: &mut egui::FullOutput) {
            if let Some(text) = take_label_copy(ctx, output) {
                self.0.lock().unwrap().push(text);
            }
        }
    }

    fn pointer(pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    #[test]
    fn mouse_selection_is_extracted_without_copying_to_clipboard() {
        let ctx = egui::Context::default();
        let selected = Arc::new(Mutex::new(Vec::new()));
        ctx.add_plugin(Capture(selected.clone()));
        let mut rect = egui::Rect::NOTHING;
        let mut frame = |events, menu_copy: bool| {
            ctx.run_ui(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ui| {
                    with_label_copy(&ctx, || {
                        rect = ui
                            .add(egui::Label::new("hello world").selectable(true))
                            .rect;
                    });
                    assert!(!ui.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Copy))));
                    if menu_copy {
                        ui.copy_text("menu copy".into());
                    }
                },
            )
        };
        frame(vec![], false).drop_without_applying_deltas();
        // Labels in a root UI start at the origin. Drag beyond the last glyph.
        let start = egui::pos2(0.1, 7.0);
        let end = egui::pos2(75.0, 7.0);
        frame(
            vec![
                egui::Event::PointerMoved(start),
                pointer(start, egui::PointerButton::Primary, true),
            ],
            false,
        )
        .drop_without_applying_deltas();
        let drag = frame(vec![egui::Event::PointerMoved(end)], true);
        assert_eq!(
            selected.lock().unwrap().last().map(String::as_str),
            Some("hello world")
        );
        assert_eq!(
            drag.platform_output.commands,
            vec![egui::OutputCommand::CopyText("menu copy".into())]
        );
        drag.drop_without_applying_deltas();
        frame(
            vec![pointer(end, egui::PointerButton::Primary, false)],
            false,
        )
        .drop_without_applying_deltas();
        // Explicit copies are left to the existing clipboard path.
        let copied = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Copy],
                ..Default::default()
            },
            |ui| {
                with_label_copy(&ctx, || {
                    ui.add(egui::Label::new("hello world").selectable(true));
                });
            },
        );
        assert!(
            copied
                .platform_output
                .commands
                .contains(&egui::OutputCommand::CopyText("hello world".into()))
        );
        copied.drop_without_applying_deltas();
    }

    #[test]
    fn an_empty_selection_does_not_consume_a_context_menu_copy() {
        let ctx = egui::Context::default();
        ctx.add_plugin(Capture(Default::default()));
        ctx.run_ui(
            egui::RawInput {
                events: vec![pointer(
                    egui::pos2(300.0, 300.0),
                    egui::PointerButton::Primary,
                    true,
                )],
                ..Default::default()
            },
            |_| {},
        )
        .drop_without_applying_deltas();
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            with_label_copy(&ctx, || {
                ui.add(egui::Label::new("hello").selectable(true));
            });
            ui.copy_text("menu copy".into());
        });
        assert_eq!(
            output.platform_output.commands,
            vec![egui::OutputCommand::CopyText("menu copy".into())]
        );
        output.drop_without_applying_deltas();
    }

    #[test]
    fn middle_paste_places_cursor_at_click_and_ignores_other_buttons() {
        let ctx = egui::Context::default();
        let mut text = "hello world".to_owned();
        let mut rect = egui::Rect::NOTHING;
        ctx.run_ui(egui::RawInput::default(), |ui| {
            rect = egui::TextEdit::singleline(&mut text).show(ui).response.rect;
        })
        .drop_without_applying_deltas();
        let pos = rect.left_center() + egui::vec2(3.0, 0.0);
        ctx.run_ui(
            egui::RawInput {
                events: vec![
                    egui::Event::PointerMoved(pos),
                    pointer(pos, egui::PointerButton::Middle, true),
                ],
                ..Default::default()
            },
            |ui| {
                let output = egui::TextEdit::singleline(&mut text).show(ui);
                assert_eq!(
                    middle_paste_with(ui, &output, || Some("external 🎉".into())),
                    Some("external 🎉".into())
                );
                assert_eq!(
                    middle_paste_with(ui, &output, || panic!("already pasted this frame")),
                    None
                );
                let state = egui::TextEdit::load_state(ui.ctx(), output.response.id).unwrap();
                assert_eq!(state.cursor.char_range().unwrap().primary.index.0, 0);
            },
        )
        .drop_without_applying_deltas();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            let output = egui::TextEdit::singleline(&mut text).show(ui);
            assert_eq!(
                middle_paste_with(ui, &output, || panic!("no middle press")),
                None
            );
        })
        .drop_without_applying_deltas();
    }
}

#[cfg(test)]
mod transcript_tests {
    use super::*;

    #[test]
    fn primary_uses_transcript_headers_and_restores_emoji() {
        let ctx = egui::Context::default();
        let first = format!("before {}", crate::emoji::PLACEHOLDER);
        ctx.add_plugin(crate::transcript::CopyAnnotator {
            rows: std::sync::Arc::new(std::sync::Mutex::new(vec![
                crate::transcript::Row {
                    header: "[9:00, 1/2/2026] Ada: ".into(),
                    body: first.clone(),
                    placements: vec!["🎉".into()],
                    ..Default::default()
                },
                crate::transcript::Row {
                    header: "[9:01, 1/2/2026] Lin: ".into(),
                    body: "after".into(),
                    ..Default::default()
                },
            ])),
            primary: Default::default(),
        });
        let first_rect = std::cell::Cell::new(egui::Rect::NOTHING);
        let last_rect = std::cell::Cell::new(egui::Rect::NOTHING);
        let frame = |events| {
            ctx.run_ui(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ui| {
                    with_label_copy(&ctx, || {
                        first_rect.set(ui.add(egui::Label::new(&first).selectable(true)).rect);
                        last_rect.set(ui.add(egui::Label::new("after").selectable(true)).rect);
                    });
                },
            )
        };
        frame(vec![]).drop_without_applying_deltas();
        let start = first_rect.get().left_center() + egui::vec2(0.1, 0.0);
        let end = last_rect.get().right_center();
        frame(vec![
            egui::Event::PointerMoved(start),
            egui::Event::PointerButton {
                pos: start,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ])
        .drop_without_applying_deltas();
        let output = frame(vec![egui::Event::PointerMoved(end)]);
        assert!(output.platform_output.commands.is_empty());
        output.drop_without_applying_deltas();
        assert_eq!(
            ctx.plugin::<crate::transcript::CopyAnnotator>()
                .lock()
                .primary
                .last_text
                .as_deref(),
            Some("[9:00, 1/2/2026] Ada: before 🎉\n[9:01, 1/2/2026] Lin: after")
        );
    }
}

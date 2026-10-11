//! Native AccessKit semantics and keyboard movement without changing layout.

use egui::{Context, Id, Key, Response, Ui};

pub(crate) fn list(ui: &Ui, label: &str, count: usize) {
    ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::List);
        node.set_label(label);
        node.set_size_of_set(count);
    });
}

pub(crate) fn item(response: &Response, position: usize, count: usize) {
    response
        .ctx
        .data_mut(|data| data.insert_temp(response.id.with("accessible-list-item"), true));
    response.ctx.accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListItem);
        node.set_position_in_set(position + 1);
        node.set_size_of_set(count);
    });
    let pending = response.ctx.data(|data| data.get_temp::<Id>(pending_id()));
    if pending == Some(response.id) {
        response.request_focus();
        response
            .ctx
            .data_mut(|data| data.remove::<Id>(pending_id()));
    }
}

pub(crate) fn list_focused(ctx: &Context) -> bool {
    ctx.memory(|memory| memory.focused()).is_some_and(|id| {
        ctx.data(|data| data.get_temp::<bool>(id.with("accessible-list-item")))
            .unwrap_or(false)
    })
}

pub(crate) fn toolbar(ui: &Ui, label: &str) {
    ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label(label);
    });
}

pub(crate) fn suggestions(ui: &Ui, label: &str, count: usize) {
    ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::ListBox);
        node.set_label(label);
        node.set_size_of_set(count);
    });
}

pub(crate) fn suggestion(
    response: &Response,
    field: Id,
    label: &str,
    position: usize,
    count: usize,
    selected: bool,
) {
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            response.enabled(),
            selected,
            label,
        )
    });
    response.ctx.accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_position_in_set(position + 1);
        node.set_size_of_set(count);
    });
    if selected {
        response
            .ctx
            .data_mut(|data| data.insert_temp(field.with("suggestion-selected"), response.id));
    }
}

#[derive(Clone, Default)]
struct Results {
    ids: Vec<Id>,
    total: usize,
    active: bool,
}

fn results_id() -> Id {
    Id::new("accessible-sidebar-results")
}

pub(crate) fn begin_results(ui: &Ui, label: &str, total: usize) {
    let previous = ui
        .ctx()
        .data(|data| data.get_temp::<Results>(results_id()))
        .unwrap_or_default();
    if ui.is_enabled() && !egui::Popup::is_any_open(ui.ctx()) {
        navigate(ui.ctx(), &previous.ids);
    }
    list(ui, label, total);
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            results_id(),
            Results {
                ids: Vec::new(),
                total,
                active: true,
            },
        )
    });
}

pub(crate) fn result_item(response: &Response) {
    let position = response.ctx.data_mut(|data| {
        let results = data.get_temp_mut_or_default::<Results>(results_id());
        if !results.active {
            return None;
        }
        let position = results.ids.len();
        results.ids.push(response.id);
        Some((position, results.total))
    });
    if let Some((position, total)) = position {
        item(response, position, total);
    }
}

pub(crate) fn end_results(ctx: &Context) {
    ctx.data_mut(|data| data.get_temp_mut_or_default::<Results>(results_id()).active = false);
}

fn pending_id() -> Id {
    Id::new("accessible-list-pending-focus")
}

pub(crate) fn request_item(ctx: &Context, id: Id) {
    ctx.data_mut(|data| data.insert_temp(pending_id(), id));
    ctx.data_mut(|data| data.insert_temp(crate::theme::keyboard_focus_id(), true));
    ctx.memory_mut(|memory| {
        memory.move_focus(egui::FocusDirection::None);
        memory.request_focus(id);
    });
}

/// Computes movement only when an actual list item owns focus. Text fields,
/// media subcontrols, menus and overlays retain their existing shortcuts.
pub(crate) fn navigate(ctx: &Context, items: &[Id]) -> Option<usize> {
    let focused = ctx.memory(|memory| memory.focused())?;
    let current = items.iter().position(|id| *id == focused)?;
    let key = ctx.input_mut(|input| {
        [
            Key::Home,
            Key::End,
            Key::PageUp,
            Key::PageDown,
            Key::ArrowUp,
            Key::ArrowDown,
        ]
        .into_iter()
        .find(|key| super::keys::take_plain(input, *key))
    })?;
    let target = movement(current, items.len(), key)?;
    request_item(ctx, items[target]);
    Some(target)
}

fn movement(current: usize, count: usize, key: Key) -> Option<usize> {
    let last = count.checked_sub(1)?;
    Some(match key {
        Key::Home => 0,
        Key::End => last,
        Key::PageUp => current.saturating_sub(10),
        Key::PageDown => current.saturating_add(10).min(last),
        Key::ArrowUp => current.saturating_sub(1),
        Key::ArrowDown => current.saturating_add(1).min(last),
        _ => return None,
    })
}

pub(crate) fn live(ui: &mut Ui, announcements: &mut Vec<String>) {
    let id = Id::new("accessible-status-announcement");
    let previous = ui.ctx().data(|data| data.get_temp::<String>(id));
    let label = if previous.is_some() {
        ui.ctx().data_mut(|data| data.remove::<String>(id));
        if !announcements.is_empty() {
            ui.ctx().request_repaint();
        }
        String::new()
    } else if announcements.is_empty() {
        String::new()
    } else {
        let label = std::mem::take(announcements).join(". ");
        ui.ctx()
            .data_mut(|data| data.insert_temp(id, label.clone()));
        ui.ctx().request_repaint();
        label
    };
    // A zero-sized hover widget supplies a real parent without adding a tab
    // stop or changing painted geometry. The live label is never logged.
    let response = ui.interact(
        egui::Rect::from_min_size(ui.min_rect().min, egui::Vec2::ZERO),
        id,
        egui::Sense::hover(),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_live(egui::accesskit::Live::Polite);
        node.set_live_atomic();
        node.set_label(label);
    });
}

pub(crate) fn clear_live(ctx: &Context) {
    ctx.data_mut(|data| {
        data.remove::<String>(Id::new("accessible-status-announcement"));
        data.remove::<Id>(pending_id());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_movement_clamps_and_reaches_both_ends() {
        assert_eq!(movement(3, 19, Key::Home), Some(0));
        assert_eq!(movement(3, 19, Key::End), Some(18));
        assert_eq!(movement(3, 19, Key::PageUp), Some(0));
        assert_eq!(movement(3, 19, Key::PageDown), Some(13));
        assert_eq!(movement(18, 19, Key::ArrowDown), Some(18));
        assert_eq!(movement(0, 0, Key::End), None);
    }

    #[test]
    fn accesskit_exposes_real_lists_positions_and_counts() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            list(ui, "Synthetic messages", 19);
            let response = ui.button("Synthetic message");
            item(&response, 2, 19);
        });
        output.textures_delta.clear();
        let update = output.platform_output.accesskit_update.unwrap();
        assert!(
            update
                .nodes
                .iter()
                .any(|(_, node)| node.role() == egui::accesskit::Role::List
                    && node.size_of_set() == Some(19))
        );
        assert!(
            update
                .nodes
                .iter()
                .any(|(_, node)| node.role() == egui::accesskit::Role::ListItem
                    && node.position_in_set() == Some(3)
                    && node.size_of_set() == Some(19))
        );
    }
}

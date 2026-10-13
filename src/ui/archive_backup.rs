//! Password entry stays transient and is erased when the operation starts.

use zeroize::Zeroizing;

use crate::i18n::gettext;
use crate::{app::App, archive::backup::BackupPassphrase, model::Action, theme};

#[derive(Default)]
pub struct BackupDraft {
    pub passphrase: Zeroizing<String>,
    pub confirmation: Zeroizing<String>,
    pub merge: bool,
}

pub fn show(app: &mut App, ui: &mut egui::Ui, restore: bool) {
    let palette = app.palette;
    let locale = app.locale;
    ui.heading(if restore {
        gettext(locale, "Restore an account backup")
    } else {
        gettext(locale, "Back up this account")
    });
    ui.add_space(10.0);
    if restore {
        ui.checkbox(
            &mut app.backup_draft.merge,
            gettext(locale, "Merge messages into this account"),
        );
        if app.backup_draft.merge {
            ui.label(gettext(locale, "A matching account identity and an exact preview are required. Current messages and work queues stay intact. Conflicting edits and imported pending work are retained separately without sending."));
        }
    }
    ui.label(if restore && app.backup_draft.merge {
        gettext(locale, "Merge historical messages and files. The current linked-device identity stays in use.")
    } else if restore {
        gettext(locale, "Restore into a separate account. Existing accounts stay intact. Full backups include the linked-device identity; archive-only backups open offline.")
    } else {
        gettext(locale, "Save this account, its linked-device identity and downloaded files in an encrypted backup. The connection pauses while saving. Keep the passphrase safe: it cannot be recovered.")
    });
    ui.add_space(10.0);
    ui.label(gettext(
        locale,
        "Backup passphrase (at least 12 characters)",
    ));
    ui.add(
        egui::TextEdit::singleline(&mut *app.backup_draft.passphrase)
            .password(true)
            .desired_width(f32::INFINITY),
    );
    if !restore {
        ui.label(gettext(locale, "Repeat passphrase"));
        ui.add(
            egui::TextEdit::singleline(&mut *app.backup_draft.confirmation)
                .password(true)
                .desired_width(f32::INFINITY),
        );
    }
    let long_enough = app.backup_draft.passphrase.chars().count() >= 12;
    let matches = restore || app.backup_draft.passphrase == app.backup_draft.confirmation;
    if !app.backup_draft.passphrase.is_empty() && !long_enough {
        ui.colored_label(
            palette.secondary,
            gettext(locale, "Backup passphrase (at least 12 characters)"),
        );
    } else if !app.backup_draft.confirmation.is_empty() && !matches {
        ui.colored_label(palette.danger, gettext(locale, "Passphrases do not match."));
    }
    let ready =
        long_enough && (restore || app.backup_draft.passphrase == app.backup_draft.confirmation);
    ui.add_space(12.0);
    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled_ui(ready, |ui| {
                theme::pill_button(
                    ui,
                    &palette,
                    &if restore {
                        gettext(locale, "Choose backup…")
                    } else {
                        gettext(locale, "Save backup…")
                    },
                    true,
                )
            })
            .inner
            .clicked()
        {
            let passphrase =
                BackupPassphrase::new(std::mem::take(&mut app.backup_draft.passphrase));
            let merge = app.backup_draft.merge;
            app.backup_draft = BackupDraft::default();
            app.actions.push(if restore && merge {
                Action::PrepareBackupMerge(passphrase)
            } else if restore {
                Action::RestoreArchiveBackup(passphrase)
            } else {
                Action::ExportArchiveBackup(passphrase)
            });
            app.actions.push(Action::CloseDialog);
        }
        if theme::pill_button(ui, &palette, &gettext(locale, "Cancel"), false).clicked() {
            app.backup_draft = BackupDraft::default();
            app.actions.push(Action::CloseDialog);
        }
    });
}

pub fn confirm_merge(app: &mut App, ui: &mut egui::Ui) {
    let Some(preview) = app.backup_merge_preview.clone() else {
        return;
    };
    ui.heading(gettext(app.locale, "Merge this backup?"));
    ui.label(gettext(app.locale, "{added} messages added, {edited} edited snapshots updated. {removed} current messages removed by confirmed backup deletions. {protected} deleted or pending-deletion messages are protected. {conflicts} conflicting alternatives and {pending} imported pending records are retained separately.")
        .replace("{added}",&preview.added.to_string()).replace("{edited}",&preview.edited.to_string())
        .replace("{removed}",&preview.removed.to_string()).replace("{protected}",&preview.protected.to_string())
        .replace("{conflicts}",&preview.conflicts.to_string()).replace("{pending}",&preview.imported_pending.to_string()));
    ui.label(gettext(app.locale, "Session credentials and current pending work are preserved. Confirmed backup deletions listed above are applied. Imported pending sends never run. Different edited snapshots without a revision timestamp keep the current version and retain the imported alternative in the encrypted archive."));
    ui.label(gettext(
        app.locale,
        "If the current archive changes before confirmation, prepare a new preview.",
    ));
    ui.horizontal_wrapped(|ui| {
        if theme::pill_button(ui, &app.palette, &gettext(app.locale, "Merge"), true).clicked() {
            app.actions.push(Action::ConfirmBackupMerge);
            app.actions.push(Action::CloseDialog);
        }
        if theme::pill_button(ui, &app.palette, &gettext(app.locale, "Cancel"), false).clicked() {
            app.actions.push(Action::CancelBackupMerge);
            app.actions.push(Action::CloseDialog);
        }
    });
}

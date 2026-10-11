//! Password entry stays transient and is erased when the operation starts.

use zeroize::Zeroizing;

use crate::i18n::gettext;
use crate::{app::App, archive::backup::BackupPassphrase, model::Action, theme};

#[derive(Default)]
pub struct BackupDraft {
    pub passphrase: Zeroizing<String>,
    pub confirmation: Zeroizing<String>,
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
    ui.label(if restore {
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
    let ready = app.backup_draft.passphrase.chars().count() >= 12
        && (restore || app.backup_draft.passphrase == app.backup_draft.confirmation);
    ui.add_space(12.0);
    ui.horizontal(|ui| {
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
            app.backup_draft = BackupDraft::default();
            app.actions.push(if restore {
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

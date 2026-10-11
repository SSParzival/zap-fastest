//! Process-owned copies of user-supplied colour bitmap emoji fonts.
//!
//! Import happens off the frame thread. No fonts are downloaded or redistributed.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use skrifa::MetadataProvider;

use crate::paths::AppDirs;

pub type PickResult = Option<Result<PathBuf, String>>;

pub struct PendingPick {
    pub request: u64,
    pub receiver: std::sync::mpsc::Receiver<PickResult>,
}

static LATEST: AtomicU64 = AtomicU64::new(0);
static DIRECTORY: Mutex<()> = Mutex::new(());
// Apple's system colour font is about 190 MB. Bound allocations while allowing it.
const MAX_BYTES: u64 = 256 * 1024 * 1024;

fn directory(dirs: &AppDirs) -> PathBuf {
    dirs.state.join("emoji-fonts")
}

pub fn claim(request: u64) {
    LATEST.store(request, Ordering::SeqCst);
}

pub fn pick(dirs: AppDirs, request: u64, ctx: egui::Context) -> std::io::Result<PendingPick> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("emoji-font-picker".into())
        .spawn(move || {
            let mut dialog = rfd::FileDialog::new()
                .set_title("Choose a colour emoji font you are allowed to use")
                .add_filter("Colour bitmap emoji fonts", &crate::custom_font::EXTENSIONS);
            if let Some(directory) = crate::custom_font::system_directory() {
                dialog = dialog.set_directory(directory);
            }
            let result = dialog
                .pick_file()
                .and_then(|path| import(&path, &dirs, request));
            let _ = sender.send(result);
            ctx.request_repaint();
        })?;
    Ok(PendingPick { request, receiver })
}

/// Rejects ordinary interface fonts, COLR/SVG-only faces and corrupt files.
/// The supported API selects only the first face of a font collection.
fn validate(bytes: &[u8]) -> Result<(), String> {
    use skrifa::bitmap::{BitmapData, BitmapStrikes};
    use skrifa::instance::Size;

    let font = skrifa::FontRef::from_index(bytes, 0).map_err(|_| "Not a font file")?;
    let strikes = BitmapStrikes::new(&font);
    if strikes.is_empty() {
        return Err(
            "Choose a CBDT or sbix colour bitmap font. COLR and SVG fonts are not supported."
                .into(),
        );
    }
    let grinning = font.charmap().map('😀').ok_or("Font has no emoji")?;
    if !strikes
        .glyph_for_size(Size::new(64.0), grinning)
        .is_some_and(|glyph| !matches!(glyph.data, BitmapData::Mask(_)))
    {
        return Err("Font has no colour emoji bitmaps".into());
    }
    Ok(())
}

fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > limit {
        return Err("Emoji font exceeds the supported size limit (256 MiB)".into());
    }
    Ok(bytes)
}

/// Validates before retaining any bytes for the process-lifetime renderer.
pub fn load(path: &Path) -> Result<Vec<u8>, String> {
    let bytes = read_bounded(path, MAX_BYTES)?;
    validate(&bytes)?;
    Ok(bytes)
}

fn import(source: &Path, dirs: &AppDirs, request: u64) -> PickResult {
    let bytes = match load(source) {
        Ok(bytes) => bytes,
        Err(error) => return Some(Err(error)),
    };
    let Some(name) = source.file_name() else {
        return Some(Err("Not a file".into()));
    };
    let _guard = DIRECTORY.lock().unwrap_or_else(|error| error.into_inner());
    if LATEST.load(Ordering::SeqCst) != request {
        return None;
    }
    let root = directory(dirs);
    let staged = (|| {
        std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        let mut number = 0u32;
        let folder = loop {
            let folder = root.join(number.to_string());
            match std::fs::create_dir(&folder) {
                Ok(()) => break folder,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    number = number.checked_add(1).ok_or("Too many imported fonts")?;
                }
                Err(error) => return Err(error.to_string()),
            }
        };
        let target = folder.join(name);
        if let Err(error) = std::fs::write(&target, &bytes) {
            let _ = std::fs::remove_dir_all(&folder);
            return Err(error.to_string());
        }
        Ok(target)
    })();
    if LATEST.load(Ordering::SeqCst) != request {
        if let Ok(target) = &staged {
            let _ = std::fs::remove_dir_all(target.parent().expect("staged folder"));
        }
        return None;
    }
    Some(staged)
}

pub fn cleanup(dirs: AppDirs, keep: Option<PathBuf>, request: u64) -> std::io::Result<()> {
    std::thread::Builder::new()
        .name("emoji-font-cleanup".into())
        .spawn(move || prune(&dirs, keep.as_deref(), request))
        .map(|_| ())
}

fn prune(dirs: &AppDirs, keep: Option<&Path>, request: u64) {
    let _guard = DIRECTORY.lock().unwrap_or_else(|error| error.into_inner());
    if LATEST.load(Ordering::SeqCst) != request {
        return;
    }
    let root = directory(dirs);
    let keep = match keep {
        Some(path) => {
            let Some(folder) = path
                .parent()
                .filter(|folder| folder.parent() == Some(root.as_path()))
            else {
                return;
            };
            Some(folder)
        }
        None => None,
    };
    let Ok(entries) = std::fs::read_dir(&root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if keep != Some(path.as_path()) {
            let _ = std::fs::remove_dir_all(&path).or_else(|_| std::fs::remove_file(&path));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const COLOUR: &[u8] = include_bytes!("../assets/fonts/NotoColorEmoji.ttf");

    #[test]
    fn accepts_colour_bitmaps_and_rejects_non_emoji_fonts() {
        assert!(validate(COLOUR).is_ok());
        assert!(validate(b"not a font").is_err());
        let fonts = egui::FontDefinitions::default();
        assert!(validate(&fonts.font_data["Ubuntu-Light"].font).is_err());
    }

    #[test]
    fn file_reads_are_bounded_before_validation() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("font.ttf");
        std::fs::write(&path, b"123456789").unwrap();
        assert!(read_bounded(&path, 8).is_err());
        assert_eq!(read_bounded(&path, 9).unwrap(), b"123456789");
    }

    #[test]
    fn stale_and_failed_imports_preserve_selected_copy_and_reset_cancels_cleanup() {
        let root = tempfile::tempdir().unwrap();
        let dirs = AppDirs::under(&root.path().join("app"));
        let source = root.path().join("emoji.ttf");
        std::fs::write(&source, COLOUR).unwrap();
        claim(1);
        let first = import(&source, &dirs, 1).unwrap().unwrap();
        assert_eq!(load(&first).unwrap(), COLOUR);
        claim(2);
        let second = import(&source, &dirs, 2).unwrap().unwrap();
        assert_ne!(first, second);
        claim(3);
        assert!(import(&source, &dirs, 2).is_none());
        prune(&dirs, None, 2);
        assert!(first.exists() && second.exists());
        let invalid = root.path().join("invalid.ttf");
        std::fs::write(&invalid, b"invalid font").unwrap();
        assert!(import(&invalid, &dirs, 3).unwrap().is_err());
        prune(&dirs, Some(&second), 3);
        assert!(!first.exists());
        assert!(second.exists());
        // A settings path outside the owned font directory cannot cause pruning.
        prune(&dirs, Some(&source), 3);
        assert!(source.exists() && second.exists());
        claim(4);
        prune(&dirs, None, 4);
        assert!(!second.exists());
        assert!(source.exists());
    }
}

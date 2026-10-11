//! Account-owned display overrides. Never changes downloaded photos or uploads to WhatsApp.
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

static WRITES: Mutex<()> = Mutex::new(());
const LIMIT: u64 = 32 * 1024 * 1024;

pub fn key(id: &str) -> String {
    Sha256::digest(id.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn folder(state: &Path) -> PathBuf {
    state.join("avatar-overrides")
}
fn regular(metadata: &fs::Metadata, directory: bool) -> bool {
    if metadata.file_type().is_symlink()
        || (directory && !metadata.is_dir())
        || (!directory && !metadata.is_file())
    {
        return false;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x400 != 0 {
            return false;
        }
    }
    true
}
fn validate_folder(state: &Path) -> Result<(), String> {
    match fs::symlink_metadata(folder(state)) {
        Ok(metadata) if regular(&metadata, true) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        _ => Err("Local picture folder is unavailable".into()),
    }
}
fn safe_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}
fn safe_name(value: &str) -> bool {
    value
        .strip_suffix(".png")
        .and_then(|value| value.split_once('-'))
        .is_some_and(|(a, b)| safe_hex(a) && safe_hex(b))
}

fn read_index(state: &Path) -> Result<HashMap<String, String>, String> {
    validate_folder(state)?;
    let path = folder(state).join("index.json");
    match fs::symlink_metadata(&path) {
        Ok(metadata) if regular(&metadata, false) && metadata.len() <= 16 * 1024 * 1024 => (),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(HashMap::new()),
        _ => return Err("Local picture preferences are unavailable".into()),
    }
    let contents = match fs::read(&path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(_) => return Err("Could not read local picture preferences".into()),
    };
    if contents.len() > 16 * 1024 * 1024 {
        return Err("Local picture preferences are too large".into());
    }
    let index: HashMap<String, String> = serde_json::from_slice(&contents)
        .map_err(|_| "Local picture preferences are invalid".to_owned())?;
    if index
        .iter()
        .any(|(id, name)| !safe_hex(id) || !safe_name(name) || !name.starts_with(&format!("{id}-")))
    {
        return Err("Local picture preferences contain an invalid path".into());
    }
    Ok(index)
}

pub fn load(state: &Path) -> Result<HashMap<String, PathBuf>, String> {
    Ok(read_index(state)?
        .into_iter()
        .filter_map(|(id, name)| {
            let path = folder(state).join(name);
            fs::symlink_metadata(&path)
                .ok()
                .filter(|metadata| regular(metadata, false))
                .map(|_| (id, path))
        })
        .collect())
}

fn save_index(state: &Path, index: &HashMap<String, String>) -> Result<(), String> {
    let directory = folder(state);
    fs::create_dir_all(&directory)
        .map_err(|_| "Could not create the local picture folder".to_owned())?;
    let mut temporary = tempfile::NamedTempFile::new_in(&directory)
        .map_err(|_| "Could not save local picture preferences".to_owned())?;
    serde_json::to_writer(&mut temporary, index)
        .map_err(|_| "Could not save local picture preferences".to_owned())?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|_| "Could not save local picture preferences".to_owned())?;
    crate::archive::backup::durable_move(temporary.path(), &directory.join("index.json"), true)
        .map_err(|_| "Could not publish local picture preferences".to_owned())
}

pub fn import(state: &Path, id: &str, source: &Path) -> Result<PathBuf, String> {
    let metadata = fs::metadata(source).map_err(|_| "Could not open this image".to_owned())?;
    if metadata.len() > LIMIT {
        return Err("Choose an image smaller than 32 MiB".into());
    }
    let mut reader = image::ImageReader::open(source)
        .and_then(image::ImageReader::with_guessed_format)
        .map_err(|_| "Could not open this image".to_owned())?;
    let mut limits = image::Limits::default();
    limits.max_alloc = Some(64 * 1024 * 1024);
    limits.max_image_width = Some(16384);
    limits.max_image_height = Some(16384);
    reader.limits(limits);
    let image = reader
        .decode()
        .map_err(|_| "Could not decode this image".to_owned())?;
    let image = image.thumbnail(1024, 1024);
    let mut contents = std::io::Cursor::new(Vec::new());
    image
        .write_to(&mut contents, image::ImageFormat::Png)
        .map_err(|_| "Could not encode this image".to_owned())?;
    let contents = contents.into_inner();
    let _guard = WRITES
        .lock()
        .map_err(|_| "Local picture preferences are unavailable".to_owned())?;
    let mut index = read_index(state)?;
    let id = key(id);
    let hash: String = Sha256::digest(&contents)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let name = format!("{id}-{hash}.png");
    let directory = folder(state);
    fs::create_dir_all(&directory)
        .map_err(|_| "Could not create the local picture folder".to_owned())?;
    let target = directory.join(&name);
    if target.exists() {
        let metadata =
            fs::symlink_metadata(&target).map_err(|_| "Local picture is unavailable".to_owned())?;
        if !regular(&metadata, false)
            || fs::read(&target).map_err(|_| "Could not read local picture".to_owned())? != contents
        {
            return Err("Local picture ownership changed".into());
        }
    } else {
        let mut temporary = tempfile::NamedTempFile::new_in(&directory)
            .map_err(|_| "Could not save this image".to_owned())?;
        temporary
            .write_all(&contents)
            .and_then(|()| temporary.as_file().sync_all())
            .map_err(|_| "Could not save this image".to_owned())?;
        crate::archive::backup::durable_move(temporary.path(), &target, false)
            .map_err(|_| "Could not publish this image".to_owned())?;
    }
    let previous = index.insert(id, name.clone());
    save_index(state, &index)?;
    if let Some(previous) = previous.filter(|previous| previous != &name) {
        let _ = fs::remove_file(directory.join(previous));
    }
    Ok(target)
}

pub fn reset(state: &Path, id: &str) -> Result<(), String> {
    let _guard = WRITES
        .lock()
        .map_err(|_| "Local picture preferences are unavailable".to_owned())?;
    let mut index = read_index(state)?;
    if let Some(previous) = index.remove(&key(id)) {
        save_index(state, &index)?;
        fs::remove_file(folder(state).join(previous))
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(error)
                }
            })
            .map_err(|_| "Could not remove the copied local picture".to_owned())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imports_are_account_scoped_and_reset_preserves_downloaded_and_source_images() {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source.png");
        image::RgbaImage::from_pixel(12, 8, image::Rgba([10, 20, 30, 255]))
            .save(&source)
            .unwrap();
        let a = root.path().join("a");
        let b = root.path().join("b");
        fs::create_dir_all(a.join("avatars")).unwrap();
        fs::write(a.join("avatars/fetched.jpg"), b"fetched fixture").unwrap();
        let original = fs::read(&source).unwrap();
        let target = import(&a, "fixture-contact", &source).unwrap();
        assert_eq!(
            load(&a).unwrap().get(&key("fixture-contact")),
            Some(&target)
        );
        assert!(load(&b).unwrap().is_empty());
        assert_eq!(import(&a, "fixture-contact", &source).unwrap(), target);
        reset(&a, "fixture-contact").unwrap();
        assert!(load(&a).unwrap().is_empty());
        assert!(!target.exists());
        assert_eq!(fs::read(source).unwrap(), original);
        assert_eq!(
            fs::read(a.join("avatars/fetched.jpg")).unwrap(),
            b"fetched fixture"
        );
    }
    #[test]
    fn invalid_index_cannot_escape_managed_directory() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(folder(root.path())).unwrap();
        fs::write(
            folder(root.path()).join("index.json"),
            format!(r#"{{"{}":"../unrelated.png"}}"#, key("fixture")),
        )
        .unwrap();
        assert!(reset(root.path(), "fixture").is_err());
        assert!(load(root.path()).is_err());
    }
}

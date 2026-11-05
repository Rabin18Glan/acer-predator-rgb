use crate::model::Settings;
use std::{env, fs, io, path::PathBuf};

pub fn root() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            return Ok(path);
        }
    }
    env::var_os("HOME")
        .map(|p| PathBuf::from(p).join(".config"))
        .ok_or_else(|| "HOME is unavailable".into())
}
pub fn path() -> Result<PathBuf, String> {
    Ok(root()?.join("acer-predator-rgb/settings.json"))
}

pub fn load() -> Result<(Settings, String), String> {
    let own = path()?;
    let (path, source) = match fs::read_to_string(&own) {
        Ok(text) => {
            return decode(&text)
                .map(|s| (s, "Loaded saved Rust settings. Edits apply live.".into()));
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => (
            root()?.join("predator-rgb/settings.json"),
            "Imported Python settings (read-only). Edits apply live.",
        ),
        Err(e) => return Err(format!("Cannot read {}: {e}", own.display())),
    };
    match fs::read_to_string(&path) {
        Ok(text) => decode(&text).map(|s| (s, source.into())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok((
            Settings::default(),
            "Default settings. Nothing sent to hardware.".into(),
        )),
        Err(e) => Err(format!("Cannot read {}: {e}", path.display())),
    }
}
fn decode(text: &str) -> Result<Settings, String> {
    let s: Settings = serde_json::from_str(text).map_err(|e| format!("Invalid settings: {e}"))?;
    s.validate()?;
    Ok(s)
}
pub fn save(s: &Settings) -> Result<PathBuf, String> {
    s.validate()?;
    let path = path()?;
    let parent = path.parent().ok_or("Invalid config path")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let tmp = parent.join(format!("settings-{}.tmp", std::process::id()));
    let text = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    fs::write(&tmp, text).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(path)
}

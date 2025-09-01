use std::env::{home_dir, var_os};
use std::path::PathBuf;

#[cfg(all(unix, not(target_os = "macos")))]
pub fn app_data() -> PathBuf {
    var_os("XDG_DATA_HOME")
        .map_or_else(|| home().join(".local/share"), PathBuf::from)
}

#[cfg(target_os = "macos")]
pub fn app_data() -> PathBuf {
    var_os("XDG_DATA_HOME")
        .map_or_else(|| home().join("Library/Application Support"), PathBuf::from)
}

#[cfg(windows)]
pub fn app_data() -> PathBuf {
    var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap()
}

fn home() -> PathBuf {
    home_dir().unwrap()
}

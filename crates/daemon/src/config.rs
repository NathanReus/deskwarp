use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub gui_path: Option<PathBuf>,
    // later: profiles, schedules, etc.
}

impl Config {
    pub fn load(config_path: &std::path::Path) -> Self {
        match std::fs::read_to_string(config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let gui_path = default_gui_path();
        Config { gui_path }
    }
}

fn default_gui_path() -> Option<PathBuf> {
    let daemon_exe = std::env::current_exe().ok()?;
    let daemon_dir = daemon_exe.parent()?;

    // Choose the GUI executable name with the correct platform extension
    #[cfg(target_os = "windows")]
    let gui_name = "deskwarp-gui.exe";
    #[cfg(not(target_os = "windows"))]
    let gui_name = "deskwarp-gui";

    let candidate = daemon_dir.join(gui_name);
    if candidate.exists() {
        Some(candidate)
    } else {
        // Fallback: just return the expected path so the user can adjust it if needed
        // (the daemon will fail to launch it, but at least the config file will show the intent)
        Some(candidate)
    }
}

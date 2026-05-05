use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::WallpaperManager;
use crate::error::WallpaperError;
use crate::style::WallpaperStyle;

pub struct MacWallpaperManager;

impl WallpaperManager for MacWallpaperManager {
    fn new() -> Result<Self, WallpaperError> {
        Err(WallpaperError::Unsupported)
    }

    fn set(
        &self,
        _path: &Path,
        _style: WallpaperStyle,
        _monitor: Option<&str>,
    ) -> Result<(), WallpaperError> {
        Err(WallpaperError::Unsupported)
    }

    fn get(&self, _monitor: Option<&str>) -> Result<Option<PathBuf>, WallpaperError> {
        Err(WallpaperError::Unsupported)
    }

    fn set_style(&self, style: WallpaperStyle) -> Result<(), WallpaperError> {
        Err(WallpaperError::Unsupported)
    }

    fn get_style(&self) -> Result<WallpaperStyle, WallpaperError> {
        Err(WallpaperError::Unsupported)
    }

    fn list_monitors(&self) -> Result<Vec<OsString>, WallpaperError> {
        Err(WallpaperError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WallpaperStyle;
    use std::path::Path;

    #[test]
    fn test_mac_stub_new() {
        let result = MacWallpaperManager::new();
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }

    #[test]
    fn test_mac_stub_set() {
        let manager = MacWallpaperManager;
        let result = manager.set(Path::new("test.jpg"), WallpaperStyle::Fill, None);
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }

    #[test]
    fn test_mac_stub_get() {
        let manager = MacWallpaperManager;
        let result = manager.get(None);
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }

    #[test]
    fn test_mac_stub_set_style() {
        let manager = MacWallpaperManager;
        let result = manager.set_style(WallpaperStyle::Fill);
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }

    #[test]
    fn test_mac_stub_get_style() {
        let manager = MacWallpaperManager;
        let result = manager.get_style();
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }

    #[test]
    fn test_mac_stub_list_monitors() {
        let manager = MacWallpaperManager;
        let result = manager.list_monitors();
        assert!(matches!(result, Err(WallpaperError::Unsupported)));
    }
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::LinuxWallpaperManager as WallpaperManagerImpl;

#[cfg(target_os = "macos")]
pub use self::macos::MacWallpaperManager as WallpaperManagerImpl;

#[cfg(target_os = "windows")]
pub use self::windows::WindowsWallpaperManager as WallpaperManagerImpl;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::{WallpaperError, WallpaperStyle};

/// A trait for managing the wallpaper on the system.
///
/// This trait is implemented by platform-specific wallpaper managers.
pub trait WallpaperManager {
    fn new() -> Result<Self, WallpaperError>
    where
        Self: Sized;

    fn set(
        &self,
        path: &Path,
        style: WallpaperStyle,
        monitor: Option<&OsStr>,
    ) -> Result<(), WallpaperError>;

    fn get(&self, monitor: Option<&OsStr>) -> Result<Option<PathBuf>, WallpaperError>;

    fn set_style(&self, style: WallpaperStyle) -> Result<(), WallpaperError>;

    fn get_style(&self) -> Result<WallpaperStyle, WallpaperError>;

    fn list_monitors(&self) -> Result<Vec<OsString>, WallpaperError>;
}

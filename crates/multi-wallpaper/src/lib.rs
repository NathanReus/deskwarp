//! A cross‑platform library for managing the user’s desktop wallpaper.
//!
//! Currently fully supports **Windows** via the
//! [`IDesktopWallpaper`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-idesktopwallpaper)
//! COM interface. Linux and macOS are stubbed and will return
//! [`WallpaperError::Unsupported`] until implemented.
//!
//! # Quick Start (Windows)
//!
//! ```rust,no_run
//! use multi_wallpaper::{Wallpaper, WallpaperStyle};
//!
//! let wp = Wallpaper::new()?;
//! wp.set(
//!     std::path::Path::new("C:\\Users\\me\\Pictures\\cat.png"),
//!     WallpaperStyle::Fill,
//!     None, // all monitors
//! )?;
//! # Ok::<(), multi_wallpaper::WallpaperError>(())
//! ```

pub mod error;
mod platform;
pub mod style;

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use crate::platform::WallpaperManager;
pub use error::WallpaperError;
pub use style::WallpaperStyle;

/// Public API: a wallpaper manager that works on all supported platforms.
///
/// Under the hood it delegates to the OS‑specific implementation.
pub struct Wallpaper {
    inner: platform::WallpaperManagerImpl,
}

impl Wallpaper {
    /// Create a new `Wallpaper` instance.
    ///
    /// On Windows this requires COM to be initialised on the calling thread.
    /// The Windows implementation performs a one‑shot STA initialisation.
    pub fn new() -> Result<Self, WallpaperError> {
        Ok(Self {
            inner: platform::WallpaperManagerImpl::new()?,
        })
    }

    /// Set the wallpaper image and style for a monitor (or all monitors).
    ///
    /// * `path` - absolute path to an image file.
    /// * `style` - positioning mode (Fill, Fit, …).
    /// * `monitor` - monitor ID (e.g. `"\\\\.\\DISPLAY1"` on Windows). Use `None`
    ///   to apply to every monitor.
    pub fn set(
        &self,
        path: &std::path::Path,
        style: WallpaperStyle,
        monitor: Option<&OsStr>,
    ) -> Result<(), WallpaperError> {
        self.inner.set(path, style, monitor)
    }

    /// Get the absolute file path of the current wallpaper for a monitor.
    ///
    /// Returns `None` if no wallpaper is set.
    pub fn get(&self, monitor: Option<&OsStr>) -> Result<Option<PathBuf>, WallpaperError> {
        self.inner.get(monitor)
    }

    /// Change only the positioning style.
    pub fn set_style(&self, style: WallpaperStyle) -> Result<(), WallpaperError> {
        self.inner.set_style(style)
    }

    /// Get the current positioning style.
    pub fn get_style(&self) -> Result<WallpaperStyle, WallpaperError> {
        self.inner.get_style()
    }

    /// Return a list of available monitor identifiers.
    pub fn list_monitors(&self) -> Result<Vec<OsString>, WallpaperError> {
        self.inner.list_monitors()
    }
}

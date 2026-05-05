pub(crate) mod com;

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};
use windows::core::{HSTRING, PCWSTR};

use super::WallpaperManager;
use crate::error::WallpaperError;
use crate::style::WallpaperStyle;

/// Windows wallpaper manager wrapping the `IDesktopWallpaper` COM interface.
///
/// **Important**: COM is initialised automatically when this object is created
/// and is cleaned up automatically when it is dropped.
pub struct WindowsWallpaperManager {
    wallpaper: IDesktopWallpaper,
}

impl WallpaperManager for WindowsWallpaperManager {
    /// Create a new manager by instantiating the `IDesktopWallpaper` COM object.
    fn new() -> Result<Self, WallpaperError> {
        let _com = com::Com::new()?;

        let wallpaper: IDesktopWallpaper =
            unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)? };
        Ok(Self { wallpaper })
    }

    /// Set the wallpaper image for a specific monitor or all monitors.
    ///
    /// * `path` - absolute path to the image file (PNG, BMP, JPEG).
    /// * `style` - how the image should be positioned.
    /// * `monitor` - monitor device ID (e.g. `"\\\\.\\DISPLAY1"`) or `None` for all monitors.
    fn set(
        &self,
        path: &Path,
        style: WallpaperStyle,
        monitor: Option<&OsStr>,
    ) -> Result<(), WallpaperError> {
        // If a specific monitor was requested, ensure it exists.
        if let Some(mon) = monitor {
            let monitors = self.list_monitors()?;
            if !monitors.iter().any(|m| m.as_os_str() == mon) {
                return Err(WallpaperError::InvalidMonitor(
                    mon.to_string_lossy().into_owned(),
                ));
            }
        }

        // Convert the file path to a PCWSTR (always required).
        let path_hstr = HSTRING::from(path);
        let path_pcwstr = PCWSTR(path_hstr.as_ptr());

        // Build the monitor PCWSTR: null if None, otherwise a pointer to the string.
        let monitor_hstr;
        let monitor_pcwstr = if let Some(mon) = monitor {
            monitor_hstr = HSTRING::from(mon);
            PCWSTR(monitor_hstr.as_ptr())
        } else {
            PCWSTR::null()
        };

        unsafe {
            // Set the wallpaper image.
            self.wallpaper.SetWallpaper(monitor_pcwstr, path_pcwstr)?;
            // Apply the desired placement style.
            self.wallpaper.SetPosition(style.to_windows())?;
        }
        Ok(())
    }

    /// Retrieve the file path of the current wallpaper for a given monitor.
    ///
    /// Returns `None` if no wallpaper has been set.
    fn get(&self, monitor: Option<&OsStr>) -> Result<Option<PathBuf>, WallpaperError> {
        let monitor_hstr;
        let monitor_pcwstr = if let Some(mon) = monitor {
            monitor_hstr = HSTRING::from(mon);
            PCWSTR(monitor_hstr.as_ptr())
        } else {
            PCWSTR::null()
        };

        // Call the Win32 API
        let path_hstr = unsafe { self.wallpaper.GetWallpaper(monitor_pcwstr)? };

        // Empty string means no wallpaper set
        if unsafe { path_hstr.is_empty() } {
            return Ok(None);
        }

        // Convert UTF-16 -> OsString > PathBuf (lossless, correct on Windows)
        let path = PathBuf::from(OsString::from_wide(unsafe { path_hstr.as_wide() }));

        Ok(Some(path))
    }

    /// Change only the positioning style.
    fn set_style(&self, style: WallpaperStyle) -> Result<(), WallpaperError> {
        unsafe {
            self.wallpaper.SetPosition(style.to_windows())?;
        }
        Ok(())
    }

    /// Get the current positioning style.
    fn get_style(&self) -> Result<WallpaperStyle, WallpaperError> {
        let pos = unsafe { self.wallpaper.GetPosition() }?;
        WallpaperStyle::from_windows(pos).ok_or(WallpaperError::UnknownWallpaperPosition(format!(
            "{:?}",
            pos
        )))
    }

    /// Enumerate all monitor device IDs that can be used with this API.
    ///
    /// Returns a vector of OS strings like `"\\\\.\\DISPLAY1"`.
    fn list_monitors(&self) -> Result<Vec<OsString>, WallpaperError> {
        let count = unsafe { self.wallpaper.GetMonitorDevicePathCount()? };
        let mut monitors = Vec::with_capacity(count as usize);
        for i in 0..count {
            let hstr = unsafe { self.wallpaper.GetMonitorDevicePathAt(i)? };
            let os = OsString::from_wide(unsafe { hstr.as_wide() });
            monitors.push(os);
        }
        Ok(monitors)
    }
}

// SAFETY:
// 1. The inner COM interface is a proxy to an out‑of‑process shell object.
//    Calls are marshalled and thread‑safe at the system level.
// 2. This type is intended to be used only on the thread that creates it
//    (the COM initialisation performed by `com.rs` ensures that thread is
//    COM‑ready).  We do not move it to other threads after construction.
unsafe impl Send for WindowsWallpaperManager {}
unsafe impl Sync for WindowsWallpaperManager {}

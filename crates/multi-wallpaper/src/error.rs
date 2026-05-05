use thiserror::Error;

/// Unified error type for all wallpaper operations.
#[derive(Debug, Error)]
pub enum WallpaperError {
    /// An error coming from the underlying OS layer (e.g., Win32 COM).
    #[error("OS operation failed: {0}")]
    Os(String),

    /// A standard I/O error (e.g., when a file does not exist).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The requested feature is not available on the current platform.
    #[error("Feature not supported on this platform")]
    Unsupported,

    /// The supplied monitor identifier is invalid.
    #[error("Invalid monitor ID: {0}")]
    InvalidMonitor(String),

    /// An unknown wallpaper positioning value was returned by the OS.
    #[error("Unknown desktop wallpaper position returned: {0:?}")]
    UnknownWallpaperPosition(String),
}

// Allow automatic conversion from windows::core::Error on Windows.
#[cfg(target_os = "windows")]
impl From<windows::core::Error> for WallpaperError {
    fn from(err: windows::core::Error) -> Self {
        WallpaperError::Os(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = WallpaperError::Unsupported;
        assert_eq!(format!("{}", err), "Feature not supported on this platform");

        let err = WallpaperError::InvalidMonitor("test".to_string());
        assert_eq!(format!("{}", err), "Invalid monitor ID: test");
    }

    #[test]
    fn test_io_error_conversion_integration() {
        use std::io::{Error, ErrorKind};
        let io_err = Error::new(ErrorKind::NotFound, "file not found");
        let wallpaper_err: WallpaperError = io_err.into();
        assert!(matches!(wallpaper_err, WallpaperError::Io(_)));
    }
}

#[cfg(all(test, target_os = "windows"))]
mod windows_tests {
    use super::*;

    #[test]
    fn test_windows_core_error_conversion() {
        let hr = windows::core::HRESULT::from_win32(0x80070002); // ERROR_FILE_NOT_FOUND
        let err = windows::core::Error::from(hr);
        let wallpaper_err: WallpaperError = err.into();
        assert!(matches!(wallpaper_err, WallpaperError::Os(_)));
    }
}

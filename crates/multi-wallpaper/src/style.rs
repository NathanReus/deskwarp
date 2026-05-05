/// How the wallpaper should be displayed on the desktop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallpaperStyle {
    /// Center the image without scaling.
    Center,
    /// Tile the image repeatedly.
    Tile,
    /// Stretch the image to fill the screen.
    Stretch,
    /// Fit the image to the screen, adding letterboxing.
    Fit,
    /// Fill the screen, cropping if necessary.
    Fill,
    /// Span the image across multiple monitors (Windows 8+).
    Span,
}

// ---------------------------------------------------------------------------
// Windows‑specific conversion to the COM enumeration.
// ---------------------------------------------------------------------------
#[cfg(target_os = "windows")]
impl WallpaperStyle {
    /// Convert to the Win32 `DESKTOP_WALLPAPER_POSITION` equivalent.
    pub(crate) fn to_windows(self) -> windows::Win32::UI::Shell::DESKTOP_WALLPAPER_POSITION {
        use windows::Win32::UI::Shell::{
            DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
        };
        match self {
            WallpaperStyle::Center => DWPOS_CENTER,
            WallpaperStyle::Tile => DWPOS_TILE,
            WallpaperStyle::Stretch => DWPOS_STRETCH,
            WallpaperStyle::Fit => DWPOS_FIT,
            WallpaperStyle::Fill => DWPOS_FILL,
            WallpaperStyle::Span => DWPOS_SPAN,
        }
    }

    /// Create a `WallpaperStyle` from a Windows `DESKTOP_WALLPAPER_POSITION`.
    pub(crate) fn from_windows(
        pos: windows::Win32::UI::Shell::DESKTOP_WALLPAPER_POSITION,
    ) -> Option<Self> {
        use windows::Win32::UI::Shell::{
            DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
        };
        Some(match pos {
            DWPOS_CENTER => WallpaperStyle::Center,
            DWPOS_TILE => WallpaperStyle::Tile,
            DWPOS_STRETCH => WallpaperStyle::Stretch,
            DWPOS_FIT => WallpaperStyle::Fit,
            DWPOS_FILL => WallpaperStyle::Fill,
            DWPOS_SPAN => WallpaperStyle::Span,
            _ => return None,
        })
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;
    use windows::Win32::UI::Shell::{
        DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
    };

    #[test]
    fn test_to_windows_conversion() {
        assert_eq!(WallpaperStyle::Center.to_windows(), DWPOS_CENTER);
        assert_eq!(WallpaperStyle::Tile.to_windows(), DWPOS_TILE);
        assert_eq!(WallpaperStyle::Stretch.to_windows(), DWPOS_STRETCH);
        assert_eq!(WallpaperStyle::Fit.to_windows(), DWPOS_FIT);
        assert_eq!(WallpaperStyle::Fill.to_windows(), DWPOS_FILL);
        assert_eq!(WallpaperStyle::Span.to_windows(), DWPOS_SPAN);
    }

    #[test]
    fn test_from_windows_known_positions() {
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_CENTER),
            Some(WallpaperStyle::Center)
        );
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_TILE),
            Some(WallpaperStyle::Tile)
        );
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_STRETCH),
            Some(WallpaperStyle::Stretch)
        );
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_FIT),
            Some(WallpaperStyle::Fit)
        );
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_FILL),
            Some(WallpaperStyle::Fill)
        );
        assert_eq!(
            WallpaperStyle::from_windows(DWPOS_SPAN),
            Some(WallpaperStyle::Span)
        );
    }

    #[test]
    fn test_from_windows_unknown_position() {
        // Simulate an unknown value. The actual numeric value isn't important,
        // but it should be one that is not mapped.
        let unknown_pos = unsafe {
            std::mem::transmute::<i32, windows::Win32::UI::Shell::DESKTOP_WALLPAPER_POSITION>(99)
        };
        assert_eq!(WallpaperStyle::from_windows(unknown_pos), None);
    }
}

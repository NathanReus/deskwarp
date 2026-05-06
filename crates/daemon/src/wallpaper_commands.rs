use multi_wallpaper::WallpaperStyle;
use std::path::PathBuf;
use tokio::sync::oneshot;

/// A wallpaper operation the HTTP layer can request from the main thread.
/// This must be on the main thread due to Windows requiring COM communication there.
pub enum WallpaperCommand {
    /// List all monitors
    ListMonitors { reply: oneshot::Sender<Vec<String>> },
    /// Get the current wallpaper path for a monitor
    GetWallpaper {
        monitor: Option<PathBuf>,
        reply: oneshot::Sender<Option<PathBuf>>,
    },
    GetStyle {
        reply: oneshot::Sender<WallpaperStyle>,
    },
    SetStyle {
        style: WallpaperStyle,
        reply: oneshot::Sender<Result<(), String>>,
    },
    SetWallpaper {
        path: PathBuf,
        style: WallpaperStyle,
        monitor: Option<String>,
        reply: oneshot::Sender<Result<(), String>>,
    },
}

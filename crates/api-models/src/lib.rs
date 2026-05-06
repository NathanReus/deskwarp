use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ── Monitor ───────────────────────────────────────────────────────

/// Information about a single monitor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    /// The system’s identifier for the monitor (e.g. `\\.\\DISPLAY1`).
    pub id: String,
    /// A human‑readable name (if available, otherwise the same as `id`).
    pub name: String,
}

// ── Wallpaper Style ───────────────────────────────────────────────

/// Positioning style for a wallpaper image.
///
/// This enum deliberately mirrors the library’s own `WallpaperStyle`
/// so that conversion is a simple one‑to‑one mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperStyle {
    Center,
    Tile,
    Stretch,
    Fit,
    Fill,
    Span,
}

// ── Set Wallpaper Request ─────────────────────────────────────────

/// Request body for `POST /api/wallpaper/set`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWallpaperRequest {
    /// Absolute path to the image file.
    pub path: PathBuf,
    /// How the image should be positioned.
    pub style: WallpaperStyle,
    /// Apply to a specific monitor (by its `id`).
    /// Omit or set to `None` to apply to **all** monitors.
    pub monitor: Option<String>,
}

// ── Current Wallpaper Response ────────────────────────────────────

/// Response for `GET /api/wallpaper?monitor=…`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentWallpaper {
    /// The monitor this information applies to.
    pub monitor: PathBuf,
    /// Absolute path of the current image, or `None` if no wallpaper set.
    pub path: Option<PathBuf>,
}

// ── Monitor List Response ─────────────────────────────────────────

/// Response for `GET /api/monitors`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorList {
    pub monitors: Vec<Monitor>,
}

// ── Set Style Request ─────────────────────────────────────────

/// Request body for `PUT /api/style`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleRequest {
    pub style: WallpaperStyle,
}

// ── Current Style Response ────────────────────────────────────

/// Response body for `GET /api/style`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleResponse {
    pub style: WallpaperStyle,
}

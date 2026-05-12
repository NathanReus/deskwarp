use crate::events::UserEvent;
use crate::server::helpers;
use crate::state::AppState;
use crate::wallpaper_commands::WallpaperCommand;
use axum::{
    Json,
    extract::{Query, State},
};
use deskwarp_api_models::*;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::oneshot;

/// `GET /api/monitors` – list all available monitors.
pub async fn list_monitors(State(state): State<Arc<AppState>>) -> Json<MonitorList> {
    tracing::debug!("GET /api/monitors");

    let (tx, rx) = oneshot::channel();
    let cmd = WallpaperCommand::ListMonitors { reply: tx };
    // Send command to main thread (ignore error if disconnected)
    let _ = state
        .proxy
        .send_event(UserEvent::WallpaperCommandEvent(cmd));
    // Wait for response

    let monitors = rx.await.unwrap_or_default();
    tracing::debug!(count = monitors.len(), "Monitors fetched");
    let monitor_list = monitors
        .into_iter()
        .map(|id| Monitor {
            id: id.clone(),
            name: id, // For now, name = id
        })
        .collect();

    tracing::debug!(monitors = ?monitor_list, "Monitors received");
    Json(MonitorList {
        monitors: monitor_list,
    })
}

/// Query parameters for `GET /api/wallpaper`.
#[derive(Deserialize)]
pub struct WallpaperQuery {
    pub monitor: Option<PathBuf>,
}

/// `GET /api/wallpaper` – get current wallpaper for a monitor (or default).
pub async fn get_current_wallpaper(
    State(state): State<Arc<AppState>>,
    Query(query): Query<WallpaperQuery>,
) -> Json<CurrentWallpaper> {
    tracing::debug!(monitor = ?query.monitor, "GET /api/wallpaper");

    let (tx, rx) = oneshot::channel();
    let cmd = WallpaperCommand::GetWallpaper {
        monitor: query.monitor.clone(),
        reply: tx,
    };
    // Send command to main thread (ignore error if disconnected)
    let _ = state
        .proxy
        .send_event(UserEvent::WallpaperCommandEvent(cmd));
    // Wait for response
    let path = rx.await.unwrap_or(None);

    tracing::debug!(
        monitor = ?query.monitor.clone(),
        ?path,
        "Wallpaper data returned"
    );

    Json(CurrentWallpaper {
        monitor: query.monitor.unwrap_or_else(|| PathBuf::from("all")),
        path,
    })
}

/// `GET /api/style`
pub async fn get_style(State(state): State<Arc<AppState>>) -> Json<StyleResponse> {
    tracing::debug!("GET /api/style");

    let (tx, rx) = oneshot::channel();
    let cmd = WallpaperCommand::GetStyle { reply: tx };
    let _ = state
        .proxy
        .send_event(UserEvent::WallpaperCommandEvent(cmd));
    let style = rx.await.expect("No style received");

    tracing::debug!(style = ?style, "Style received");
    Json(StyleResponse {
        style: helpers::into_api_style(style),
    })
}

/// `PUT /api/style`
pub async fn set_style(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SetStyleRequest>,
) -> Result<Json<StyleResponse>, String> {
    tracing::debug!(style = ?req.style, "PUT /api/style");
    let (tx, rx) = oneshot::channel();
    let cmd = WallpaperCommand::SetStyle {
        style: helpers::into_multi_style(req.style),
        reply: tx,
    };
    let _ = state
        .proxy
        .send_event(UserEvent::WallpaperCommandEvent(cmd));
    rx.await
        .map_err(|_| "internal error".to_string())?
        .map_err(|e| e)?;

    Ok(Json(StyleResponse { style: req.style }))
    // TODO: Change the response of this to just be a success/failure
}

/// `POST /api/wallpaper`
pub async fn set_wallpaper(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SetWallpaperRequest>,
) -> Result<Json<CurrentWallpaper>, String> {
    tracing::debug!(path = ?req.path.display(), style = ?req.style, monitor = ?req.monitor, "POST /api/wallpaper");
    let (tx, rx) = oneshot::channel();
    let cmd = WallpaperCommand::SetWallpaper {
        path: req.path.clone(),
        style: helpers::into_multi_style(req.style),
        monitor: req.monitor.clone(),
        reply: tx,
    };
    let _ = state
        .proxy
        .send_event(UserEvent::WallpaperCommandEvent(cmd));
    rx.await
        .map_err(|_| "internal error".to_string())?
        .map_err(|e| e)?;

    tracing::debug!("Wallpaper set successfully");
    Ok(Json(CurrentWallpaper {
        monitor: PathBuf::from(req.monitor.unwrap_or_else(|| "all".to_string())),
        path: Some(req.path),
    }))
}

mod helpers;
mod routes;

use crate::state::AppState;
use anyhow::Context;
use axum::{Router, routing::get};
use std::sync::Arc;
use std::{path::PathBuf, thread::JoinHandle};
use tokio::sync::oneshot;

/// Manages the background HTTP daemon server.
pub struct Server {
    port_rx: Option<oneshot::Receiver<u16>>,
    thread: JoinHandle<anyhow::Result<()>>,
}

impl Server {
    /// Spawns a new server thread and returns a handle to it.
    pub fn new(
        shutdown_rx: oneshot::Receiver<()>,
        port_file: PathBuf,
        state: Arc<AppState>,
    ) -> Self {
        let (port_tx, port_rx) = oneshot::channel();
        let thread = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(Self::run(port_tx, shutdown_rx, port_file, state))
        });
        Self {
            port_rx: Some(port_rx),
            thread,
        }
    }

    /// Blocks until the server has bound to a port and returns the port.
    pub fn wait_for_port(&mut self) -> anyhow::Result<u16> {
        let rx = self.port_rx.take().context("Port already taken")?;
        rx.blocking_recv().context("server did not start")
    }

    /// Blocks until the server thread finishes (after graceful shutdown).
    pub fn join(self) -> anyhow::Result<()> {
        self.thread.join().unwrap()
    }

    // ── private helpers ────────────────────────────────────────────

    async fn run(
        port_tx: oneshot::Sender<u16>,
        shutdown_rx: oneshot::Receiver<()>,
        port_file: PathBuf,
        state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        let router = Router::new()
            .route("/api/health", get(|| async { "OK" }))
            .route("/api/monitors", get(routes::list_monitors))
            .route(
                "/api/wallpaper",
                get(routes::get_current_wallpaper).post(routes::set_wallpaper),
            )
            .route("/api/style", get(routes::get_style).put(routes::set_style))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .context("Failed to bind TCP listener")?;

        let port = listener.local_addr()?.port();

        tracing::info!(port = %port, "Server socket bound");

        // Write port to file
        std::fs::write(&port_file, port.to_string())?;

        // Send port back to main thread so the tray can be setup
        let _ = port_tx.send(port);

        tracing::info!("Server ready, awaiting requests");

        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
                tracing::info!("Shutdown signal received, stopping server...");
            })
            .await?;

        tracing::info!("Server stopped gracefully");

        Ok(())
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod events;
mod server;
mod state;

use config::Config;
use events::UserEvent;
use server::Server;

use std::sync::{Arc, RwLock};
use tokio::sync::oneshot::{Sender, channel};
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::state::AppState;

struct App {
    shutdown_tx: Option<Sender<()>>,
    server_handle: Option<Server>,
    config: Config,
    state: Arc<RwLock<AppState>>,
    tray_menu: Option<Menu>,
    tray_menu_next_wallpaper: MenuItem,
    tray_menu_open_gui: MenuItem,
    tray_menu_exit: MenuItem,
    tray_icon: Option<TrayIcon>,
}

impl App {
    fn new_tray_icon(&mut self) -> TrayIcon {
        let icon = Self::load_icon();

        TrayIconBuilder::new()
            .with_menu(Box::new(self.tray_menu.as_ref().unwrap().clone()))
            .with_tooltip("DeskWarp")
            .with_icon(icon)
            .build()
            .expect("Failed to create tray icon")
    }

    fn load_icon() -> Icon {
        let rgba = image::load_from_memory(include_bytes!("../assets/images/icon.png"))
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();

        Icon::from_rgba(pixels, width, height).expect("Failed to load icon")
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.tray_icon.is_none() {
            self.tray_icon = Some(self.new_tray_icon());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // No windows to handle, but Trait requires this method
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(tray_icon_event) => {}
            UserEvent::MenuEvent(menu_event) => {
                if menu_event.id() == self.tray_menu_next_wallpaper.id() {
                    tracing::info!("Next wallpaper requested");
                } else if menu_event.id() == self.tray_menu_open_gui.id() {
                    tracing::info!("Open GUI requested");
                } else if menu_event.id() == self.tray_menu_exit.id() {
                    tracing::info!("Exit requested");
                    if let Some(tx) = self.shutdown_tx.take() {
                        let _ = tx.send(());
                    }
                    event_loop.exit();
                }
            }
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        // Perform any cleanup here before exiting
    }
}

fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Project directories
    let proj_dirs = directories::ProjectDirs::from("app", "deskwarp", "deskwarp-daemon")
        .expect("failed to get project directories");
    std::fs::create_dir_all(proj_dirs.config_dir())?;

    // Config
    let config_path = proj_dirs.config_dir().join("config.toml");
    let config = Config::load(&config_path);
    let port_file = proj_dirs.config_dir().join("daemon_port");

    // Graceful shutdown channel
    let (shutdown_tx, shutdown_rx) = channel();

    // Create the app state
    let state = Arc::new(RwLock::new(AppState::new()));

    // Start a server on a background thread
    let mut server_handle = Server::new(shutdown_rx, port_file, state.clone());

    // Wait for server's port
    let port = server_handle.wait_for_port()?;
    tracing::info!("Daemon ready, tray will connect to port {}", port);

    // Tray (main thread)
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;

    // Forward events from tray to the event loop
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    // Forward events from menu to the event loop
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event));
    }));

    // Build the tray menu
    let menu_next_wallpaper = MenuItem::new("Next wallpaper", true, None);
    let menu_open_gui = MenuItem::new("Open settings", true, None);
    let menu_exit = MenuItem::new("Exit", true, None);
    let tray_menu = Menu::with_items(&[&menu_next_wallpaper, &menu_open_gui, &menu_exit])?;

    let mut app = App {
        shutdown_tx: Some(shutdown_tx),
        server_handle: Some(server_handle),
        config: config,
        state: state,
        tray_menu: Some(tray_menu),
        tray_menu_next_wallpaper: menu_next_wallpaper,
        tray_menu_open_gui: menu_open_gui,
        tray_menu_exit: menu_exit,
        tray_icon: None,
    };

    // This blocks until event_loop.exit() is called
    event_loop.run_app(&mut app)?;

    // Wait for the server thread to finish
    if let Some(handle) = app.server_handle.take() {
        handle.join()?;
    }

    Ok(())
}

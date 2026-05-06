use crate::wallpaper_commands::WallpaperCommand;
use tray_icon::{TrayIconEvent, menu::MenuEvent};

pub(crate) enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
    WallpaperCommandEvent(WallpaperCommand),
}

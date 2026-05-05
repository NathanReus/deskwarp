use multi_wallpaper::Wallpaper;
use std::sync::{Arc, RwLock};

pub struct AppState {
    pub wallpaper: Wallpaper,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            wallpaper: Wallpaper::new().expect("failed to create wallpaper"),
        }
    }

    pub fn into_arc(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}

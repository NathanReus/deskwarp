use crate::events::UserEvent;
use winit::event_loop::EventLoopProxy;

pub struct AppState {
    pub proxy: EventLoopProxy<UserEvent>,
}

impl AppState {
    pub fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        Self { proxy }
    }
}

pub mod create;
pub mod delete;
pub mod presets;
pub mod rename;
pub mod sessions;
pub mod set_default_preset;

use crate::app::driver::{AppState, AppEvent};

pub trait Menu {
    fn handle_event(&mut self, event: AppEvent, state: &mut AppState);
}

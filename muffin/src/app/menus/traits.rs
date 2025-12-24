use crate::app::driver::AppState;

use super::super::driver::AppEvent;

pub trait Menu {
    fn handle_event(&mut self, event: AppEvent, state: &mut AppState);
}

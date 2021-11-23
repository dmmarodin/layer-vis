use super::logging;
use super::window;

use window::WindowState;

struct GameState {
    window_state: WindowState
}

impl GameState {
    pub fn new(window_state: WindowState) -> Self {
        Self {
            window_state
        }
    }
}

pub fn init() {
    logging::prep_logging();
    WindowState::new();
}
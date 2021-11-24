use super::logging;
use super::window;

use window::WindowState;
use winit::dpi::PhysicalSize;
use winit::event::*;

pub struct PlataformState {
    pub window_state: WindowState,
}

impl PlataformState {
    pub fn new() -> Self {
        let window_state = WindowState::new();

        Self { window_state }
    }
}

pub fn init() -> PlataformState {
    logging::prep_logging();
    let plataform_state = PlataformState::new();
    plataform_state
}
use super::logging;
use super::window;

use window::WindowState;
use winit::dpi::PhysicalSize;
use winit::event::*;

struct PlataformState {
    pub window_state: WindowState,
}

impl PlataformState {
    pub fn new() -> Self {
        let window_state = WindowState::new();

        Self { window_state }
    }
}

pub fn init() {
    logging::prep_logging();
    let plataform_state = PlataformState::new();

    plataform_state
        .window_state
        .event_loop
        .run(move |event, _, _| {
            let body = plataform_state.window_state.document.body().unwrap();
            let size = PhysicalSize {width: body.client_width() as u32, height: body.client_height() as u32};
            if plataform_state.window_state.window.inner_size() != size {
                plataform_state.window_state.window.set_inner_size(size);
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == plataform_state.window_state.window.id() => match event {
                    _ => {}
                },
                _ => {}
            }
        });
}
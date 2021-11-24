use wasm_bindgen::prelude::*;
use winit::event::Event;
use winit::dpi::PhysicalSize;

mod plataform;

#[wasm_bindgen(start)]
pub fn main() {
    let plataform_state = plataform::init();

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


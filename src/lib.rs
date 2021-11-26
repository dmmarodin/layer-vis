use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::Event;

mod plataform;
mod renderer;
use renderer::RendererState;

#[wasm_bindgen(start)]
pub async fn main() {
    let plataform_state = plataform::init();
    let mut renderer_state = RendererState::new(&plataform_state.window_state.window).await;

    plataform_state
        .window_state
        .event_loop
        .run(move |event, _, _| {
            let body = plataform_state.window_state.document.body().unwrap();
            let size = PhysicalSize {
                width: body.client_width() as u32,
                height: body.client_height() as u32,
            };
            if plataform_state.window_state.window.inner_size() != size {
                plataform_state.window_state.window.set_inner_size(size);
                renderer_state.resize(size);
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == plataform_state.window_state.window.id() => {
                    if !renderer_state.input(event) {
                        match event {
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    renderer_state.render();
                },
                Event::MainEventsCleared => {
                    plataform_state.window_state.window.request_redraw();
                },
                _ => {}
            }
        });
}

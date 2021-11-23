extern crate winit;
extern crate js_sys;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub struct WindowState {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
    pub document: web_sys::Document,
    canvas: HtmlCanvasElement
}

impl WindowState {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("LayerVis")
                .build(&event_loop)
                .unwrap();

            use winit::platform::web::WindowExtWebSys;
            let canvas = window
                .canvas()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

            let html_window: web_sys::Window = web_sys::window().unwrap();
            let document = html_window.document().unwrap();

            let body = document
                .body()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            body.append_child(&canvas)
                .expect("Append canvas to HTML body");

            Self {
                window,
                event_loop,
                document,
                canvas,
            }
        }

    }
}
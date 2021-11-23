extern crate winit;
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::Document;
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
    event_loop::EventLoop
};

pub struct WindowState {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement,
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
            body.style().set_property("margin", "0px");
            body.style().set_property("height", "100%");

            let screen_size_x = body.client_width() as u32;
            let screen_size_y = body.client_height() as u32;

            canvas.set_width(screen_size_x);
            canvas.set_height(screen_size_y);
            canvas
                .style()
                .set_property("width", &format!("{}px", screen_size_x))
                .unwrap();
            canvas
                .style()
                .set_property("height", &format!("{}px", screen_size_y))
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
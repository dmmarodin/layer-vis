use wasm_bindgen::prelude::*;

mod plataform;

#[wasm_bindgen(start)]
pub fn main() {
    plataform::init();
}


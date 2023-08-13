use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    web_sys::console::log_1(&"WALRUST TIME".into());
}

use wasm_bindgen::prelude::*;

mod app;
mod components;
mod models;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}

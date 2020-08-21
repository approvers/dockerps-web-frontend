#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

mod app;
mod components;
mod layouts;
mod models;
mod services;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}

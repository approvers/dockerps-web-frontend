use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}

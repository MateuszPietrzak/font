use wasm_bindgen::prelude::*;

pub mod state;
pub mod renderer;
pub mod bezier;

#[wasm_bindgen(module = "/src/loop.js")]
extern "C" {
    fn run_js_loop(state: state::AppState);
}

#[wasm_bindgen(start)]
async fn start() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);

    let state = state::AppState::new_with_init().await;

    run_js_loop(state);
}

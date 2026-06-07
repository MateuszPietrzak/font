use wasm_bindgen::prelude::*;

use crate::renderer;

#[wasm_bindgen]
pub struct AppState {
    renderer: renderer::RenderingState,
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen]
    pub async fn new_with_init() -> AppState {
        Self {
            renderer: renderer::RenderingState::new().await,
        }
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.renderer.resize(new_width, new_height);
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.renderer.update();
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.renderer.render();
    }
}

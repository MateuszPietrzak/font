use crate::{bezier, renderer};
use wasm_bindgen::prelude::*;
extern crate nalgebra_glm as glm;

#[wasm_bindgen]
pub struct AppState {
    renderer: renderer::RenderingState,
    beziers: Vec<bezier::Bezier>,
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen]
    pub async fn new_with_init() -> AppState {
        Self {
            renderer: renderer::RenderingState::new().await,
            beziers: vec![
                bezier::Bezier::new(
                    glm::Vec2::new(-0.5, 0.25),
                    glm::Vec2::new(0.25, 0.25),
                    glm::Vec2::new(-0.5, -0.25),
                    glm::Vec2::new(0.25, -0.25),
                ),
                bezier::Bezier::new(
                    glm::Vec2::new(-0.25, 0.25),
                    glm::Vec2::new(-0.25, 0.75),
                    glm::Vec2::new(0.25, 0.75),
                    glm::Vec2::new(0.25, 0.25),
                ),
            ],
        }
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.renderer.resize(new_width, new_height);
    }

    #[wasm_bindgen]
    pub fn frame(&mut self) {
        self.renderer.begin_draw();

        for bezier in &self.beziers {
            self.renderer.bezier(bezier);
        }

        self.renderer.end_draw();
    }
}

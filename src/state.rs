use crate::{bezier, renderer};
use wasm_bindgen::prelude::*;
extern crate nalgebra_glm as glm;

#[wasm_bindgen]
pub struct AppState {
    renderer: renderer::RenderingState,
    beziers: Vec<bezier::Bezier>,
    start_time: web_time::SystemTime,
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
            start_time: web_time::SystemTime::now(),
        }
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.renderer.resize(new_width, new_height);
    }

    #[wasm_bindgen]
    pub fn frame(&mut self) {
        let time = web_time::SystemTime::now()
            .duration_since(self.start_time)
            .unwrap()
            .as_secs_f32();

        self.beziers[0] = bezier::Bezier::new(
            glm::Vec2::new(-0.5 + time.cos() * 0.25, 0.25 + time.sin() * 0.125),
            glm::Vec2::new(0.25, 0.25),
            glm::Vec2::new(-0.5 + (time * 0.5).cos() * 0.25, -0.25 + (time * 0.5).sin() * 0.25),
            glm::Vec2::new(0.25, -0.25),
        );

        self.renderer.begin_draw();

        for bezier in &self.beziers {
            self.renderer.bezier(bezier);
        }

        self.renderer.end_draw();
    }
}

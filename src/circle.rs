extern crate nalgebra_glm as glm;

pub struct Circle {
    pub p: glm::Vec2,
    pub r: f32,
}

impl Circle {
    pub fn new(p: glm::Vec2, r: f32) -> Self {
        Self { p, r }
    }
}

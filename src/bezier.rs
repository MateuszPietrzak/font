extern crate nalgebra_glm as glm;

pub struct Bezier {
    pub p0: glm::Vec2,
    pub p1: glm::Vec2,
    pub p2: glm::Vec2,
    pub p3: glm::Vec2,
}

impl Bezier {
    pub fn new(p0: glm::Vec2, p1: glm::Vec2, p2: glm::Vec2, p3: glm::Vec2) -> Self {
        Self { p0, p1, p2, p3 }
    }
}

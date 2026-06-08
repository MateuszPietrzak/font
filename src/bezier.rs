extern crate nalgebra_glm as glm;

pub struct Bezier {
    p0: glm::Vec2,
    p1: glm::Vec2,
    p2: glm::Vec2,
    p3: glm::Vec2,
}

impl Bezier {
    pub fn new(p0: glm::Vec2, p1: glm::Vec2, p2: glm::Vec2, p3: glm::Vec2) -> Self {
        Self { p0, p1, p2, p3 }
    }

    pub fn to_list(&self) -> [glm::Vec2; 4] {
        [self.p0, self.p1, self.p2, self.p3]
    }
}

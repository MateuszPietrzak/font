extern crate nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    dimensions: [f32; 2],
    _padding: [f32; 2]
}

impl Uniform {
    pub fn new(dimensions: [f32; 2]) -> Self {
        Self {
            dimensions,
            _padding: [0.0, 0.0],
        }
    }
}
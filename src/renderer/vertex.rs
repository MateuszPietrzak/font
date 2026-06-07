extern crate nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0] },
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [1.0, -1.0] },
];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
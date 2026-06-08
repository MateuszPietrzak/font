use crate::bezier;

extern crate nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceInput {
    matrix_col_0: [f32; 2],
    matrix_col_1: [f32; 2],
    matrix_col_2: [f32; 2],
    p0: [f32; 2],
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
}

impl InstanceInput {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x2,
        5 => Float32x2,
        6 => Float32x2,
        7 => Float32x2,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub fn bezier_to_instance_input(bezier: &bezier::Bezier) -> InstanceInput {
    let points = [bezier.p0, bezier.p1, bezier.p2, bezier.p3];
    let min_x: f32 = points.iter().map(|p| p.x).reduce(f32::min).unwrap_or(-1.0);
    let min_y: f32 = points.iter().map(|p| p.y).reduce(f32::min).unwrap_or(-1.0);
    let max_x: f32 = points.iter().map(|p| p.x).reduce(f32::max).unwrap_or(1.0);
    let max_y: f32 = points.iter().map(|p| p.y).reduce(f32::max).unwrap_or(1.0);

    let mid_x = (max_x + min_x) / 2.0;
    let mid_y = (max_y + min_y) / 2.0;

    let width = max_x - min_x;
    let height = max_y - min_y;

    InstanceInput {
        matrix_col_0: [width * 0.5 + 0.05, 0.0],
        matrix_col_1: [0.0, height * 0.5 + 0.05],
        matrix_col_2: [mid_x, mid_y],
        p0: [bezier.p0.x, bezier.p0.y],
        p1: [bezier.p1.x, bezier.p1.y],
        p2: [bezier.p2.x, bezier.p2.y],
        p3: [bezier.p3.x, bezier.p3.y],
    }
}
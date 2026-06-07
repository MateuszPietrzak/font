// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct InstanceInput {
    @location(1) mat_col_0: vec2<f32>,
    @location(2) mat_col_1: vec2<f32>,
    @location(3) mat_col_2: vec2<f32>,
    @location(4) p0: vec2<f32>,
    @location(5) p1: vec2<f32>,
    @location(6) p2: vec2<f32>,
    @location(7) p3: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) p0: vec2<f32>,
    @location(2) p1: vec2<f32>,
    @location(3) p2: vec2<f32>,
    @location(4) p3: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let mat = mat2x2<f32>(instance.mat_col_0, instance.mat_col_1);

    var out: VertexOutput;
    out.color = model.position.xyy * 0.5 + 0.5; 
    out.clip_position = vec4<f32>(mat * model.position + instance.mat_col_2, 0.0, 1.0);

    out.p0 = instance.p0;
    out.p1 = instance.p1;
    out.p2 = instance.p2;
    out.p3 = instance.p3;

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    return vec4<f32>(in.color, 1.0);
}

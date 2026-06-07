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
    @location(0) ndc_pos: vec2<f32>,
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
    let transformed_pos = mat * model.position + instance.mat_col_2;
    out.clip_position = vec4<f32>(transformed_pos, 0.0, 1.0);
    out.ndc_pos = transformed_pos;

    out.p0 = instance.p0;
    out.p1 = instance.p1;
    out.p2 = instance.p2;
    out.p3 = instance.p3;

    return out;
}

// Fragment shader

struct Uniform {
    dimensions: vec2<f32>,
    _padding: vec2<f32>,
};

@group(0) @binding(0) 
var<uniform> dim_uniform: Uniform;

fn cubic_pos(t: f32, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> vec2<f32>{
    let mt = 1.0 - t;
    return mt * mt * mt * p0 + 3.0 * mt * mt * t * p1 + 3.0 * mt * t * t * p2 + t * t * t * p3;
}

fn cubic_tangent(t: f32, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> vec2<f32> {
    let mt = 1.0 - t;
    return 3.0 * mt * mt * (p1 - p0) + 6.0 * mt * t * (p2 - p1) + 3.0 * t * t * (p3 - p2);
}

fn closest_t(pixel_pos: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> f32 {
    var t = 0.5; 
    
    for (var i = 0; i < 5; i++) {
        let p = cubic_pos(t, p0, p1, p2, p3);
        let tangent = cubic_tangent(t, p0, p1, p2, p3);
        
        let f = dot(p - pixel_pos, tangent);
        let f_prime = dot(tangent, tangent);
        
        t = clamp(t - f / f_prime, 0.0, 1.0);
    }
    return t;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let t = closest_t(in.ndc_pos, in.p0, in.p1, in.p2, in.p3);

    let on_curve = cubic_pos(t, in.p0, in.p1, in.p2, in.p3);
    let dist = length(in.ndc_pos - on_curve);

    let half_width= 0.005;

    let edge_blur = fwidth(dist);
    let alpha = smoothstep(half_width + edge_blur, half_width - edge_blur, dist);
    
    if (alpha == 0.0) {
        discard;
    }

    
    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}

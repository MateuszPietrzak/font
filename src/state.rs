use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
extern crate nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0] },
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [1.0, -1.0] },
];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceInput {
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

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

fn points_to_instance_input(points: [glm::Vec2; 4]) -> InstanceInput {
    let min_x: f32 = points.iter().map(|p| p.x).reduce(f32::min).unwrap_or(-1.0);
    let min_y: f32 = points.iter().map(|p| p.y).reduce(f32::min).unwrap_or(-1.0);
    let max_x: f32 = points.iter().map(|p| p.x).reduce(f32::max).unwrap_or(1.0);
    let max_y: f32 = points.iter().map(|p| p.y).reduce(f32::max).unwrap_or(1.0);

    let mid_x = (max_x + min_x) / 2.0;
    let mid_y = (max_y + min_y) / 2.0;

    let width = max_x - min_x;
    let height = max_y - min_y;

    InstanceInput {
        matrix_col_0: [width, 0.0],
        matrix_col_1: [0.0, height],
        matrix_col_2: [mid_x * 2.0 - 1.0, -mid_y * 2.0 + 1.0],
        p0: [points[0].x, points[0].y],
        p1: [points[1].x, points[1].y],
        p2: [points[2].x, points[2].y],
        p3: [points[3].x, points[3].y],
    }
}

#[wasm_bindgen]
pub struct AppState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen]
    pub async fn new_with_init() -> AppState {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap_throw();

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        log::info!("Dimensions: {} {}", canvas_width, canvas_height);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
        });

        let surface_target = wgpu::SurfaceTarget::Canvas(canvas);
        let surface = instance
            .create_surface(surface_target)
            .expect("Failed to create surface!");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                // Use WebGL-compatible defaults instead of standard defaults
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: wgpu::MemoryHints::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create device");

        let config = surface
            .get_default_config(&adapter, canvas_width, canvas_height)
            .expect("Surface not supported by adapter");

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), InstanceInput::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instances: &[InstanceInput] = &[
            points_to_instance_input([
                glm::Vec2::new(0.5, 0.0),
                glm::Vec2::new(0.5, 0.0),
                glm::Vec2::new(0.5, 0.0),
                glm::Vec2::new(1.0, 0.5),
            ]),
            points_to_instance_input([
                glm::Vec2::new(0.375, 0.5),
                glm::Vec2::new(0.5, 0.5),
                glm::Vec2::new(0.5, 0.5),
                glm::Vec2::new(0.5, 0.625),
            ]),
            points_to_instance_input([
                glm::Vec2::new(0.5, 0.625),
                glm::Vec2::new(0.5, 0.7),
                glm::Vec2::new(0.5, 0.7),
                glm::Vec2::new(0.75, 0.875),
            ]),
        ];

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            instance_buffer,
        }
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw(0..(VERTICES.len() as u32), 0..3);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

use crate::bezier;
extern crate nalgebra_glm as glm;

mod vertex;
mod bezier_renderer;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    bezier_renderer: bezier_renderer::BezierRenderer,
}

impl Renderer {
    pub async fn new() -> Self {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap_throw();

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

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

        let bezier_renderer = bezier_renderer::BezierRenderer::new(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            bezier_renderer,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);

            self.bezier_renderer.resize(&self.queue, &self.config);
        }
    }

    pub fn begin_draw(&mut self) {
        self.bezier_renderer.begin_draw();
    }

    pub fn bezier(&mut self, bezier: &bezier::Bezier) {
        self.bezier_renderer.bezier(bezier);
    }

    pub fn end_draw(&mut self) {
        self.bezier_renderer.end_draw(&self.queue);
        self.render();
    }

    fn render(&self) {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.bezier_renderer.render(&mut encoder, &view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

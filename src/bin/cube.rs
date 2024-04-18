extern crate env_logger;
extern crate log;

use std::borrow::Cow;
use std::mem;

use bytemuck::{Pod, Zeroable};
use wgpu::{Device, Queue, RenderPipeline, Surface};
use wgpu::util::DeviceExt;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Copy, Clone, Default, Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> rust_playstation_emulator::gpu::opengl::Position {
        rust_playstation_emulator::gpu::opengl::Position {
            x: x,
            y: y,
        }
    }

    pub fn from_packed(val: u32) -> rust_playstation_emulator::gpu::opengl::Position {
        let x = val as i16;
        let y = (val >> 16) as i16;

        rust_playstation_emulator::gpu::opengl::Position { x, y }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> rust_playstation_emulator::gpu::opengl::Color {
        rust_playstation_emulator::gpu::opengl::Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn from_packed(val: u32) -> rust_playstation_emulator::gpu::opengl::Color {
        let r = val as u8;
        let g = (val >> 8) as u8;
        let b = (val >> 16) as u8;

        rust_playstation_emulator::gpu::opengl::Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

fn vertex(pos: [i8; 3], tc: [i8; 3]) -> Vertex {
    Vertex {
        position: [pos[0] as f32, pos[1] as f32, pos[2] as f32],
        color: [tc[0] as f32, tc[1] as f32, tc[2] as f32],
    }
}

fn main() {
    env_logger::builder().init();

    let event_loop = EventLoop::new().unwrap();

    let fb_x_res = 1024;
    let fb_y_res = 512;

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(fb_x_res as f64, fb_y_res as f64))
        .build(&event_loop)
        .unwrap();

    let mut app = App::from(window);

    let _ = event_loop.run(move |event, target| {
        if let Event::WindowEvent {
            window_id: _,
            event,
        } = event
        {
            match event {
                WindowEvent::RedrawRequested => {
                    app.push_triangle(&[
                        Vertex { position: [512.0, 128.0, 0.0], color: [1.0, 0.0, 0.0] },
                        Vertex { position: [256.0, 384.0, 0.0], color: [0.0, 1.0, 0.0] },
                        Vertex { position: [768.0, 384.0, 0.0], color: [0.0, 0.0, 1.0] },
                    ]);
                    app.draw();
                    // window.request_redraw();
                }
                WindowEvent::CloseRequested => target.exit(),
                _ => {}
            };
        }
    });
}


struct App {
    fb_y_res: u32,
    fb_x_res: u32,
    buffer: Vec<Vertex>,
    nvertices: u32,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    surface: Surface<'static>,
}

impl App {
    fn from(window: Window) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();
        let adapter = futures::executor::block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }))
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = futures::executor::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            ))
            .expect("Failed to create device");

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<Vertex>();


        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        }];

        // let config = surface.get_default_config(&adapter, 1024, 512).unwrap();
        // surface.configure(&device, &config);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb, // Assuming this is the swap chain format
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });



        Self {
            fb_y_res: 512,
            fb_x_res: 1024,
            device,
            queue,
            buffer: Vec::with_capacity(1024),
            nvertices: 0,
            pipeline,
            surface,
        }
    }

    pub fn push_triangle(&mut self, vertices: &[Vertex; 3]) {
        // self.buffer.push(Vertex { position: [((vertices[0].position[0] / 1024.0) * 2.0) - 1.0, 1.0 - (vertices[0].position[1] / 512.0) * 2.0, 0.0], color: vertices[0].color });
        self.buffer.push(vertices[0]);
        // self.buffer.push(Vertex { position: [((vertices[1].position[0] / 1024.0) * 2.0) - 1.0, 1.0 - (vertices[1].position[1] / 512.0) * 2.0, 0.0], color: vertices[1].color });
        self.buffer.push(vertices[1]);
        // self.buffer.push(Vertex { position: [((vertices[2].position[0] / 1024.0) * 2.0) - 1.0, 1.0 - (vertices[2].position[1] / 512.0) * 2.0, 0.0], color: vertices[2].color });
        self.buffer.push(vertices[2]);
        // self.nvertices += 3;
    }

    pub fn draw(&mut self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let vertex_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.buffer),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());


        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
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
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, vertex_buf.slice(..));
            rpass.draw(0..3, 0..1);

            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            // rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            // if let Some(ref pipe) = self.pipeline_wire {
            //     rpass.set_pipeline(pipe);
            //     rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            // }
        }

        self.queue.submit(Some(encoder.finish()));

        frame.present();
    }
}
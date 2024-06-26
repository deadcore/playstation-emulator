use std::borrow::Cow;
use std::{fmt, mem};
use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use wgpu::{Device, Queue, RenderPipeline, Surface};
use wgpu::util::DeviceExt;
use winit::{
    event_loop::EventLoop,
    window::Window,
};

/// Maximum number of vertex that can be stored in an attribute
/// buffers
const VERTEX_BUFFER_LEN: usize = 64 * 1024;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn new(pos: Position, color: Color) -> Vertex {
        Vertex {
            position: [pos.x as f32, pos.y as f32, 0.0],
            color: [color.r as f32, color.g as f32, color.b as f32],
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertex({:?}, {:?})", self.position, self.color)
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Position {
            x: x,
            y: y,
        }
    }

    pub fn from_packed(val: u32) -> Position {
        let x = val as i16;
        let y = (val >> 16) as i16;

        Position { x, y }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn from_packed(val: u32) -> Color {
        let r = val as u8;
        let g = (val >> 8) as u8;
        let b = (val >> 16) as u8;

        Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct Renderer {
    fb_y_res: u32,
    fb_x_res: u32,
    buffer: Vec<Vertex>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    surface: Surface<'static>,
    window: Arc<Window>
}

impl Renderer {
    pub fn new(_event_loop: &EventLoop<()>, window: Arc<Window>) -> Renderer {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();
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
            buffer: Vec::with_capacity(VERTEX_BUFFER_LEN),
            pipeline,
            surface,
            window: window,
        }
    }

    /// Add a triangle to the draw buffer
    pub fn push_triangle(&mut self, vertices: &[Vertex; 3]) {
        debug!("Current number of vertices in the buffer: [{} / {}]. Pushing a vertices to the frame {:?}", self.buffer.len(), VERTEX_BUFFER_LEN, vertices);


        // Make sure we have enough room left to queue the vertex. We
        // need to push two triangles to draw a quad, so 6 vertex
        if self.buffer.len() + 3 > VERTEX_BUFFER_LEN {
            debug!("The vertex attribute buffers are full, force an early draw");
            self.draw();
        }
        self.buffer.push(vertices[0]);
        self.buffer.push(vertices[1]);
        self.buffer.push(vertices[2]);
    }

    /// Add a quad to the draw buffer
    pub fn push_quad(&mut self, vertices: &[Vertex; 4]) {
        debug!("Pushing a quad to the frame {:?}", vertices);

        if self.buffer.len() + 6 > VERTEX_BUFFER_LEN {
            self.draw();
        }

        self.push_triangle(&[vertices[0], vertices[1], vertices[2]]);
        self.push_triangle(&[vertices[1], vertices[2], vertices[3]]);
    }

    /// Set the value of the uniform draw offset
    pub fn set_draw_offset(&mut self, _x: i16, _y: i16) {
        // Force draw for the primitives with the current offset
        self.draw();

//        self.uniforms = uniform! {
//            offset : [x as i32, y as i32],
//        }
    }

    /// Set the drawing area. Coordinates are offsets in the
    /// PlayStation VRAM
    pub fn set_drawing_area(&mut self,
                            left: u16, top: u16,
                            right: u16, bottom: u16) {
        // Render any pending primitives
        self.draw();

        let fb_x_res = self.fb_x_res as i32;
        let fb_y_res = self.fb_y_res as i32;

        // Scale PlayStation VRAM coordinates if our framebuffer is
        // not at the native resolution
        let left = (left as i32 * fb_x_res) / 1024;
        let right = (right as i32 * fb_x_res) / 1024;

        let top = (top as i32 * fb_y_res) / 512;
        let bottom = (bottom as i32 * fb_y_res) / 512;

        // Width and height are inclusive
        let width = right - left + 1;
        let height = bottom - top + 1;

        // OpenGL has (0, 0) at the bottom left, the PSX at the top left
        let bottom = fb_y_res - bottom - 1;

        if width < 0 || height < 0 {
            // XXX What should we do here?
            println!("Unsupported drawing area: {}x{} [{}x{}->{}x{}]",
                     width, height,
                     left, top, right, bottom);
        } else {}
    }

    /// Draw the buffered commands and reset the buffers
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
        let view = frame.texture
            .create_view(&wgpu::TextureViewDescriptor::default());


        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
            rpass.draw(0..(self.buffer.len() as u32), 0..1);

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
        self.window.request_redraw();
        // self.buffer.clear();
    }

    /// Draw the buffered commands and display them
    pub fn display(&mut self) {
        self.draw();
        debug!("Displaying the view");
    }
}


trait Block {
    fn wait(self) -> <Self as futures::Future>::Output
        where Self: Sized, Self: futures::Future
    {
        futures::executor::block_on(self)
    }
}
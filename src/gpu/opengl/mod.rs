use std::borrow::Cow;
use std::{fmt, thread};
use std::future::Future;
use futures::{executor, pin_mut};
use std::task::{Context, Poll};
use futures::executor::enter;
use futures::task::waker_ref;

use wgpu::{Device, Queue, RenderPipeline};
use wgpu::util::DeviceExt;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

/// Maximum number of vertex that can be stored in an attribute
/// buffers
const VERTEX_BUFFER_LEN: u32 = 64 * 1024;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [i16; 2],
    pub color: [u8; 3],
}

impl Vertex {
    pub fn new(pos: Position, color: Color) -> Vertex {
        Vertex {
            position: [pos.x, pos.y],
            color: [color.r, color.g, color.b],
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
    /// Current number or vertices in the buffers
    nvertices: u32,
    // swap_chain: SwapChain,
    device: Device,
    // render_pipeline: RenderPipeline,
    // queue: Queue,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>) -> Renderer {
        let fb_x_res = 1024;
        let fb_y_res = 512;

        /** Shader setup **/
        let vertex = include_str!("shader.vert.glsl");
        let fragment = include_str!("shader.frag.glsl");

        let mut compiler = shaderc::Compiler::new().unwrap();
        let mut options = shaderc::CompileOptions::new().unwrap();
        options.add_macro_definition("EP", Some("main"));

        let vertext_result = compiler
            .compile_into_spirv(
                vertex,
                shaderc::ShaderKind::Vertex,
                "shader.vert.glsl",
                "main",
                Some(&options),
            )
            .unwrap();

        let fragment_result = compiler
            .compile_into_spirv(
                fragment,
                shaderc::ShaderKind::Fragment,
                "shader.frag.glsl",
                "main",
                Some(&options),
            )
            .unwrap();

        let vs = vertext_result.as_binary_u8();
        let fs = fragment_result.as_binary_u8();

        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(fb_x_res as f64, fb_y_res as f64))
            .build(event_loop)
            .unwrap();

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = executor::block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }))
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = executor::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            ))
            .expect("Failed to create device");

        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);

        Renderer {
            fb_y_res,
            fb_x_res,
            device,
            buffer: Vec::with_capacity(VERTEX_BUFFER_LEN as usize),
            nvertices: 0,
        }
    }

    /// Add a triangle to the draw buffer
    pub fn push_triangle(&mut self, vertices: &[Vertex; 3]) {
        debug!("Current number of vertices in the buffer: [{} / {}]. Pushing a vertices to the frame {:?}", self.nvertices, VERTEX_BUFFER_LEN, vertices);


        // Make sure we have enough room left to queue the vertex. We
        // need to push two triangles to draw a quad, so 6 vertex
        if self.nvertices + 3 > VERTEX_BUFFER_LEN {
            debug!("The vertex attribute buffers are full, force an early draw");
            self.draw();
        }
        self.buffer.push(vertices[0]);
        self.buffer.push(vertices[1]);
        self.buffer.push(vertices[2]);
        self.nvertices += 3;
    }

    /// Add a quad to the draw buffer
    pub fn push_quad(&mut self, vertices: &[Vertex; 4]) {
        debug!("Pushing a quad to the frame {:?}", vertices);

        self.push_triangle(&[vertices[0], vertices[1], vertices[2]]);
        self.push_triangle(&[vertices[1], vertices[2], vertices[3]]);
    }

    /// Set the value of the uniform draw offset
    pub fn set_draw_offset(&mut self, x: i16, y: i16) {
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

        // Width And height are inclusive
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

    /// Draw the buffered commands And reset the buffers
    pub fn draw(&mut self) {
        debug!("Drawing the view");

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("primary"),
        });

        // let frame = self.swap_chain.get_next_texture();
        // let vertex_buf = self.device
        //                      .create_buffer_mapped(
        //                          self.buffer.len(),
        //                          wgpu::BufferUsage::VERTEX,
        //                      )
        //                      .fill_from_slice(&self.buffer);

        // let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        // {
        //     let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: Some("low resolution"),
        //         color_attachments: &[wgpu::RenderPassColorAttachment {
        //             view: &frame.view,
        //             resolve_target: None,
        //             load_op: wgpu::LoadOp::Clear,
        //             store_op: wgpu::StoreOp::Store,
        //             clear_color: wgpu::Color::BLACK,
        //         }],
        //         depth_stencil_attachment: None,
        //     });
        //     rpass.set_pipeline(&self.render_pipeline);
        //     rpass.set_vertex_buffers(0, &[(&vertex_buf, 0)]);
        //     rpass.draw(0..(self.buffer.len() as u32), 0..1);
        // }
        // self.queue.submit(&[encoder.finish()]);

        self.nvertices = 0;
    }

    /// Draw the buffered commands And display them
    pub fn display(&mut self) {
        self.draw();
        debug!("Displaying the view");
    }
}
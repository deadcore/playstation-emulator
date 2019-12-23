use std::fmt;

use wgpu::{Device, Queue, RenderPipeline, SwapChain};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

/// Maximum number of vertex that can be stored in an attribute
/// buffers
const VERTEX_BUFFER_LEN: u32 = 64 * 1024;

#[derive(Copy, Clone, Debug)]
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
    swap_chain: SwapChain,
    device: Device,
    render_pipeline: RenderPipeline,
    queue: Queue,
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

        let surface = wgpu::Surface::create(&window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                backends: wgpu::BackendBit::PRIMARY,
            },
        )
            .unwrap();

        let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let vs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());
        let fs_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

        let vertex_size = std::mem::size_of::<Vertex>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Short2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Uchar4,
                        offset: 2 * 2,
                        shader_location: 1,
                    },
                ],
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let mut swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: fb_x_res,
                height: fb_y_res,
                present_mode: wgpu::PresentMode::Vsync,
            },
        );


        Renderer {
            fb_y_res,
            fb_x_res,
            swap_chain,
            device,
            render_pipeline,
            queue,
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
        debug!("Drawing the view");

        let frame = self.swap_chain.get_next_texture();
        let vertex_buf = self.device
                             .create_buffer_mapped(
                                 self.buffer.len(),
                                 wgpu::BufferUsage::VERTEX,
                             )
                             .fill_from_slice(&self.buffer);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_vertex_buffers(0, &[(&vertex_buf, 0)]);
            rpass.draw(0..(self.buffer.len() as u32), 0..1);
        }
        self.queue.submit(&[encoder.finish()]);

        self.nvertices = 0;
    }

    /// Draw the buffered commands and display them
    pub fn display(&mut self) {
        self.draw();
        debug!("Displaying the view");
    }
}

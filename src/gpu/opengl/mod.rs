use std::fmt;

use glium::{DrawParameters, Frame, Program, Rect, Surface, VertexBuffer};
use glium::debug::DebugCallbackBehavior;
use glium::uniforms::{EmptyUniforms, UniformsStorage};
use glium_sdl2;
use glium_sdl2::DisplayBuild;
use sdl2;
use sdl2::video::GLProfile;

/// Maximum number of vertex that can be stored in an attribute
/// buffers
const VERTEX_BUFFER_LEN: u32 = 64 * 1024;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [i16; 2],
    pub color: [u8; 3],
}

implement_vertex!(Vertex, position, color);

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

        Position {
            x: x,
            y: y,
        }
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
    /// Glium display
    window: glium_sdl2::SDL2Facade,
    /// Glium surface,
    target: Option<Frame>,
    /// Framebuffer horizontal resolution (native: 1024)
    fb_x_res: u16,
    /// Framebuffer vertical resolution (native: 512)
    fb_y_res: u16,
    /// OpenGL Program object
    program: Program,
    /// Permanent vertex buffer
    vertex_buffer: VertexBuffer<Vertex>,
    /// GLSL uniforms
    uniforms: UniformsStorage<'static, [i32; 2], EmptyUniforms>,
    /// Glium draw parameters
    params: DrawParameters<'static>,
    /// Current number or vertices in the buffers
    nvertices: u32,
}

impl Renderer {
    pub fn new(sdl_context: &sdl2::Sdl) -> Renderer {
        use glium_sdl2::DisplayBuild;
        // Native PSX VRAM resolution
        let fb_x_res = 1024u16;
        let fb_y_res = 512u16;

        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        // XXX Debug context is likely to be slower, we should make
        // that configurable at some point.
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window = video_subsystem
            .window("Rustation", fb_x_res as u32, fb_y_res as u32)
            .position_centered()
            .resizable()
            .build()
            .ok()
            .expect("Can't create SDL2 window");


        {
            let mut target = window.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target.finish().unwrap();
        }
        // "Slurp" the contents of the shader files. Note: this is a
        // compile-time thing.
        let vs_src = include_str!("vertex.glsl");
        let fs_src = include_str!("fragment.glsl");

        let program = Program::from_source(&window, vs_src, fs_src, None).unwrap();

        let vertex_buffer = VertexBuffer::empty_persistent(&window,
                                                           VERTEX_BUFFER_LEN as usize).unwrap();

        let uniforms = uniform! {
            offset: [0; 2],
        };

        let params = DrawParameters {
            // Default to full screen
            scissor: Some(Rect {
                left: 0,
                bottom: 0,
                width: fb_x_res as u32,
                height: fb_y_res as u32,
            }),
            ..Default::default()
        };

        Renderer {
            target: Some(window.draw()),
            window: window,
            fb_x_res: fb_x_res,
            fb_y_res: fb_y_res,
            program: program,
            vertex_buffer: vertex_buffer,
            uniforms: uniforms,
            params: params,
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

        let slice = self.vertex_buffer.slice(self.nvertices as usize..(self.nvertices + 3) as usize).unwrap();
        slice.write(vertices);
        self.nvertices += 3;
    }

    /// Add a quad to the draw buffer
    pub fn push_quad(&mut self, vertices: &[Vertex; 4]) {
        debug!("Pushing a quad to the frame {:?}", vertices);

        // XXX Doesn't work, le slice retourne un [Vertex]
        //// Push the first triangle
        //self.push_triangle(&vertices[0..3]);
        //// Push the 2nd triangle
        //self.push_triangle(&vertices[1..4]);
        self.push_triangle(&[vertices[0], vertices[1], vertices[2]]);
        self.push_triangle(&[vertices[1], vertices[2], vertices[3]]);
    }

    /// Set the value of the uniform draw offset
    pub fn set_draw_offset(&mut self, x: i16, y: i16) {
        // Force draw for the primitives with the current offset
        self.draw();

        self.uniforms = uniform! {
            offset : [x as i32, y as i32],
        }
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
            self.params.scissor = Some(Rect {
                left: 0,
                bottom: 0,
                width: 0,
                height: 0,
            });
        } else {
            self.params.scissor = Some(Rect {
                left: left as u32,
                bottom: bottom as u32,
                width: width as u32,
                height: height as u32,
            });
        }
    }

    /// Draw the buffered commands and reset the buffers
    pub fn draw(&mut self) {
        debug!("Drawing the view");

        use glium::index;

        self.target
            .as_mut()
            .unwrap()
            .draw(self.vertex_buffer.slice(0..self.nvertices as usize).unwrap(),
                  &index::NoIndices(index::PrimitiveType::TrianglesList),
                  &self.program,
                  &self.uniforms,
                  &self.params)
            .unwrap();

        // Reset the buffers
        self.nvertices = 0;
    }

    /// Draw the buffered commands and display them
    pub fn display(&mut self) {
        debug!("Displaying the view");
        {
            let target = self.target.take().unwrap();
            target.finish().unwrap();
        }

        self.target = Some(self.window.draw());
    }
}


impl Drop for Renderer {
    fn drop(&mut self) {
        if let Some(frame) = self.target.take() {
            frame.finish().unwrap();
        }
    }
}

use sdl2;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};

use crate::gpu::vertex::Vertex;

//use libc::c_void;

pub struct Renderer {
    events: sdl2::EventPump,
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(title: &'static str, width: u32, height: u32, fps_limit: u32) -> Result<Renderer, String> {
        let sdl = sdl2::init()?;
        let vid_s = sdl.video()?;
        let events = sdl.event_pump()?;

        let window = vid_s.window(title, width, height)
                          .position_centered()
                          .build()
                          .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Renderer {
            events,
            canvas,
        })
    }

    pub fn push_triangle(&mut self, verticies: &[Vertex; 3]) {
        info!("Pushing triangles")
    }

    pub fn draw(&mut self) {
        info!("Drawing");

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}
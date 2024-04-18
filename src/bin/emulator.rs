extern crate env_logger;
extern crate log;

use std::env;
use std::path::Path;
use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use rust_playstation_emulator::bios::Bios;
use rust_playstation_emulator::cpu::Cpu;
use rust_playstation_emulator::cpu::interconnect::Interconnect;
use rust_playstation_emulator::gpu::Gpu;
use rust_playstation_emulator::gpu::opengl::Renderer;
use rust_playstation_emulator::memory::ram::Ram;

fn main() {
    env_logger::builder()
//        .default_format_level(false)
//        .default_format_module_path(false)
//        .default_format_timestamp(false)
        .init();

    let bios_filepath = match env::args().nth(1) {
        Some(x) => x,
        None => panic!("usage: rpsx.exe rom game")
    };

    let event_loop = EventLoop::new().unwrap();

    let fb_x_res = 1024;
    let fb_y_res = 512;

    let window = Arc::new(WindowBuilder::new()
        .with_inner_size(LogicalSize::new(fb_x_res as f64, fb_y_res as f64))
        .build(&event_loop)
        .unwrap());

    let display = Renderer::new(&event_loop, window.clone());

    let bios = Bios::new(&Path::new(&bios_filepath)).unwrap();
    let ram = Ram::new();
    let gpu = Gpu::new(display);

    let inter = Interconnect::new(
        bios,
        ram,
        gpu,
    );
    let mut cpu = Cpu::new(inter);

    let mut running = true;

    let _ = event_loop.run(move |event, target| {
        if let Event::WindowEvent {
            window_id: _,
            event,
        } = event
        {
            match event {
                WindowEvent::RedrawRequested => {
                    while running {
                    for i in 0..1_000_000 {
                        cpu.run_next_instruction();
                        // window.request_redraw()
                    }
                    }
                }
                WindowEvent::CloseRequested => target.exit(),
                _ => {}
            };
        }
    });
}

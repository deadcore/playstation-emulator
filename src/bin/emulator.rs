extern crate env_logger;
extern crate log;

use std::env;
use std::path::Path;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

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

    // We must initialize SDL before the interconnect is created since
    // it contains the GPU and the GPU needs to create a window
    let sdl_context = sdl2::init().unwrap();

    let display = Renderer::new(&sdl_context);

    let bios = Bios::new(&Path::new(&bios_filepath)).unwrap();
    let ram = Ram::new();
    let gpu = Gpu::new(display);

    let inter = Interconnect::new(
        bios,
        ram,
        gpu,
    );
    let mut cpu = Cpu::new(inter);
    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        for _ in 0..1_000_000 {
            cpu.run_next_instruction();
        }

        // See if we should quit
        for e in event_pump.poll_iter() {
            match e {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
                Event::Quit { .. } => return,
                _ => (),
            }
        }
    }
}

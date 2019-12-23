extern crate env_logger;
extern crate log;

use std::env;
use std::path::Path;

use shaderc;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

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

    let event_loop = EventLoop::new();

    let display = Renderer::new(&event_loop);

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::KeyboardInput {
                    input:
                    event::KeyboardInput {
                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                        state: event::ElementState::Pressed,
                        ..
                    },
                    ..
                } | event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            event::Event::EventsCleared => {
                while running {
                    for _ in 0..1_000_000 {
                        cpu.run_next_instruction();
                    }
                }
            }
            _ => (),
        }
    });
}

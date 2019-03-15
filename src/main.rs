extern crate log;
extern crate env_logger;

use std::path::Path;
use std::env;
use rust_playstation_emulator::cpu::Cpu;
use rust_playstation_emulator::cpu::interconnect::Interconnect;
use rust_playstation_emulator::bios::Bios;

fn main() {
    env_logger::init();

    let bios_filepath = match env::args().nth(1) {
        Some(x) => x,
        None => {
            panic!("usage: rpsx.exe rom game");
        }
    };

    let bios = Bios::new(&Path::new(&bios_filepath)).unwrap();
    let inter = Interconnect::new(bios);
    let mut cpu = Cpu::new(inter);
    loop {
        cpu.run_next_instruction();
    }
}
extern crate env_logger;
extern crate log;

use std::env;
use std::path::Path;

use rust_playstation_emulator::bios::Bios;
use rust_playstation_emulator::cpu::Cpu;
use rust_playstation_emulator::cpu::interconnect::Interconnect;
use rust_playstation_emulator::ram::Ram;

fn main() {
    env_logger::init();

    let bios_filepath = match env::args().nth(1) {
        Some(x) => x,
        None => panic!("usage: rpsx.exe rom game")
    };

    let bios = Bios::new(&Path::new(&bios_filepath)).unwrap();
    let ram = Ram::new();
    let inter = Interconnect::new(
        bios,
        ram,
    );
    let mut cpu = Cpu::new(inter);

    loop {
        cpu.run_next_instruction();
    }
}
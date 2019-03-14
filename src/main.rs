#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::{Result, Error, ErrorKind};
use std::env;

fn main() {

    let bios_filepath = match env::args().nth(1) {
        Some(x) => x,
        None => {
            error!("usage: rpsx.exe rom game");
            return;
        }
    };

    let bios = Bios::new(&Path::new(&bios_filepath)).unwrap();
    let inter = Interconnect::new(bios);
    let mut cpu = Cpu::new(inter);
    loop {
        cpu.run_next_instruction();
    }
}


/// CPU state
pub struct Cpu {
    /// The program counter register
    pc: u32,

    /// General purpose registers
    /// The first entry must always contain 0
    regs: [u32; 32],

    /// Memory interface
    inter: Interconnect,
}


impl Cpu {
    pub fn new(inter: Interconnect) -> Cpu {
        let mut regs = [0xdeadbeef; 32];
        // R0 is hardwired to 0
        regs[0] = 0;

        Cpu {
            pc: 0xbfc00000,
            regs,
            inter,
        }
    }

    fn reg(&self, index: u32) -> u32 {
        self.regs[index as usize]
    }

    fn set_reg(&mut self, index: u32, val: u32) {
        self.regs[index as usize] = val;
        // Make sure R0 is always 0
        self.regs[0] = 0;
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.pc;

        // Fetch instruction at PC
        let instruction = Instruction(self.load32(pc));

        // Increment PC to point to the next instruction.
        // Wrapping add means that we want the PC to wrap back to 0 in case of an overflow (i.e. 0xfffffffc + 4 => 0x00000000)
        self.pc = pc.wrapping_add(4);

        self.decode_and_execute(instruction);
    }

    /// Load 32bit value from the interconnect
    fn load32(&self, addr: u32) -> u32 {
        self.inter.load32(addr)
    }

    fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.function() {
            0b001111 => self.op_lui(instruction),
            _ => panic!("Unhandled instruction {:08x}", instruction.0)
        }
    }

    fn op_lui(&mut self, instruction: Instruction) {
        debug!("[{:08x}]", instruction.function());
        let i = instruction.imm();
        let t = instruction.t();

        let v = i << 16;

        self.set_reg(t, v);
    }

}

/// BIOS image
pub struct Bios {
    /// BIOS memory
    data: Vec<u8>
}

/// BIOS images are always 512KB in length
const BIOS_SIZE: u64 = 512 * 1024;

impl Bios {
    /// Load a BIOS image from the file located at ‘path‘
    pub fn new(path: &Path) -> Result<Bios> {
        let file = File::open(path)?;
        let mut data = Vec::new();
        // Load the BIOS
        file.take(BIOS_SIZE).read_to_end(&mut data)?;
        if data.len() == BIOS_SIZE as usize {
            Ok(Bios {
                data
            })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid BIOS size"))
        }
    }

    /// Fetch the 32bit little endian word at ‘offset ‘
    pub fn load32(&self, offset: u32) -> u32 {
        let offset = offset as usize;
        let b0 = self.data[offset + 0] as u32;
        let b1 = self.data[offset + 1] as u32;
        let b2 = self.data[offset + 2] as u32;
        let b3 = self.data[offset + 3] as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
}

/// Global interconnect
pub struct Interconnect {
    /// Basic Input/Output memory
    bios: Bios,
}

impl Interconnect {
    pub fn new(bios: Bios) -> Interconnect {
        Interconnect {
            bios,
        }
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if let Some(offset) = map::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }
        panic!("unhandled fetch32 at address {:08x}", addr);
    }
}

mod map {
    pub struct Range(u32, u32);

    impl Range {
        /// Return ‘Some(offset)‘ if addr is contained in ‘self ‘
        pub fn contains(self, addr: u32) -> Option<u32> {
            let Range(start, length) = self;
            if addr >= start && addr < start + length {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const BIOS: Range = Range(0xbfc00000, 512 * 1024);
}

/// Simple wrapper around an instruction word to provide type-safety.
#[derive(Clone, Copy)]
struct Instruction(u32);

impl Instruction {
    /// Return bits [31:26] of the instruction
    fn function(self) -> u32 {
        let Instruction(op) = self;
        op >> 26
    }

    /// Return register index in bits [20:16]
    fn t(self) -> u32 {
        let Instruction(op) = self;
        (op >> 16) & 0x1f
    }

    /// Return immediate value in bits [16:0]
    fn imm(self) -> u32 {
        let Instruction(op) = self;
        op & 0xffff
    }
}
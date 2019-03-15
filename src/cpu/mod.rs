extern crate log;
extern crate env_logger;

pub mod interconnect;

use crate::instruction::Instruction;
use self::interconnect::Interconnect;

pub const REGISTERS: [&str; 32] = [
    "$zero",
    "$at",
    "$v0", "$v1",
    "$a0", "$a1", "$a2", "$a3",
    "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7",
    "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7",
    "$t8", "$t9",
    "$k0", "$k1",
    "$gp",
    "$sp",
    "$fp",
    "$ra"
];

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

    fn store32(&mut self, addr: u32, val: u32) {
        self.inter.store32(addr, val);
    }

    fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.function() {
            0b000000 => match instruction.subfunction() {
                0b000000 => self.op_sll(instruction),
                _ => panic!("Unhandled instruction {:08x}", instruction.0),
            },
            0b001111 => self.op_lui(instruction),
            0b001101 => self.op_ori(instruction),
            0b101011 => self.op_sw(instruction),
            _ => panic!("Unhandled instruction 0x{:08x}", instruction.0)
        }
    }

    fn op_sll(&mut self, instruction: Instruction) {
        debug!("LUI {}, 0x{:04x}", REGISTERS[instruction.t() as usize], instruction.shift());

        let i = instruction.shift();
        let t = instruction.t();
        let d = instruction.d();

        let v = self.reg(t) << i;

        self.set_reg(d, v)
    }

    fn op_lui(&mut self, instruction: Instruction) {
        debug!("LUI {}, 0x{:04x}", REGISTERS[instruction.t() as usize], instruction.imm());

        let i = instruction.imm();
        let t = instruction.t();

        let v = i << 16;

        self.set_reg(t, v);
    }

    fn op_ori(&mut self, instruction: Instruction) {
        debug!("ORI {}, {}, 0x{:04x}", REGISTERS[instruction.t() as usize], REGISTERS[instruction.s() as usize], instruction.imm());

        let i = instruction.imm();
        let t = instruction.t();
        let s = instruction.s();
        let v = self.reg(s) | i;
        self.set_reg(t, v);
    }

    /// Store word
    fn op_sw(&mut self, instruction: Instruction) {
        debug!("SW {}, 0x{:04x}({})", REGISTERS[instruction.t() as usize], instruction.imm_se(), REGISTERS[instruction.s() as usize]);

        let i = instruction.imm_se();
        let t = instruction.t();
        let s = instruction.s();

        let addr = self.reg(s).wrapping_add(i);
        let v = self.reg(t);

        self.store32(addr, v)
    }
}
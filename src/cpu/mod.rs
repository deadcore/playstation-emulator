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

    /// Next instruction to be executed , used to simulate the branch delay slot
    next_instruction: Instruction,

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
            next_instruction: Instruction(0x0),
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
        self.dump_registers();

        let pc = self.pc;
        let instruction = self.next_instruction;
        self.next_instruction = Instruction(self.load32(pc));

        // Increment PC to point to the next instruction.
        // Wrapping add means that we want the PC to wrap back to 0 in case of an overflow (i.e. 0xfffffffc + 4 => 0x00000000)
        self.pc = pc.wrapping_add(4);

        self.decode_and_execute(instruction);
    }

    fn dump_registers(&self) {
        trace!("$pc: 0x{:08x}", self.pc);

        self.regs.iter().zip(REGISTERS.iter_mut()).for_each(|(value, register)| {
            trace!("{} = 0x{:08x}", register, value)
        });
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
            0b001001 => self.op_aaddiu(instruction),
            0b000010 => self.op_j(instruction),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    // Operations

    /// Jump
    fn op_j(&mut self, instruction: Instruction) {
        debug!("J 0x{:04x}", (self.pc & 0xf000_0000) | (instruction.imm_jump() << 2));

        let i = instruction.imm_jump();

        self.pc = (self.pc & 0xf0000000) | (i << 2);
    }

    fn op_aaddiu(&mut self, instruction: Instruction) {
        let i = instruction.shift();
        let t = instruction.t();
        let s = instruction.s();

        let v = self.reg(s).wrapping_add(i);

        debug!("ADDIU {}, {}, 0x{:04x}", REGISTERS[t as usize], REGISTERS[s as usize], i);

        self.set_reg(t, v)
    }

    fn op_sll(&mut self, instruction: Instruction) {
        let i = instruction.shift();
        let t = instruction.t();
        let d = instruction.d();

        let v = self.reg(t) << i;

        debug!("SLL {}, {}, {}", REGISTERS[d as usize], REGISTERS[t as usize], i);

        self.set_reg(d, v)
    }

    /// Load Upper Immediate
    fn op_lui(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();

        let v = i << 16;

        debug!("LUI {}, 0x{:04x}", REGISTERS[t as usize], i);

        self.set_reg(t, v);
    }

    fn op_ori(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();
        let s = instruction.s();
        let v = self.reg(s) | i;

        debug!("ORI {}, {}, 0x{:04x}", REGISTERS[t as usize], REGISTERS[s as usize], i);

        self.set_reg(t, v);
    }

    /// Store word
    fn op_sw(&mut self, instruction: Instruction) {
        let i = instruction.imm_se();
        let t = instruction.t();
        let s = instruction.s();

        let addr = self.reg(s).wrapping_add(i);
        let v = self.reg(t);

        debug!("SW {}, 0x{:04x}({})", REGISTERS[t as usize], i, REGISTERS[s as usize]);

        self.store32(addr, v)
    }
}
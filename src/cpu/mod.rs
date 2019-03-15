extern crate log;
extern crate env_logger;

pub mod interconnect;

use crate::instruction::{Instruction, RegisterIndex, REGISTERS};
use self::interconnect::Interconnect;

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

    /// Cop0 register 12: Status Register
    sr: u32,
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
            sr: 0,
        }
    }

    fn reg(&self, index: RegisterIndex) -> u32 {
        self.regs[index.to_usize()]
    }

    fn set_reg(&mut self, index: RegisterIndex, val: u32) {
        self.regs[index.to_usize()] = val;
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
                0b100101 => self.op_or(instruction),
                _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.subfunction())
            },
            0b001111 => self.op_lui(instruction),
            0b001101 => self.op_ori(instruction),
            0b101011 => self.op_sw(instruction),
            0b001001 => self.op_aaddiu(instruction),
            0b000010 => self.op_j(instruction),
            0b010000 => self.op_cop0(instruction),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    // Operations

    fn op_mtc0(&mut self, instruction: Instruction) {
        let cpu_r = instruction.t();
        let RegisterIndex(cop_r) = instruction.d();

        let v = self.reg(cpu_r);

        debug!("MTC0 {}, cop0_{}", cpu_r.name(), cop_r);

        match cop_r {
            12 => self.sr = v,
            n => panic!("Unhandled cop0 register: {:08x}", n)
        }
    }

    /// Coprocessor 0 opcode
    fn op_cop0(&mut self, instruction: Instruction) {
        match instruction.cop_opcode() {
            0b000100 => self.op_mtc0(instruction),
            _ => panic!("unhandled cop0 instruction [{:#08b}]", instruction.cop_opcode())
        }
    }

    /// Jump
    fn op_j(&mut self, instruction: Instruction) {
        debug!("J 0x{:04x}", (self.pc & 0xf000_0000) | (instruction.imm_jump() << 2));

        let i = instruction.imm_jump();

        self.pc = (self.pc & 0xf0000000) | (i << 2);
    }

    /// Add Immediate Unsigned
    fn op_aaddiu(&mut self, instruction: Instruction) {
        let i = instruction.shift();
        let t = instruction.t();
        let s = instruction.s();

        let v = self.reg(s).wrapping_add(i);

        debug!("ADDIU {}, {}, 0x{:04x}", t.name(), s.name(), i);

        self.set_reg(t, v)
    }

    /// Shift Left Logical
    fn op_sll(&mut self, instruction: Instruction) {
        let i = instruction.shift();
        let t = instruction.t();
        let d = instruction.d();

        let v = self.reg(t) << i;

        debug!("SLL {}, {}, {}", d.name(), t.name(), i);

        self.set_reg(d, v)
    }

    /// Load Upper Immediate
    fn op_lui(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();

        let v = i << 16;

        debug!("LUI {}, 0x{:04x}", t.name(), i);

        self.set_reg(t, v);
    }

    /// Bitwise Or
    fn op_or(&mut self, instruction: Instruction) {
        let d = instruction.d();
        let s = instruction.s();
        let t = instruction.t();

        let v = self.reg(s) | self.reg(t);

        debug!("OR {}, {}, {}", d.name(), s.name(), t.name());

        self.set_reg(d, v);
    }

    /// Bitwise Or Immediate
    fn op_ori(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();
        let s = instruction.s();
        let v = self.reg(s) | i;

        debug!("ORI {}, {}, 0x{:04x}", t.name(), s.name(), i);

        self.set_reg(t, v);
    }

    /// Store word
    fn op_sw(&mut self, instruction: Instruction) {
        if self.sr & 0x10000 != 0 {
            // Cache is isolated , ignore write
            warn!("ignoring store while cache is isolated");
            return;
        }

        let i = instruction.imm_se();
        let t = instruction.t();
        let s = instruction.s();

        let addr = self.reg(s).wrapping_add(i);
        let v = self.reg(t);

        debug!("SW {}, 0x{:04x}({})", t.name(), i, s.name());

        self.store32(addr, v)
    }
}
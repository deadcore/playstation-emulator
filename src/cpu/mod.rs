extern crate log;
extern crate env_logger;

use prettytable::Table;

pub mod interconnect;

use crate::debugger::Debugger;

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

    /// 2nd set of registers used to emulate the load delay slot
    /// accurately. They contain the output of the current instruction.
    out_regs: [u32; 32],

    /// Load initiated by the current instruction
    load: (RegisterIndex, u32),
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
            out_regs: regs,
            load: (RegisterIndex(0), 0),
            sr: 0,
        }
    }

    fn reg(&self, index: RegisterIndex) -> u32 {
        self.regs[index.to_usize()]
    }

    fn set_reg(&mut self, index: RegisterIndex, val: u32) {
        self.out_regs[index.to_usize()] = val;
        // Make sure R0 is always 0
        self.out_regs[0] = 0;
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.pc;
        let instruction = self.next_instruction;
        self.next_instruction = Instruction(self.load32(pc));

        // Increment PC to point to the next instruction.
        // Wrapping add means that we want the PC to wrap back to 0 in case of an overflow (i.e. 0xfffffffc + 4 => 0x00000000)
        // Increment PC to point to the next instruction . All
        // instructions are 32bit long.
        self.pc = pc + 4;

        let (reg, val) = self.load;
        self.set_reg(reg, val);

        // We reset the load to target register 0 for the next
        // instruction
        self.load = (RegisterIndex(0), 0);

        self.decode_and_execute(instruction);

        // Copy the output registers as input for the
        // next instruction
        self.regs = self.out_regs;

    }

//    fn dump_registers(&self) {
//        // Create the table
//        let mut table = Table::new();
//
//        // Add a row per time
//        table.set_titles(row!["Register", "Value"]);
//        table.add_row(row!["$pc", format!("0x{:08x}", self.pc)]);
//
//        self.regs.iter().zip(REGISTERS.iter_mut()).for_each(|(value, register)| {
//            table.add_row(row![register, format!("0x{:08x}", value)]);
//        });
//
//        trace!("Dumping registers: \n{}", table.to_string());
//    }

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
            0b000101 => self.op_bne(instruction),
            0b001000 => self.op_addi(instruction),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    /// Branch to immediate value 'offset'
    fn branch(&mut self, offset: u32) {
        let offset = offset << 2;

        let mut pc = self.pc;

        pc = pc.wrapping_add(offset);

        // We need to compensate for the hardcoded
        // ‘pc.wrapping add(4) ‘ in ‘run next instruction ‘
        pc = pc.wrapping_sub(4);

        self.pc = pc;
    }

    // Operations

    fn op_lw(&mut self, instruction: Instruction) {
//        if self.sr & 0x10000 != 0 { // Cache is isolated , ignore write
//            println!("Ignoring load while cache is isolated");
//            return;
//        }

        let i = instruction.imm_se();
        let t = instruction.t();
        let s = instruction.s();

        let addr = self.reg(s).wrapping_add(i);
        let v = self.load32(addr);
        self.set_reg(t, v);
    }

    /// Add Immediate Unsigned and check for overflow
    fn op_addi(&mut self, instruction: Instruction) {
        let i = instruction.imm_se() as i32;
        let t = instruction.t();
        let s = instruction.s();

        debug!("ADDI {}, {}, {}", t.name(), s.name(), instruction.imm_se());

        let s = self.reg(s) as i32;

        let v = match s.checked_add(i) {
            Some(v) => v as u32,
            None => panic!("ADDI overflow"),
        };

        self.set_reg(t, v);
    }

    /// Branch if Not Equal
    fn op_bne(&mut self, instruction: Instruction) {
        let i = instruction.imm_se();
        let s = instruction.s();
        let t = instruction.t();

        debug!("BNE {}, {}, 0x{:08x}", s.name(), t.name(), i);


        if self.reg(s) != self.reg(t) {
            self.branch(i);
        }
    }

    /// MTC Code
    fn op_mtc0(&mut self, instruction: Instruction) {
        let cpu_r = instruction.t();
        let cop_r = instruction.d().0;

        debug!("MTC0 {}, cop0_{}", cpu_r.name(), cop_r);

        let v = self.reg(cpu_r);

        match cop_r {
            3 | 5 | 6 | 7 | 9 | 11 => // Breakpoints registers
                if v != 0 {
                    panic!("Unhandled write to cop0r{}: {:08x}", cop_r, v)
                },
            12 => self.sr = v,
            _ => panic!("Unhandled cop0 register {}", cop_r),
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
        let i = instruction.imm_se();
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
        let i = instruction.imm_se();
        let t = instruction.t();
        let s = instruction.s();

        debug!("SW {}, 0x{:04x}({})", t.name(), i, s.name());

        if self.sr & 0x10000 != 0 {
            // Cache is isolated , ignore write
            warn!("ignoring store while cache is isolated");
            return;
        }

        let addr = self.reg(s).wrapping_add(i);
        let v = self.reg(t);

        self.store32(addr, v)
    }
}
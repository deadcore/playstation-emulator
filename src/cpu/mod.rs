extern crate env_logger;
extern crate log;

use crate::cpu::delay::Delay;
use crate::cpu::operations::add::Add;
use crate::cpu::operations::addi::Addi;
use crate::cpu::operations::addiu::*;
use crate::cpu::operations::addu::Addu;
use crate::cpu::operations::and::And;
use crate::cpu::operations::andi::Andi;
use crate::cpu::operations::beq::Beq;
use crate::cpu::operations::bgtz::Bqtz;
use crate::cpu::operations::bltz::Bltz;
use crate::cpu::operations::bne::*;
use crate::cpu::operations::bxx::Bxx;
use crate::cpu::operations::div::Div;
use crate::cpu::operations::j::*;
use crate::cpu::operations::jal::Jal;
use crate::cpu::operations::jalr::Jarl;
use crate::cpu::operations::jr::Jr;
use crate::cpu::operations::lb::Lb;
use crate::cpu::operations::lbu::Lbu;
use crate::cpu::operations::lui::*;
use crate::cpu::operations::lw::Lw;
use crate::cpu::operations::mfc0::Mfc0;
use crate::cpu::operations::mflo::Mflo;
use crate::cpu::operations::mtc0::*;
use crate::cpu::operations::Operation;
use crate::cpu::operations::or::*;
use crate::cpu::operations::ori::*;
use crate::cpu::operations::sb::Sb;
use crate::cpu::operations::sh::Sh;
use crate::cpu::operations::sll::*;
use crate::cpu::operations::slti::Slti;
use crate::cpu::operations::sltu::Sltu;
use crate::cpu::operations::sra::Sra;
use crate::cpu::operations::subu::Subu;
use crate::cpu::operations::sw::*;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};

use self::interconnect::Interconnect;

pub mod interconnect;
pub mod registers;
pub mod delay;
pub mod operations;

/// CPU state
pub struct Cpu {
    registers: Registers,

    /// Memory interface
    interconnect: Interconnect,

    /// Next instruction to be executed , used to simulate the branch delay slot
    next_instruction: Instruction,

    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    load: Delay,
}


impl Cpu {
    pub fn new(interconnect: Interconnect) -> Cpu {
        Cpu {
            registers: Registers::new(),
            interconnect,
            next_instruction: Instruction(0x0),
            load: Delay::new(),
        }
    }

    pub fn reg(&self, index: RegisterIndex) -> u32 {
        self.registers.reg(index)
    }

    pub fn set_reg(&mut self, index: RegisterIndex, val: u32) {
        self.registers.set_reg(index, val)
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.registers.pc();
        let instruction = self.next_instruction;
        self.next_instruction = Instruction(self.load32(pc));

        self.registers.increment_pc();

        let (reg, val) = self.load.value();
        self.set_reg(reg, val);

        // We reset the load to target register 0 for the next
        // instruction
        self.load.reset();

        self.decode_and_execute(instruction);

        self.registers.swap_registers();
    }

    /// Load 32bit value from the interconnect
    fn load32(&self, addr: u32) -> u32 {
        self.interconnect.load32(addr)
    }


    fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.function() {
            0b000000 => self.decode_and_execute_sub_function(instruction),
            0b001111 => self.execute_operation(Lui::new(instruction)),
            0b001101 => self.execute_operation(Ori::new(instruction)),
            0b101011 => self.execute_operation(Sw::new(instruction)),
            0b001001 => self.execute_operation(Addiu::new(instruction)),
            0b010000 => self.decode_and_execute_cop0(instruction),
            0b000010 => self.execute_operation(J::new(instruction)),
            0b000101 => self.execute_operation(Bne::new(instruction)),
            0b001000 => self.execute_operation(Addi::new(instruction)),
            0b100011 => self.execute_operation(Lw::new(instruction)),
            0b101001 => self.execute_operation(Sh::new(instruction)),
            0b000011 => self.execute_operation(Jal::new(instruction)),
            0b001100 => self.execute_operation(Andi::new(instruction)),
            0b101000 => self.execute_operation(Sb::new(instruction)),
            0b100000 => self.execute_operation(Lb::new(instruction)),
            0b000100 => self.execute_operation(Beq::new(instruction)),
            0b000111 => self.execute_operation(Bqtz::new(instruction)),
            0b000110 => self.execute_operation(Bltz::new(instruction)),
            0b100100 => self.execute_operation(Lbu::new(instruction)),
            0b000001 => self.execute_operation(Bxx::new(instruction)),
            0b001010 => self.execute_operation(Slti::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    fn decode_and_execute_sub_function(&mut self, instruction: Instruction) {
        match instruction.subfunction() {
            0b000000 => self.execute_operation(Sll::new(instruction)),
            0b100101 => self.execute_operation(Or::new(instruction)),
            0b101011 => self.execute_operation(Sltu::new(instruction)),
            0b100001 => self.execute_operation(Addu::new(instruction)),
            0b001000 => self.execute_operation(Jr::new(instruction)),
            0b100100 => self.execute_operation(And::new(instruction)),
            0b100000 => self.execute_operation(Add::new(instruction)),
            0b001001 => self.execute_operation(Jarl::new(instruction)),
            0b100011 => self.execute_operation(Subu::new(instruction)),
            0b000011 => self.execute_operation(Sra::new(instruction)),
            0b011010 => self.execute_operation(Div::new(instruction)),
            0b010010 => self.execute_operation(Mflo::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Sub function call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Coprocessor 0 opcode
    fn decode_and_execute_cop0(&mut self, instruction: Instruction) {
        match instruction.cop_opcode() {
            0b000100 => self.execute_operation(Mtc0::new(instruction)),
            0b000000 => self.execute_operation(Mfc0::new(instruction)),
            _ => panic!("Unhandled cop0 instruction [0x{:08x}]. Cop0 call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    fn execute_operation(&mut self, op: impl Operation) {
        debug!("[0x{:08x}] {}", self.registers.pc(), op.gnu());

        op.perform(&mut self.registers, &mut self.interconnect, &mut self.load)
    }
}
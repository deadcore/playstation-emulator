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
use crate::cpu::operations::divu::Divu;
use crate::cpu::operations::j::*;
use crate::cpu::operations::jal::Jal;
use crate::cpu::operations::jalr::Jarl;
use crate::cpu::operations::jr::Jr;
use crate::cpu::operations::lb::Lb;
use crate::cpu::operations::lbu::Lbu;
use crate::cpu::operations::lui::*;
use crate::cpu::operations::lw::Lw;
use crate::cpu::operations::mfc0::Mfc0;
use crate::cpu::operations::mfhi::Mfhi;
use crate::cpu::operations::mflo::Mflo;
use crate::cpu::operations::mtc0::*;
use crate::cpu::operations::mtlo::Mtlo;
use crate::cpu::operations::Operation;
use crate::cpu::operations::or::*;
use crate::cpu::operations::ori::*;
use crate::cpu::operations::sb::Sb;
use crate::cpu::operations::sh::Sh;
use crate::cpu::operations::sll::*;
use crate::cpu::operations::slt::Slt;
use crate::cpu::operations::slti::Slti;
use crate::cpu::operations::sltiu::Sltiu;
use crate::cpu::operations::sltu::Sltu;
use crate::cpu::operations::sra::Sra;
use crate::cpu::operations::srl::Srl;
use crate::cpu::operations::subu::Subu;
use crate::cpu::operations::sw::*;
use crate::cpu::operations::syscall::Syscall;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Word;

use self::interconnect::Interconnect;

pub mod interconnect;
pub mod registers;
pub mod delay;
pub mod operations;
pub mod exception;

/// CPU state
pub struct Cpu {
    registers: Registers,

    /// Memory interface
    interconnect: Interconnect,

    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    load: Delay,
}


impl Cpu {
    pub fn new(interconnect: Interconnect) -> Cpu {
        Cpu {
            registers: Registers::new(),
            interconnect,
            load: Delay::new(),
        }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.registers.pc();
        let instruction = Instruction(self.interconnect.load::<Word>(pc));

        // Save the address of the current instruction to save in
        // ‘EPC‘ in case of an exception.
        self.registers.set_current_pc(pc);

        self.registers.set_pc(self.registers.next_pc());
        self.registers.set_next_pc(self.registers.next_pc().wrapping_add(4));

        let (reg, val) = self.load.value();
        self.registers.set_reg(reg, val);

        // We reset the load to target register 0 for the next
        // instruction
        self.load.reset();

        let operation = self.decode_and_execute(instruction);

        debug!("0x{:08x}: {}", self.registers.pc(), operation.gnu());

        operation.perform(&mut self.registers, &mut self.interconnect, &mut self.load);

        self.registers.swap_registers();
    }

    fn decode_and_execute(&mut self, instruction: Instruction) -> Box<dyn Operation> {
        match instruction.function() {
            0b000000 => self.decode_and_execute_sub_function(instruction),
            0b001111 => Box::new(Lui::new(instruction)),
            0b001101 => Box::new(Ori::new(instruction)),
            0b101011 => Box::new(Sw::new(instruction)),
            0b001001 => Box::new(Addiu::new(instruction)),
            0b010000 => self.decode_and_execute_cop0(instruction),
            0b000010 => Box::new(J::new(instruction)),
            0b000101 => Box::new(Bne::new(instruction)),
            0b001000 => Box::new(Addi::new(instruction)),
            0b100011 => Box::new(Lw::new(instruction)),
            0b101001 => Box::new(Sh::new(instruction)),
            0b000011 => Box::new(Jal::new(instruction)),
            0b001100 => Box::new(Andi::new(instruction)),
            0b101000 => Box::new(Sb::new(instruction)),
            0b100000 => Box::new(Lb::new(instruction)),
            0b000100 => Box::new(Beq::new(instruction)),
            0b000111 => Box::new(Bqtz::new(instruction)),
            0b000110 => Box::new(Bltz::new(instruction)),
            0b100100 => Box::new(Lbu::new(instruction)),
            0b000001 => Box::new(Bxx::new(instruction)),
            0b001010 => Box::new(Slti::new(instruction)),
            0b001011 => Box::new(Sltiu::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    fn decode_and_execute_sub_function(&mut self, instruction: Instruction) -> Box<dyn Operation> {
        match instruction.subfunction() {
            0b000000 => Box::new(Sll::new(instruction)),
            0b100101 => Box::new(Or::new(instruction)),
            0b101011 => Box::new(Sltu::new(instruction)),
            0b100001 => Box::new(Addu::new(instruction)),
            0b001000 => Box::new(Jr::new(instruction)),
            0b100100 => Box::new(And::new(instruction)),
            0b100000 => Box::new(Add::new(instruction)),
            0b001001 => Box::new(Jarl::new(instruction)),
            0b100011 => Box::new(Subu::new(instruction)),
            0b000011 => Box::new(Sra::new(instruction)),
            0b011010 => Box::new(Div::new(instruction)),
            0b010010 => Box::new(Mflo::new(instruction)),
            0b010000 => Box::new(Mfhi::new(instruction)),
            0b000010 => Box::new(Srl::new(instruction)),
            0b011011 => Box::new(Divu::new(instruction)),
            0b101010 => Box::new(Slt::new(instruction)),
            0b001100 => Box::new(Syscall::new(instruction)),
            0b010011 => Box::new(Mtlo::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Sub function call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Coprocessor 0 opcode
    fn decode_and_execute_cop0(&mut self, instruction: Instruction) -> Box<dyn Operation> {
        match instruction.cop_opcode() {
            0b000100 => Box::new(Mtc0::new(instruction)),
            0b000000 => Box::new(Mfc0::new(instruction)),
            _ => panic!("Unhandled cop0 instruction [0x{:08x}]. Cop0 call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }
}
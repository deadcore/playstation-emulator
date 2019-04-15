extern crate env_logger;
extern crate log;

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
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
use crate::cpu::operations::lh::Lh;
use crate::cpu::operations::lhu::Lhu;
use crate::cpu::operations::lui::*;
use crate::cpu::operations::lw::Lw;
use crate::cpu::operations::lwl::Lwl;
use crate::cpu::operations::lwr::Lwr;
use crate::cpu::operations::mfc0::Mfc0;
use crate::cpu::operations::mfhi::Mfhi;
use crate::cpu::operations::mflo::Mflo;
use crate::cpu::operations::mtc0::*;
use crate::cpu::operations::mthi::Mtlo;
use crate::cpu::operations::mtlo::Mthi;
use crate::cpu::operations::multu::Multu;
use crate::cpu::operations::nor::Nor;
use crate::cpu::operations::Operation;
use crate::cpu::operations::or::*;
use crate::cpu::operations::ori::*;
use crate::cpu::operations::rfe::Rfe;
use crate::cpu::operations::sb::Sb;
use crate::cpu::operations::sh::Sh;
use crate::cpu::operations::sll::*;
use crate::cpu::operations::sllv::Sllv;
use crate::cpu::operations::slt::Slt;
use crate::cpu::operations::slti::Slti;
use crate::cpu::operations::sltiu::Sltiu;
use crate::cpu::operations::sltu::Sltu;
use crate::cpu::operations::sra::Sra;
use crate::cpu::operations::srl::Srl;
use crate::cpu::operations::srlv::Srlv;
use crate::cpu::operations::sub::Sub;
use crate::cpu::operations::subu::Subu;
use crate::cpu::operations::sw::*;
use crate::cpu::operations::syscall::Syscall;
use crate::cpu::operations::xor::Xor;
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
        // TODO - Raise PC alignment exception
        let pc = self.registers.pc();
        let instruction = Instruction(self.interconnect.load::<Word>(pc));

        // Save the address of the current instruction to save in
        // 'EPC' in case of an exception.
        self.registers.set_current_pc(pc);

        self.registers.set_pc(self.registers.next_pc());
        self.registers.set_next_pc(self.registers.next_pc().wrapping_add(4));

        let (reg, val) = self.load.value();
        self.registers.set_reg(reg, val);

        // We reset the load to target register 0 for the next
        // instruction
        self.load.reset();

        // If the last instruction was a branch then we're in the // delay slot
        self.load.set_delay_slot(self.load.branch());
        self.load.set_branch(false);

        let operation = self.decode(instruction);

        debug!("0x{:08x}: {}", self.registers.pc(), operation.gnu());

        let maybe_exception = operation.perform(&mut self.registers, &mut self.interconnect, &mut self.load);

        if let Some(exception) = maybe_exception {
            self.enter_exception(exception)
        }

        self.registers.swap_registers();
    }

    fn decode(&mut self, instruction: Instruction) -> Box<dyn Operation> {
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
            0b100101 => Box::new(Lhu::new(instruction)),
            0b100001 => Box::new(Lh::new(instruction)),
            0b100010 => Box::new(Lwl::new(instruction)),
            0b100110 => Box::new(Lwr::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    fn decode_and_execute_sub_function(&mut self, instruction: Instruction) -> Box<dyn Operation> {
        match instruction.subfunction() {
            0b000000 => Box::new(Sll::new(instruction)),
            0b100101 => Box::new(Or::new(instruction)),
            0b100111 => Box::new(Nor::new(instruction)),
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
            0b001100 => Box::new(Syscall::new()),
            0b010011 => Box::new(Mtlo::new(instruction)),
            0b010001 => Box::new(Mthi::new(instruction)),
            0b000100 => Box::new(Sllv::new(instruction)),
            0b100110 => Box::new(Xor::new(instruction)),
            0b011001 => Box::new(Multu::new(instruction)),
            0b000110 => Box::new(Srlv::new(instruction)),
            0b100010 => Box::new(Sub::new(instruction)),
            _ => panic!("Unhandled instruction [0x{:08x}]. Sub function call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Coprocessor 0 opcode
    fn decode_and_execute_cop0(&mut self, instruction: Instruction) -> Box<dyn Operation> {
        match instruction.cop_opcode() {
            0b000100 => Box::new(Mtc0::new(instruction)),
            0b000000 => Box::new(Mfc0::new(instruction)),
            0b010000 => Box::new(Rfe::new(instruction)),
            _ => panic!("Unhandled cop0 instruction [0x{:08x}]. Cop0 call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Update SR, CAUSE and EPC when an exception is
    /// triggered. Returns the address of the exception handler.
    fn enter_exception(&mut self, cause: Exception) {
        info!("Exception encountered");

        // Exception handler address depends on the 'BEV' bit:
        let handler = match self.registers.sr() & (1 << 22) != 0 {
            true => 0xbfc00180,
            false => 0x80000080,
        };

        // Shift bits [5:0] of `SR` two places to the left. Those bits
        // are three pairs of Interrupt Enable/User Mode bits behaving
        // like a stack 3 entries deep. Entering an exception pushes a
        // pair of zeroes by left shifting the stack which disables
        // interrupts and puts the CPU in kernel mode. The original
        // third entry is discarded (it's up to the kernel to handle
        // more than two recursive exception levels).
        let mode = self.registers.sr() & 0x3f;

        let mut sr = self.registers.sr();

        sr &= !0x3f;
        sr |= (mode << 2) & 0x3f;

        self.registers.set_sr(sr);

        // Update `CAUSE` register with the exception code (bits
        // [6:2])
        let mut register_cause = self.registers.cause();

        register_cause |= (cause as u32) << 2;

        self.registers.set_cause(register_cause);

        // Save current instruction address in 'EPC'
        self.registers.set_epc(self.registers.current_pc());

        if self.load.delay_slot() {
            // When an exception occurs in a delay slot `EPC` points
            // to the branch instruction and bit 31 of `CAUSE` is set.
            self.registers.set_epc(self.registers.pc().wrapping_sub(4));
            let mut cause = self.registers.cause();
            cause |= 1 << 31;
            self.registers.set_cause(cause);
        }

        // into the handler
        self.registers.set_pc(handler);
        self.registers.set_next_pc(self.registers.pc().wrapping_add(4));
    }
}
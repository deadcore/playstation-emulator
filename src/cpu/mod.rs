extern crate env_logger;
extern crate log;

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::operations::Operation;
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
    pub registers: Registers,

    /// Memory interface
    pub interconnect: Interconnect,

    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    pub load: Delay,
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

        let reg = self.load.register_index();
        let val = self.load.value();
        self.registers.set_reg(reg, val);

        // We reset the load to target register 0 for the next
        // instruction
        self.load.reset();

        // If the last instruction was a branch then we're in the // delay slot
        self.load.set_delay_slot(self.load.branch());
        self.load.set_branch(false);

        let operation = self.decode(instruction);

        if log_enabled!(log::Level::Debug) {
            debug!("0x{:08x}: {}", self.registers.pc(), operation.gnu());
        }

        let maybe_exception = operation.perform(&mut self.registers, &mut self.interconnect, &mut self.load);

        if let Err(exception) = maybe_exception {
            self.enter_exception(exception)
        }

        self.registers.swap_registers();
    }

    fn decode(&mut self, instruction: Instruction) -> Operation {
        match instruction.function() {
            0b000000 => self.decode_and_execute_sub_function(instruction),
            0b001111 => Operation::Lui(instruction),
            0b001101 => Operation::Ori(instruction),
            0b101011 => Operation::Sw(instruction),
            0b001001 => Operation::Addiu(instruction),
            0b010000 => self.decode_and_execute_cop0(instruction),
            0b000010 => Operation::J(instruction),
            0b000101 => Operation::Bne(instruction),
            0b001000 => Operation::Addi(instruction),
            0b100011 => Operation::Lw(instruction),
            0b101001 => Operation::Sh(instruction),
            0b000011 => Operation::Jal(instruction),
            0b001100 => Operation::Andi(instruction),
            0b101000 => Operation::Sb(instruction),
            0b100000 => Operation::Lb(instruction),
            0b000100 => Operation::Beq(instruction),
            0b000111 => Operation::Bqtz(instruction),
            0b000110 => Operation::Bltz(instruction),
            0b100100 => Operation::Lbu(instruction),
            0b000001 => Operation::Bxx(instruction),
            0b001010 => Operation::Slti(instruction),
            0b001011 => Operation::Sltiu(instruction),
            0b100101 => Operation::Lhu(instruction),
            0b100001 => Operation::Lh(instruction),
            0b100010 => Operation::Lwl(instruction),
            0b100110 => Operation::Lwr(instruction),
            0b101010 => Operation::Swl(instruction),
            0b101110 => Operation::Swr(instruction),
            _ => panic!("Unhandled instruction [0x{:08x}]. Function call was: [{:#08b}]", instruction.0, instruction.function())
        }
    }

    fn decode_and_execute_sub_function(&mut self, instruction: Instruction) -> Operation {
        match instruction.subfunction() {
            0b000000 => Operation::Sll(instruction),
            0b100101 => Operation::Or(instruction),
            0b100111 => Operation::Nor(instruction),
            0b101011 => Operation::Sltu(instruction),
            0b100001 => Operation::Addu(instruction),
            0b001000 => Operation::Jr(instruction),
            0b100100 => Operation::And(instruction),
            0b100000 => Operation::Add(instruction),
            0b001001 => Operation::Jalr(instruction),
            0b100011 => Operation::Subu(instruction),
            0b000011 => Operation::Sra(instruction),
            0b011010 => Operation::Div(instruction),
            0b010010 => Operation::Mflo(instruction),
            0b010000 => Operation::Mfhi(instruction),
            0b000010 => Operation::Srl(instruction),
            0b011011 => Operation::Divu(instruction),
            0b101010 => Operation::Slt(instruction),
            0b001100 => Operation::Syscall(instruction),
            0b010011 => Operation::Mtlo(instruction),
            0b010001 => Operation::Mthi(instruction),
            0b000100 => Operation::Sllv(instruction),
            0b100110 => Operation::Xor(instruction),
            0b011001 => Operation::Multu(instruction),
            0b000110 => Operation::Srlv(instruction),
            0b100010 => Operation::Sub(instruction),
            _ => panic!("Unhandled instruction [0x{:08x}]. Sub function call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Coprocessor 0 opcode
    fn decode_and_execute_cop0(&mut self, instruction: Instruction) -> Operation {
        match instruction.cop_opcode() {
            0b000100 => Operation::Mtc0(instruction),
            0b000000 => Operation::Mfc0(instruction),
            0b010000 => Operation::Rfe(instruction),
            _ => panic!("Unhandled cop0 instruction [0x{:08x}]. Cop0 call was: [{:#08b}]", instruction.0, instruction.subfunction())
        }
    }

    /// Update SR, CAUSE And EPC when an exception is
    /// triggered. Returns the address of the exception handler.
    fn enter_exception(&mut self, cause: Exception) {
        warn!("Exception [{}] encountered", cause);

        // Exception handler address depends on the 'BEV' bit:
        let handler = match self.registers.sr() & (1 << 22) != 0 {
            true => 0xbfc00180,
            false => 0x80000080,
        };

        // Shift bits [5:0] of `SR` two places to the left. Those bits
        // are three pairs of Interrupt Enable/User Mode bits behaving
        // like a stack 3 entries deep. Entering an exception pushes a
        // pair of zeroes by left shifting the stack which disables
        // interrupts And puts the CPU in kernel mode. The original
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
            // to the branch instruction And bit 31 of `CAUSE` is set.
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
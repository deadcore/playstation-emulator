use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};
use crate::instruction::*;
use crate::memory::Word;

/// The “store word left” (SWL) opcode is encoded by setting bits [31:26] of the instruction to 0x2a.
/// Since we only update part of the aligned target word we have to fetch its value before we can modify
/// it and store it back again:
pub struct Swl {
    instruction: Instruction
}

impl Swl {
    pub fn new(instruction: Instruction) -> impl Operation {
        Swl {
            instruction
        }
    }
}


impl Operation for Swl {
    /// Load word
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Option<Exception> {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);
        let v = registers.reg(t);

        let aligned_addr = addr & !3;

        // Load the current value for the aligned word at the target address
        let cur_mem = interconnect.load::<Word>(aligned_addr);

        // Depending on the address alignment we fetch the 1, 2, 3 or
        // 4 *most* significant bytes and put them in the target register.
        let mem = match addr & 3 {
            0 => (cur_mem & 0xffffff00) | (v >> 24),
            1 => (cur_mem & 0xffff0000) | (v >> 16),
            2 => (cur_mem & 0xff000000) | (v >> 8),
            3 => (cur_mem & 0x00000000) | (v >> 0),
            _ => unreachable!(),
        };

        interconnect.store::<Word>(addr, mem);

        None
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("swl {}, 0x{:04x}({})", t, i, s)
    }
}
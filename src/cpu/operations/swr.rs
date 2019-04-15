use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};
use crate::instruction::*;
use crate::memory::Word;

/// The “store word right” (SWR) opcode is encoded by setting bits [31:26] of the instruction to 0x2e.
/// It’s very similar to SWL except for a a few key differences:
pub struct Swr {
    instruction: Instruction
}

impl Swr {
    pub fn new(instruction: Instruction) -> impl Operation {
        Swr {
            instruction
        }
    }
}


impl Operation for Swr {
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
            0 => (cur_mem & 0x00000000) | (v << 0),
            1 => (cur_mem & 0x000000ff) | (v << 8),
            2 => (cur_mem & 0x0000ffff) | (v << 16),
            3 => (cur_mem & 0x00ffffff) | (v << 24),
            _ => unreachable!(),
        };

        interconnect.store::<Word>(addr, mem);

        None
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("swr {}, 0x{:04x}({})", t, i, s)
    }
}
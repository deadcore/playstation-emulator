use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};
use crate::instruction::*;
use crate::memory::Word;

/// The “load word left” (LWL) opcode is encoded by setting bits [31:26] of the instruction to 0x22.
pub struct Lwl {
    instruction: Instruction
}

impl Lwl {
    pub fn new(instruction: Instruction) -> impl Operation {
        Lwl {
            instruction
        }
    }
}


impl Operation for Lwl {
    /// Load word
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Option<Exception> {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);

        // This instruction bypasses the load delay restriction: this
        // instruction will merge the new contents with the value // currently being loaded i f need be .
        let cur_v = registers.out_regs()[t.to_usize()];

        // Next we load the ∗aligned∗ word containing the first // addressed byte
        let aligned_addr = addr & !3;
        let aligned_word = interconnect.load::<Word>(aligned_addr);

        // Depending on the address alignment we fetch the 1, 2, 3 or
        // 4 *most* significant bytes and put them in the target register.
        let v = match addr & 3 {
            0 => (cur_v & 0x00ffffff) | (aligned_word << 24),
            1 => (cur_v & 0x0000ffff) | (aligned_word << 16),
            2 => (cur_v & 0x000000ff) | (aligned_word << 8),
            3 => (cur_v & 0x00000000) | (aligned_word << 0),
            _ => unreachable!(),
        };

        load.set(t, v);

        None
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("lwl {}, 0x{:04x}({})", t, i, s)
    }
}
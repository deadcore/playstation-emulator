use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

pub struct J {
    instruction: Instruction
}

impl J {
    pub fn new(instruction: Instruction) -> J {
        J {
            instruction
        }
    }
}

impl Operation for J {
    /// Load word
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_jump();

        registers.set_pc((registers.pc() & 0xf0000000) | (i << 2));
    }
}
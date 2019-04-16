use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

pub struct J {
    instruction: Instruction
}

impl J {
    pub fn new(instruction: Instruction) -> impl Operation {
        J {
            instruction
        }
    }
}

impl Operation for J {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let i = self.instruction.imm_jump();

        registers.set_next_pc((registers.pc() & 0xf0000000) | (i << 2));
        Ok(())
    }

    fn gnu(&self) -> String {
        format!("J")
    }
}
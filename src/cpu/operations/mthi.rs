use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Unsurprisingly the MTLO is almost immediately followed by instruction 0x00400011 which is “move
/// to HI" (MTHI):
///
/// mtlo $2
pub struct Mtlo {
    instruction: Instruction
}

impl Mtlo {
    pub fn new(instruction: Instruction) -> impl Operation {
        Mtlo {
            instruction
        }
    }
}

impl Operation for Mtlo {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let s = self.instruction.s();

        registers.set_lo(registers.reg(s));
        None
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();

        format!("mtlo {}", s)
    }
}
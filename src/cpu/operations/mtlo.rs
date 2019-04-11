use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// In the exception handler we stumble upon 0x00400013 which is “move to LO” (MTLO):
///
/// mtlo $2
///
/// As its name implies it just moves the value from a general purpose register into the LO register.
/// Be careful though because the instruction encoding is different from MFLO:
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
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let s = self.instruction.s();

        registers.set_lo(registers.reg(s));
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();

        format!("mtlo {}", s)
    }
}
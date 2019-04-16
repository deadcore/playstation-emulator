use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We've seen that divisions store their results in the HI and LO registers but we don't know how
/// we access those yet. Unsurprisingly the next unhandled instruction does just that: 0x00001812
/// encodes “move from LO" (MFLO):
///
/// mflo $3
///
/// This instruction simply moves the contents of LO in a general purpose register. This instruction
/// would also stall if the division was not yet done but we'll implement that later:
pub struct Mflo {
    instruction: Instruction
}

impl Mflo {
    pub fn new(instruction: Instruction) -> impl Operation {
        Mflo {
            instruction
        }
    }
}

impl Operation for Mflo {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let d = self.instruction.d();
        let lo = registers.lo();

        registers.set_reg(d, lo);
        Ok(())
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();

        format!("MFLO {}", d)
    }
}
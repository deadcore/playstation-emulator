use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We already implemented MFLO, now we meet instruction 0x0000c810 which encodes “move from HI” (MFHI):
///
/// mfhi $25
///
/// Like MFLO it should be able to stall if the operation has not yet finished
/// but we’ll implement that later:
pub struct Mfhi {
    instruction: Instruction
}

impl Mfhi {
    pub fn new(instruction: Instruction) -> Mfhi {
        Mfhi {
            instruction
        }
    }
}

impl Operation for Mfhi {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let d = self.instruction.d();
        let hi = registers.hi();

        registers.set_reg(d, hi);
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();

        format!("MFHI {}", d)
    }
}
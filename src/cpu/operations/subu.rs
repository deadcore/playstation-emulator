use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x01c47023 which encodes “substract un- signed” (SUBU):
///
/// subu $14 , $14 , $4
///
/// The implementation is straightforward:
pub struct Subu {
    instruction: Instruction
}

impl Subu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Subu {
            instruction
        }
    }
}

impl Operation for Subu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let v = registers.reg(s).wrapping_sub(registers.reg(t));

        registers.set_reg(d, v)
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("SUBU {}, {}, {}", d, s, t)
    }
}
use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x0338082a which is “set on less than”:
///
/// slt $1, $25, $24
///
/// It’s like SLTU but with signed operands:
pub struct Slt {
    instruction: Instruction
}

impl Slt {
    pub fn new(instruction: Instruction) -> impl Operation {
        Slt {
            instruction
        }
    }
}

impl Operation for Slt {
    /// Set on Less Than (signed)
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let s = registers.reg(s) as i32;
        let t = registers.reg(t) as i32;
        let v = s < t;

        registers.set_reg(d, v as u32);
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("SLT {}, {}, {}", d, s, t)
    }
}
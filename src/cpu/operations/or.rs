use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Bitwise or
pub struct Or {
    instruction: Instruction
}

impl Or {
    pub fn new(instruction: Instruction) -> impl Operation {
        Or {
            instruction
        }
    }
}

impl Operation for Or {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = registers.reg(s) | registers.reg(t);

        registers.set_reg(d, v);
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("OR {}, {}, {}", d, s, t)
    }
}
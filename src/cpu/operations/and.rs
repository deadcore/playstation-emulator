use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// An other easy instruction follows a few cycles later: 0x00412024 which is a “bitwise and” (AND):
///
/// and $4, $2, $1
///
/// We’ve already implemented OR so we can reuse the code, only changing the
/// operator:
pub struct And {
    instruction: Instruction
}

impl And {
    pub fn new(instruction: Instruction) -> And {
        And {
            instruction
        }
    }
}

impl Operation for And {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = registers.reg(s) & registers.reg(t);

        registers.set_reg(d, v);
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("AND {}, {}, {}", d, s, t)
    }
}
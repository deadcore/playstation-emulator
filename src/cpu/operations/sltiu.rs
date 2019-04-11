use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// After that we meet 0x2c410045 which is “set if less than immediate unsigned” (SLTI):
///
/// sltiu $1, $2, 0x45
///
/// It’s implemented like SLTI but using unsigned integers18:
pub struct Sltiu {
    instruction: Instruction
}

impl Sltiu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sltiu {
            instruction
        }
    }
}

impl Operation for Sltiu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_se();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = registers.reg(s) < i;

        registers.set_reg(t, v as u32)
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm_se();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("SLTIU {}, {}, 0x{:04x}", t, s, i)
    }
}
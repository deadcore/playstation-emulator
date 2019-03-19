use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Bitwise Or Immediate
pub struct Ori {
    instruction: Instruction
}

impl Ori {
    pub fn new(instruction: Instruction) -> Ori {
        Ori {
            instruction
        }
    }
}

impl Operation for Ori {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm();
        let t = self.instruction.t();
        let s = self.instruction.s();
        let v = registers.reg(s) | i;

        registers.set_reg(t, v);
    }
}
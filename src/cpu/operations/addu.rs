use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Add Immediate Unsigned
pub struct Addu {
    instruction: Instruction
}

impl Addu {
    pub fn new(instruction: Instruction) -> Addu {
        Addu {
            instruction
        }
    }
}

impl Operation for Addu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let v = registers.reg(s).wrapping_add(registers.reg(t));

        registers.set_reg(d, v)
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("ADDU {}, {}, {}", d, s, t)
    }
}
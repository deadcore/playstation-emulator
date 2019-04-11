use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Load Upper Immediate
pub struct Lui {
    instruction: Instruction
}

impl Lui {
    pub fn new(instruction: Instruction) -> impl Operation {
        Lui {
            instruction
        }
    }
}

impl Operation for Lui {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm();
        let t = self.instruction.t();

        let v = i << 16;

        registers.set_reg(t, v);
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm();
        let t = self.instruction.t();

        format!("LUI {}, 0x{:04x}", t, i)
    }
}
use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

pub struct Bne {
    instruction: Instruction
}

impl Bne {
    pub fn new(instruction: Instruction) -> impl Operation {
        Bne {
            instruction
        }
    }
}

impl Operation for Bne {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let i = self.instruction.imm_se();
        let s = self.instruction.s();
        let t = self.instruction.t();

        if registers.reg(s) != registers.reg(t) {
            registers.branch(i);
        }
        Ok(())
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let i = self.instruction.imm_se();

        format!("BNE {}, {}, 0x{:04x}", s, t, i)
    }
}
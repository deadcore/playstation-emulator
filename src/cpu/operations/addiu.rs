use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Add Immediate Unsigned
pub struct Addiu {
    instruction: Instruction
}

impl Addiu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Addiu {
            instruction
        }
    }
}

impl Operation for Addiu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let v = registers.reg(s).wrapping_add(i);

        registers.set_reg(t, v);

        Ok(())
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("ADDIU {}, {}, 0x{:04x}", t, s, i)
    }
}
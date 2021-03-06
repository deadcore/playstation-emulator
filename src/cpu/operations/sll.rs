use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Shift Left Logic
pub struct Sll {
    instruction: Instruction
}

impl Sll {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sll {
            instruction
        }
    }
}

impl Operation for Sll {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let v = registers.reg(t) << i;

        registers.set_reg(d, v);

        Ok(())
    }

    fn gnu(&self) -> String {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("SLL {}, {}, {}", d, t, i)
    }
}
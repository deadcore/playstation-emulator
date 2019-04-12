use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Bitwise not or
///
/// nor $25, $2, $zero
/// It simply computes a bitwise OR between two registers and then complements
/// the result before storing it in the destination register:
pub struct Nor {
    instruction: Instruction
}

impl Nor {
    pub fn new(instruction: Instruction) -> impl Operation {
        Nor {
            instruction
        }
    }
}

impl Operation for Nor {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = !(registers.reg(s) | registers.reg(t));

        registers.set_reg(d, v);
        None
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("nor {}, {}, {}", d, s, t)
    }
}
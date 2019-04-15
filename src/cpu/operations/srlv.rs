use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We finally encounter the last shift instruction: 0x01a52806 is “shift right logical variable” (SRLV):
///
/// srlv $5, $5, $13
///
/// It’s implemented like SRAV without sign extension (or like SRL with a
/// register holding the shift amount, if you prefer):
pub struct Srlv {
    instruction: Instruction
}

impl Srlv {
    pub fn new(instruction: Instruction) -> impl Operation {
        Srlv {
            instruction
        }
    }
}

impl Operation for Srlv {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let d = self.instruction.d();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let v = registers.reg(t) >> (registers.reg(s) & 0x1f);

        registers.set_reg(d, v);
        None
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("srlv {}, {}, {}", d, t, s)
    }
}
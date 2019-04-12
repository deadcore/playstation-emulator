use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then encounter an unhandled instruction: 0x0303c826 which encodes an “exclusive or” (XOR):
/// 
/// xor $25, $24, $3
/// 
/// We can implement it by copying the OR method and replacing the | operator with ^:
pub struct Xor {
    instruction: Instruction
}

impl Xor {
    pub fn new(instruction: Instruction) -> impl Operation {
        Xor {
            instruction
        }
    }
}

impl Operation for Xor {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = registers.reg(s) ^ registers.reg(t);

        registers.set_reg(d, v);
        None
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("xor {}, {}, {}", d, s, t)
    }
}
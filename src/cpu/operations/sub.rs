use std::option::Option;

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// “Substract” (SUB) is like SUBU but with signed arithmetics and it triggers an exception on signed
/// overflow. This instruction is encoded by setting bits [31:26] of the instruction to zero and bits
/// [5:0] to 0x22.
pub struct Sub {
    instruction: Instruction
}

impl Sub {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sub {
            instruction
        }
    }
}

impl Operation for Sub {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let s = registers.reg(s) as i32;
        let t = registers.reg(t) as i32;

        match s.checked_sub(t) {
            Some(v) => {
                registers.set_reg(d, v as u32);
                None
            }
            None => Some(Exception::Overflow),
        }
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("sub {}, {}, {}", d, s, t)
    }
}
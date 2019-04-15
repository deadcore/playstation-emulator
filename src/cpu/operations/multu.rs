use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x01240019 which encodes “multiply unsigned” (MULTU):
///
/// multu $9 , $4
///
/// It’s our first multiplication opcode. The CPU does the multiplication using
/// 64bit arithmetics and store the result across the HI and LO registers:
pub struct Multu {
    instruction: Instruction
}

impl Multu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Multu {
            instruction
        }
    }
}

impl Operation for Multu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let s = self.instruction.s();
        let t = self.instruction.t();

        let a = registers.reg(s) as u64;
        let b = registers.reg(t) as u64;

        let v = a * b;

        registers.set_hi((v >> 32) as u32);
        registers.set_lo(v as u32);
        None
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("multut {}, {}", s, t)
    }
}
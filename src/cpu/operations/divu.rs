use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Now we encounter the other division instruction: 0x0064001b which encodes “divide unsigned" (DIVU):
///
/// divu $3, $4
///
/// Since this version uses unsigned operands we only have one special case: the division by zero
/// (the first line in table 7). Thus the implementation is slightly shorter than DIV:
pub struct Divu {
    instruction: Instruction
}

impl Divu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Divu {
            instruction
        }
    }
}

impl Operation for Divu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let s = self.instruction.s();
        let t = self.instruction.t();

        let n = registers.reg(s);
        let d = registers.reg(t);

        if d == 0 {
            // Division by zero , results are bogus
            registers.set_hi(n);
            registers.set_lo(0xffffffff);
        } else {
            registers.set_hi(n % d);
            registers.set_lo(n / d);
        }
        Ok(())
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("DIVU {}, {}", s, t)
    }
}
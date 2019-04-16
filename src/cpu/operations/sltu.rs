use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// After that we encounter the instruction 0x0043082b which encodes the
/// “set on less than unsigned"(STLU) opcode:
///
/// sltu $1, $2, $3
///
/// This instruction compares the value of two registers ($2 and $3 in this case)
/// and sets the value of a third one ($1) to either 0 or 1 depending on the result
/// of the “less than" comparison:
pub struct Sltu {
    instruction: Instruction
}

impl Sltu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sltu {
            instruction
        }
    }
}

impl Operation for Sltu {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = registers.reg(s) < registers.reg(t);

        registers.set_reg(d, v as u32);

        Ok(())
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("SLTU {}, {}, {}", d, s, t)
    }
}
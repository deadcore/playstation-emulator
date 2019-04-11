use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Next we meet instruction 0x00042603 which is “shift right arithmetic" (SRA):
///
/// sra $4, $4, 24
///
/// There are two versions of the shift right instruction: arithmetic and logical. The arithmetic
/// version considers that the value is signed and use the sign bit to fill the missing MSBs in the
/// register after the shift.
///
/// In Rust, C and C++ we can achieve the same behavior by casting the register value to a signed
/// integer before doing the shift:
pub struct Sra {
    instruction: Instruction
}

impl Sra {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sra {
            instruction
        }
    }
}

impl Operation for Sra {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let v = (registers.reg(t) as i32) >> i;

        registers.set_reg(d, v as u32);
        None
    }

    fn gnu(&self) -> String {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("SRA {}, {}, {}", d, t, i)
    }
}
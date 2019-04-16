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
pub struct Srav {
    instruction: Instruction
}

impl Srav {
    pub fn new(instruction: Instruction) -> impl Operation {
        Srav {
            instruction
        }
    }
}

impl Operation for Srav {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let d = self.instruction.d();
        let s = self.instruction.s();
        let t = self.instruction.t();
        // Shift amount is truncated to 5 bits
        let v = (registers.reg(t) as i32) >> (registers.reg(s) & 0x1f);

        registers.set_reg(d, v as u32);

        Ok(())
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("srav {}, {}, {}", d, t, s)
    }
}
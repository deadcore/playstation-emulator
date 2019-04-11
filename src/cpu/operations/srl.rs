use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We’ve implemented SRA not long ago, now we encounter the sister instruction 0x00057082 which is
/// a “shift right logical” (SRL):
///
/// srl $14, $5, 2
///
/// It’s very similiar to SRA except that the instruction treats the value as unsigned and fills the
/// missing MSBs with 0 after the shift. In Rust, C and C++ we can achieve this behavior by shifting
/// unsigned values:
pub struct Srl {
    instruction: Instruction
}

impl Srl {
    pub fn new(instruction: Instruction) -> impl Operation {
        Srl {
            instruction
        }
    }
}

impl Operation for Srl {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let v = registers.reg(t) >> i;

        registers.set_reg(d, v)
    }

    fn gnu(&self) -> String {
        let i = self.instruction.shift();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("SRL {}, {}, {}", d, t, i)
    }
}
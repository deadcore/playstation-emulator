use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We continue with instruction 0x308400ff which is a “bitwise and immediate” (ANDI):
///
/// andi $4, $4, 0xff
/// 
/// We can simply copy the implementation of ORI and replace the | with an &:
pub struct Andi {
    instruction: Instruction
}

impl Andi {
    pub fn new(instruction: Instruction) -> Andi {
        Andi {
            instruction
        }
    }
}

impl Operation for Andi {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm();
        let t = self.instruction.t();
        let s = self.instruction.s();
        let v = registers.reg(s) & i;

        registers.set_reg(t, v);
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("ANDI {}, {}, 0x{:04x}", t, s, i)
    }
}
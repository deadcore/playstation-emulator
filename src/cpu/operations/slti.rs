use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then encounter 0x28810010 which encodes instruction “set if less than immediate” (SLTI):
///
/// slti $1, $4, 16
///
/// It works like SLTU except that it compares a register with an immediate
/// value (sign-extended) and the comparison is done using signed arithmetic:
pub struct Slti {
    instruction: Instruction
}

impl Slti {
    pub fn new(instruction: Instruction) -> Slti {
        Slti {
            instruction
        }
    }
}

impl Operation for Slti {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_se() as i32;
        let s = self.instruction.s();
        let t = self.instruction.t();

        let v = (registers.reg(s) as i32) < i;

        registers.set_reg(t, v as u32)
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm_se() as i32;
        let s = self.instruction.s();
        let t = self.instruction.t();

        format!("SLTI {}, {}, 0x{:04x}", t, s, i)
    }
}
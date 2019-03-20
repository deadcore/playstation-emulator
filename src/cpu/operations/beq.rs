use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then get a new branch instruction: 0x11e0000c is “branch if equal” (BEQ):
///
/// beq $15, $zero, +48
///
/// We can reuse the code of BNE by changing the condition:
pub struct Beq {
    instruction: Instruction
}

impl Beq {
    pub fn new(instruction: Instruction) -> Beq {
        Beq {
            instruction
        }
    }
}

impl Operation for Beq {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_se();
        let s = self.instruction.s();
        let t = self.instruction.t();

        if registers.reg(s) == registers.reg(t) {
            registers.branch(i);
        }
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let i = self.instruction.imm_se();

        format!("BEQ {}, {}, 0x{:04x}", s, t, i)
    }
}
use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Add Immediate Unsigned
pub struct Addi {
    instruction: Instruction
}

impl Addi {
    pub fn new(instruction: Instruction) -> Addi {
        Addi {
            instruction
        }
    }
}

impl Operation for Addi {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_se() as i32;
        let t = self.instruction.t();
        let s = self.instruction.s();

        let s = registers.reg(s) as i32;

        let v = match s.checked_add(i) {
            Some(v) => v as u32,
            None => panic!("ADDI overflow"),
        };

        registers.set_reg(t, v);
    }
}
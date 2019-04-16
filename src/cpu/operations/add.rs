use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We already implemented ADDIU, ADDI and ADDU. We finally encounter “add" (ADD) in instruction 0x01094020:
///
/// add $8, $8, $9
///
/// It adds the value of two registers (like ADDU) but generates an exception on
/// signed overflow (like ADDI):
pub struct Add {
    instruction: Instruction
}

impl Add {
    pub fn new(instruction: Instruction) -> impl Operation {
        Add {
            instruction
        }
    }
}

impl Operation for Add {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        let s = registers.reg(s) as i32;
        let t = registers.reg(t) as i32;

        match s.checked_add(t) {
            Some(v) => {
                registers.set_reg(d, v as u32);
                Ok(())
            },
            None => {
                Err(Exception::Overflow)
            },
        }
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();
        let t = self.instruction.t();
        let d = self.instruction.d();

        format!("ADD {}, {}, {}", d, s, t)
    }
}
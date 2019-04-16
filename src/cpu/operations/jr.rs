use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// A few steps later we encounter 0x03e00008 which is the “jump register" (JR) instruction.
/// It simply sets the PC to the value stored in one of the general purpose registers:
///
/// jr $31
///
/// Since JAL stores the return address in $31 we can return from a subroutine
/// by calling jr $ra which is exactly what the BIOS is doing here.
pub struct Jr {
    instruction: Instruction
}

impl Jr {
    pub fn new(instruction: Instruction) -> impl Operation {
        Jr {
            instruction
        }
    }
}

impl Operation for Jr {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        let s = self.instruction.s();

        registers.set_next_pc(registers.reg(s));
        Ok(())
    }

    fn gnu(&self) -> String {
        let s = self.instruction.s();

        format!("JR {}", s)
    }
}
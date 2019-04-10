use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Word;

/// Store Word
pub struct Syscall {
    instruction: Instruction
}

impl Syscall {
    pub fn new(instruction: Instruction) -> Syscall {
        Syscall {
            instruction
        }
    }
}

impl Operation for Syscall {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        registers.enter_exception(Exception::SysCall)
    }


    fn gnu(&self) -> String {
        format!("SYSCALL")
    }
}
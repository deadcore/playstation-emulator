use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Syscall

pub fn perform(instruction: &Instruction,  _: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    Err(Exception::SysCall)
}

pub fn gnu(instruction: &Instruction) -> String {
    format!("SYSCALL")
}
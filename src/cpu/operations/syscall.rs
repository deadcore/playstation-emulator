use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;

/// Syscall
pub struct Syscall {}

impl Syscall {
    pub fn new() -> impl Operation {
        Syscall {}
    }
}

impl Operation for Syscall {
    fn perform(&self, _: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
        Err(Exception::SysCall)
    }

    fn gnu(&self) -> String {
        format!("SYSCALL")
    }
}
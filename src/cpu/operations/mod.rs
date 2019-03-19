use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::registers::Registers;

pub mod addi;
pub mod lw;
pub mod bne;
pub mod j;
pub mod sll;
pub mod or;
pub mod lui;
pub mod ori;
pub mod sw;
pub mod addiu;
pub mod mtc0;
pub mod sltu;
pub mod addu;

pub trait Operation {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay);

    fn gnu(&self) -> String;
}
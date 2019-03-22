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
pub mod sh;
pub mod jal;
pub mod andi;
pub mod sb;
pub mod jr;
pub mod lb;
pub mod beq;
pub mod mfc0;
pub mod and;
pub mod add;
pub mod bgtz;
pub mod bltz;
pub mod lbu;
pub mod jalr;
pub mod bxx;
pub mod slti;
pub mod subu;
pub mod sra;
pub mod div;
pub mod mflo;
pub mod srl;
pub mod sltiu;
pub mod divu;
pub mod mfhi;
pub mod slt;

pub trait Operation {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay);

    fn gnu(&self) -> String;
}
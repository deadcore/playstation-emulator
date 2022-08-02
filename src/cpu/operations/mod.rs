use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

mod addi;
mod lw;
mod bne;
mod j;
mod sll;
mod or;
mod lui;
mod ori;
mod sw;
mod addiu;
mod mtc0;
mod sltu;
mod addu;
mod sh;
mod jal;
mod andi;
mod sb;
mod jr;
mod lb;
mod beq;
mod mfc0;
mod and;
mod add;
mod bgtz;
mod bltz;
mod lbu;
mod jalr;
mod bxx;
mod slti;
mod subu;
mod sra;
mod div;
mod mflo;
mod srl;
mod sltiu;
mod divu;
mod mfhi;
mod slt;
mod syscall;
mod mtlo;
mod rfe;
mod mthi;
mod lhu;
mod sllv;
mod lh;
mod nor;
mod xor;
mod srav;
mod multu;
mod srlv;
mod sub;
mod lwl;
mod lwr;
mod swl;
mod swr;

pub enum Operation {
    Add(Instruction),
    Addi(Instruction),
    Addiu(Instruction),
    Addu(Instruction),
    And(Instruction),
    Andi(Instruction),
    Beq(Instruction),
    Bqtz(Instruction),
    Bltz(Instruction),
    Bne(Instruction),
    Bxx(Instruction),
    Div(Instruction),
    Divu(Instruction),
    J(Instruction),
    Jal(Instruction),
    Jalr(Instruction),
    Jr(Instruction),
    Lb(Instruction),
    Lbu(Instruction),
    Lh(Instruction),
    Lhu(Instruction),
    Lui(Instruction),
    Lw(Instruction),
    Lwl(Instruction),
    Lwr(Instruction),
    Mfc0(Instruction),
    Mfhi(Instruction),
    Mflo(Instruction),
    Mtc0(Instruction),
    Mthi(Instruction),
    Mtlo(Instruction),
    Multu(Instruction),
    Nor(Instruction),
    Or(Instruction),
    Ori(Instruction),
    Rfe(Instruction),
    Sb(Instruction),
    Sh(Instruction),
    Sll(Instruction),
    Sllv(Instruction),
    Slt(Instruction),
    Slti(Instruction),
    Sltiu(Instruction),
    Sltu(Instruction),
    Sra(Instruction),
    Srav(Instruction),
    Srl(Instruction),
    Srlv(Instruction),
    Sub(Instruction),
    Subu(Instruction),
    Sw(Instruction),
    Swl(Instruction),
    Swr(Instruction),
    Syscall(Instruction),
    Xor(Instruction),
}

impl Operation {
    pub fn gnu(&self) -> String {
        "".to_string()
    }

    //::perform(instruction , registers, interconnect, delay)

    pub fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, delay: &mut Delay) -> Result<(), Exception> {
        match self {
            Operation::Add(instruction) => add::perform(instruction, registers, interconnect, delay),
            Operation::Addi(instruction) => addi::perform(instruction, registers, interconnect, delay),
            Operation::Addiu(instruction) => addiu::perform(instruction, registers, interconnect, delay),
            Operation::Addu(instruction) => addu::perform(instruction, registers, interconnect, delay),
            Operation::And(instruction) => and::perform(instruction, registers, interconnect, delay),
            Operation::Andi(instruction) => andi::perform(instruction, registers, interconnect, delay),
            Operation::Beq(instruction) => beq::perform(instruction, registers, interconnect, delay),
            Operation::Bqtz(instruction) => bgtz::perform(instruction, registers, interconnect, delay),
            Operation::Bltz(instruction) => bltz::perform(instruction, registers, interconnect, delay),
            Operation::Bne(instruction) => bne::perform(instruction, registers, interconnect, delay),
            Operation::Bxx(instruction) => bxx::perform(instruction, registers, interconnect, delay),
            Operation::Div(instruction) => div::perform(instruction, registers, interconnect, delay),
            Operation::Divu(instruction) => divu::perform(instruction, registers, interconnect, delay),
            Operation::J(instruction) => j::perform(instruction, registers, interconnect, delay),
            Operation::Jal(instruction) => jal::perform(instruction, registers, interconnect, delay),
            Operation::Jalr(instruction) => jalr::perform(instruction, registers, interconnect, delay),
            Operation::Jr(instruction) => jr::perform(instruction, registers, interconnect, delay),
            Operation::Lb(instruction) => lb::perform(instruction, registers, interconnect, delay),
            Operation::Lbu(instruction) => lbu::perform(instruction, registers, interconnect, delay),
            Operation::Lh(instruction) => lh::perform(instruction, registers, interconnect, delay),
            Operation::Lhu(instruction) => lhu::perform(instruction, registers, interconnect, delay),
            Operation::Lui(instruction) => lui::perform(instruction, registers, interconnect, delay),
            Operation::Lw(instruction) => lw::perform(instruction, registers, interconnect, delay),
            Operation::Lwl(instruction) => lwl::perform(instruction, registers, interconnect, delay),
            Operation::Lwr(instruction) => lwr::perform(instruction, registers, interconnect, delay),
            Operation::Mfc0(instruction) => mfc0::perform(instruction, registers, interconnect, delay),
            Operation::Mfhi(instruction) => mfhi::perform(instruction, registers, interconnect, delay),
            Operation::Mflo(instruction) => mflo::perform(instruction, registers, interconnect, delay),
            Operation::Mtc0(instruction) => mtc0::perform(instruction, registers, interconnect, delay),
            Operation::Mthi(instruction) => mthi::perform(instruction, registers, interconnect, delay),
            Operation::Mtlo(instruction) => mtlo::perform(instruction, registers, interconnect, delay),
            Operation::Multu(instruction) => multu::perform(instruction, registers, interconnect, delay),
            Operation::Nor(instruction) => nor::perform(instruction, registers, interconnect, delay),
            Operation::Or(instruction) => or::perform(instruction, registers, interconnect, delay),
            Operation::Ori(instruction) => ori::perform(instruction, registers, interconnect, delay),
            Operation::Rfe(instruction) => rfe::perform(instruction, registers, interconnect, delay),
            Operation::Sb(instruction) => sb::perform(instruction, registers, interconnect, delay),
            Operation::Sh(instruction) => sh::perform(instruction, registers, interconnect, delay),
            Operation::Sll(instruction) => sll::perform(instruction, registers, interconnect, delay),
            Operation::Sllv(instruction) => sllv::perform(instruction, registers, interconnect, delay),
            Operation::Slt(instruction) => slt::perform(instruction, registers, interconnect, delay),
            Operation::Slti(instruction) => slti::perform(instruction, registers, interconnect, delay),
            Operation::Sltiu(instruction) => sltiu::perform(instruction, registers, interconnect, delay),
            Operation::Sltu(instruction) => sltu::perform(instruction, registers, interconnect, delay),
            Operation::Sra(instruction) => sra::perform(instruction, registers, interconnect, delay),
            Operation::Srav(instruction) => srav::perform(instruction, registers, interconnect, delay),
            Operation::Srl(instruction) => srl::perform(instruction, registers, interconnect, delay),
            Operation::Srlv(instruction) => srlv::perform(instruction, registers, interconnect, delay),
            Operation::Sub(instruction) => sub::perform(instruction, registers, interconnect, delay),
            Operation::Subu(instruction) => subu::perform(instruction, registers, interconnect, delay),
            Operation::Sw(instruction) => sw::perform(instruction, registers, interconnect, delay),
            Operation::Swl(instruction) => swl::perform(instruction, registers, interconnect, delay),
            Operation::Swr(instruction) => swr::perform(instruction, registers, interconnect, delay),
            Operation::Syscall(instruction) => syscall::perform(instruction, registers, interconnect, delay),
            Operation::Xor(instruction) => xor::perform(instruction, registers, interconnect, delay),
        }
    }
}
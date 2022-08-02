use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Bitwise not or
///
/// nor $25, $2, $zero
/// It simply computes a bitwise OR between two registers And then complements
/// the result before storing it in the destination register:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    let v = !(registers.reg(s) | registers.reg(t));

    registers.set_reg(d, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    format!("nor {}, {}, {}", d, s, t)
}
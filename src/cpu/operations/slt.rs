use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x0338082a which is “set on less than":
///
/// slt $1, $25, $24
///
/// It's like SLTU but with signed operands:

/// Set on Less Than (signed)
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    let s = registers.reg(s) as i32;
    let t = registers.reg(t) as i32;
    let v = s < t;

    registers.set_reg(d, v as u32);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    format!("SLT {}, {}, {}", d, s, t)
}
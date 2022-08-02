use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// An other easy instruction follows a few cycles later: 0x00412024 which is a “bitwise And" (AND):
///
/// And $4, $2, $1
///
/// We've already implemented OR so we can reuse the code, only changing the
/// operator:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    let v = registers.reg(s) & registers.reg(t);

    registers.set_reg(d, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    format!("AND {}, {}, {}", d, s, t)
}
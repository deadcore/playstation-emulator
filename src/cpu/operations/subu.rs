use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x01c47023 which encodes “substract un- signed" (SUBU):
///
/// subu $14 , $14 , $4
///
/// The implementation is straightforward:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    let v = registers.reg(s).wrapping_sub(registers.reg(t));

    registers.set_reg(d, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    format!("SUBU {}, {}, {}", d, s, t)
}
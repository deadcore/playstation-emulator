use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// After that we meet 0x2c410045 which is “set if less than immediate unsigned" (SLTI):
///
/// sltiu $1, $2, 0x45
///
/// It's implemented like SLTI but using unsigned integers18:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let s = instruction.s();
    let t = instruction.t();

    let v = registers.reg(s) < i;

    registers.set_reg(t, v as u32);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm_se();
    let s = instruction.s();
    let t = instruction.t();

    format!("SLTIU {}, {}, 0x{:04x}", t, s, i)
}
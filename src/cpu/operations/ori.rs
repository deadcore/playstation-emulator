use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Bitwise Or Immediate
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm();
    let t = instruction.t();
    let s = instruction.s();
    let v = registers.reg(s) | i;

    registers.set_reg(t, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm();
    let t = instruction.t();
    let s = instruction.s();

    format!("ORI {}, {}, 0x{:04x}", t, s, i)
}

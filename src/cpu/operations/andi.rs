use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We continue with instruction 0x308400ff which is a “bitwise And immediate" (ANDI):
///
/// andi $4, $4, 0xff
/// 
/// We can simply copy the implementation of ORI And replace the | with an &:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm();
    let t = instruction.t();
    let s = instruction.s();
    let v = registers.reg(s) & i;

    registers.set_reg(t, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm();
    let t = instruction.t();
    let s = instruction.s();

    format!("ANDI {}, {}, 0x{:04x}", t, s, i)
}
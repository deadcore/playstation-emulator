use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We finally encounter the last shift instruction: 0x01a52806 is “shift right logical variable” (SRLV):
///
/// srlv $5, $5, $13
///
/// It’s implemented like SRAV without sign extension (or like SRL with a
/// register holding the shift amount, if you prefer):

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let t = instruction.t();
    let s = instruction.s();

    let v = registers.reg(t) >> (registers.reg(s) & 0x1f);

    registers.set_reg(d, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let t = instruction.t();
    let s = instruction.s();

    format!("srlv {}, {}, {}", d, t, s)
}
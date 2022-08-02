use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We already implemented MFLO, now we meet instruction 0x0000c810 which encodes “move from HI" (MFHI):
///
/// mfhi $25
///
/// Like MFLO it should be able to stall if the operation has not yet finished
/// but we'll implement that later:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let hi = registers.hi();

    registers.set_reg(d, hi);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();

    format!("MFHI {}", d)
}
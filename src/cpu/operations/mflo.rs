use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We've seen that divisions store their results in the HI And LO registers but we don't know how
/// we access those yet. Unsurprisingly the next unhandled instruction does just that: 0x00001812
/// encodes “move from LO" (MFLO):
///
/// mflo $3
///
/// This instruction simply moves the contents of LO in a general purpose register. This instruction
/// would also stall if the division was not yet done but we'll implement that later:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let lo = registers.lo();

    registers.set_reg(d, lo);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();

    format!("MFLO {}", d)
}
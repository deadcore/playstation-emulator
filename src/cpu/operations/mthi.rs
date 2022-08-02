use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Unsurprisingly the MTLO is almost immediately followed by instruction 0x00400011 which is “move
/// to HI" (MTHI):
///
/// mtlo $2

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let s = instruction.s();

    registers.set_lo(registers.reg(s));
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();

    format!("mtlo {}", s)
}
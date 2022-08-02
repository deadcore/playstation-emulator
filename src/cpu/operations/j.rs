use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;


pub fn perform(instruction: &Instruction,  registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_jump();

    registers.set_next_pc((registers.pc() & 0xf0000000) | (i << 2));
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    format!("J")
}
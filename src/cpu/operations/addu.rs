use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Add Immediate Unsigned
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    let v = registers.reg(s).wrapping_add(registers.reg(t));

    registers.set_reg(d, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    format!("ADDU {}, {}, {}", d, s, t)
}
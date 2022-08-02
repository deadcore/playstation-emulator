use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Shift Left Logic
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    let v = registers.reg(t) << i;

    registers.set_reg(d, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    format!("SLL {}, {}, {}", d, t, i)
}
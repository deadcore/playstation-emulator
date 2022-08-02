use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// A few step later we encounter the complementary instruction 0x18a00005 which encodes
/// “branch if less than or equal to zero" (BLEZ):
///
/// blez $5, +20
///
/// It's the same thing as BGTZ with the opposite predicate:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let s = instruction.s();

    let v = registers.reg(s) as i32;

    if v <= 0 {
        registers.branch(i);
    }
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("BLTZ {}, 0x{:04x}", s, i)
}
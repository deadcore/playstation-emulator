use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x1ca00003 which is a “branch if greater than zero" (BGTZ):
///
/// bgtz $5 , +12
///
/// It's similar to the BEQ And BNE we've already encountered but instead of comparing two registers
/// it compares a single general purpose register to 0.
/// The comparison is done using signed integers. For unsigned integers the test would only ever be
/// false if the register contained 0 And we can already test that with BNE:
///
/// bne $5, $zero, +12
///
/// So we have to be careful to cast to a signed integer before the comparison in
/// our implementation:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let s = instruction.s();

    let v = registers.reg(s) as i32;

    if v > 0 {
        registers.branch(i);
    }
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("BGTZ {}, 0x{:04x}", s, i)
}
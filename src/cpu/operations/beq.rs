use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then get a new branch instruction: 0x11e0000c is “branch if equal" (BEQ):
///
/// beq $15, $zero, +48
///
/// We can reuse the code of BNE by changing the condition:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let s = instruction.s();
    let t = instruction.t();

    if registers.reg(s) == registers.reg(t) {
        registers.branch(i);
    }
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let t = instruction.t();
    let i = instruction.imm_se();

    format!("BEQ {}, {}, 0x{:04x}", s, t, i)
}
use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then encounter an unhandled instruction: 0x0303c826 which encodes an “exclusive or” (XOR):
/// 
/// xor $25, $24, $3
/// 
/// We can implement it by copying the OR method And replacing the | operator with ^:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    let v = registers.reg(s) ^ registers.reg(t);

    registers.set_reg(d, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    format!("xor {}, {}, {}", d, s, t)
}
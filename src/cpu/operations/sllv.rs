use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// After that we encounter 0x0078c804 which is “shift left logical variable” (SLLV):
///
/// sllv $25 , $24 , $3
///
/// It's like SLL except the shift amount is stored in a register instead of an immediate value.
/// The implementation is quite simple but there's something to consider: so far the shift amount
/// was always a 5bit immediate value but this time it's a 32bit register. What happens when the
/// register value is greater than 31?

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let t = instruction.t();
    let s = instruction.s();

    let v = registers.reg(t) << (registers.reg(s) & 0x1f);

    registers.set_reg(d, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();
    let t = instruction.t();
    let s = instruction.s();

    format!("sllv {}, {}, {}", d, t, s)
}
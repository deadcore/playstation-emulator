use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We then encounter 0x28810010 which encodes instruction “set if less than immediate" (SLTI):
///
/// slti $1, $4, 16
///
/// It works like SLTU except that it compares a register with an immediate
/// value (sign-extended) And the comparison is done using signed arithmetic:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se() as i32;
    let s = instruction.s();
    let t = instruction.t();

    let v = (registers.reg(s) as i32) < i;

    registers.set_reg(t, v as u32);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm_se() as i32;
    let s = instruction.s();
    let t = instruction.t();

    format!("SLTI {}, {}, 0x{:04x}", t, s, i)
}
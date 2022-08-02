use std::option::Option;

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// “Substract” (SUB) is like SUBU but with signed arithmetics And it triggers an exception on signed
/// overflow. This instruction is encoded by setting bits [31:26] of the instruction to zero And bits
/// [5:0] to 0x22.

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    let s = registers.reg(s) as i32;
    let t = registers.reg(t) as i32;

    match s.checked_sub(t) {
        Some(v) => {
            registers.set_reg(d, v as u32);
            Ok(())
        }
        None => Err(Exception::Overflow),
    }
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let t = instruction.t();
    let d = instruction.d();

    format!("sub {}, {}, {}", d, s, t)
}
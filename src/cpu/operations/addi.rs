use std::option::Option;

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Add Immediate Unsigned

pub fn perform(instruction: &Instruction,  registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se() as i32;
    let t = instruction.t();
    let s = instruction.s();

    let s = registers.reg(s) as i32;

    match s.checked_add(i) {
        Some(v) => {
            registers.set_reg(t, v as u32);
            Ok(())
        }
        None => Err(Exception::Overflow),
    }
}

fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm_se() as i32;
    let t = instruction.t();
    let s = instruction.s();

    format!("ADDI {}, {}, 0x{:04x}", t, s, i)
}
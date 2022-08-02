use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Next we meet instruction 0x00042603 which is “shift right arithmetic" (SRA):
///
/// sra $4, $4, 24
///
/// There are two versions of the shift right instruction: arithmetic And logical. The arithmetic
/// version considers that the value is signed And use the sign bit to fill the missing MSBs in the
/// register after the shift.
///
/// In Rust, C And C++ we can achieve the same behavior by casting the register value to a signed
/// integer before doing the shift:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    let v = (registers.reg(t) as i32) >> i;

    registers.set_reg(d, v as u32);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    format!("SRA {}, {}, {}", d, t, i)
}
use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We've implemented SRA not long ago, now we encounter the sister instruction 0x00057082 which is
/// a “shift right logical" (SRL):
///
/// srl $14, $5, 2
///
/// It's very similiar to SRA except that the instruction treats the value as unsigned And fills the
/// missing MSBs with 0 after the shift. In Rust, C And C++ we can achieve this behavior by shifting
/// unsigned values:

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    let v = registers.reg(t) >> i;

    registers.set_reg(d, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    format!("SRL {}, {}, {}", d, t, i)
}
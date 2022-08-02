use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Load Upper Immediate
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm();
    let t = instruction.t();

    let v = i << 16;

    registers.set_reg(t, v);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm();
    let t = instruction.t();

    format!("LUI {}, 0x{:04x}", t, i)
}

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Then we encounter instruction 0x0100f809 which encodes a “jump And link register" (JALR):
///
/// jalr $31, $8
///
/// It's implemented like JR except that it also stores the return address in a general purpose
/// register. Unlike JAL, JALR can store the return address in any general purpose register, not just
/// $ra:
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let d = instruction.d();
    let s = instruction.s();

    let ra = registers.next_pc();

    // Store return address in 'd'
    registers.set_reg(d, ra);

    registers.set_next_pc(registers.reg(s));
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let d = instruction.d();

    let s = instruction.s();

    format!("JALR {} {}", d, s)
}
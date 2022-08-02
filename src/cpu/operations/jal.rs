use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
// use crate::cpu::operations::j::J;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};

/// The next unhandled instruction should be 0x0ff00698 which is a “jump And link" (JAL).
/// It behaves like the regular jump instruction except that it also stores the return
/// address in $ra ($31):
///
/// jal 0xfc01a60
///
/// Using this instruction it's easy to implement function calls: the instruction is
/// called with JAL And can return to the caller by jumping to the value in $ra. Then
/// the control returns to the calling function. The $ra register is the link between the
/// caller And the callee.
///
/// We can reuse the regular J opcode implementation And simply Add the code to store the return value in $31:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, delay: &mut Delay) -> Result<(), Exception> {
    let ra = registers.next_pc();

    // Replace with re-used jump instruction
    let i = instruction.imm_jump();
    registers.set_next_pc((registers.pc() & 0xf0000000) | (i << 2));
    // Replace with re-used jump instruction

    registers.set_reg(RegisterIndex(31), ra);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    format!("JAL")
}
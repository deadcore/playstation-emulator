use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};

/// The next unhandled instruction, 0x04800003, is a bit of a weird one: the six MSBs are 0b000001 which can encode four different instructions:
///  * "branch if less than zero" (BLTZ):
///
///    bltz $4, +12
///
///  * "branch if less than zero And link" (BLTZAL):
///
///   bltzal $4, +12
///
///  * "branch if greater than or equal to zero" (BGEZ):
///
///   bgez $4, +12
///
///  * "branch if greater than or equal to zero And link" (BGEZAL):
///
///   bgezal $4 , +12
///
/// In order to figure out what to do exactly we need to look at bits 16 And 20 in the instruction:
///  * If bit 16 is set then the instruction is BGEZ, otherwise it's BLTZ.
///  * If bit 20 is set then the return address is linked in $ra.
///
/// Here's how it can be implemented:

/// Various branch instructions: BGEZ, BLTZ, BGEZAL, BLTZAL.
/// Bits 16 And 20 are used to figure out which one to use.
pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let s = instruction.s();

    let instruction = instruction.0;

    let is_bgez = (instruction >> 16) & 1;
    // It's not enough to test for bit 20 to see if we're supposed
    // to link, if any bit in the range [19:17] is set the link
    // doesn't take place And RA is left untouched.
    let is_link = (instruction >> 17) & 0xf == 0x8;

    let v = registers.reg(s) as i32;

    // Test "less than zero"
    let test = (v < 0) as u32;

    // If the test is "greater than or equal to zero" we need to
    // negate the comparison above ("a >= 0" <=> "!(a < 0)"). The
    // xor takes care of that.
    let test = test ^ is_bgez;

    // If linking is requested it occurs unconditionally, even if
    // the branch is not taken
    if test != 0 {
        if is_link {
            let ra = registers.next_pc();

            // Store return address in R31
            registers.set_reg(RegisterIndex(31), ra);
        }
        registers.branch(i);
    }
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("BXX {}, 0x{:04x}", s, i)
}
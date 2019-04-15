use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// As expected once the exception handler is done it executes instruction 0x42000010 which is a
/// coprocessor 0 opcode for “return from exception" (RFE):
///
/// rfe
///
/// All this instruction does is shift the Interrupt Enable/User Mode bits two places back to the
/// right. This effectively undoes the opposite shift done when entering the handler and therefore
/// puts the CPU back in the mode it was when the exception triggered (unless SR itself has been
/// modified in the handler).
/// It does not reset the PC however, it's up to the BIOS to fetch the address in EPC, increment it
/// by 4 to point at the next instruction and jump to it. The RFE instruction is typically in the
/// final jump delay slot (and that's exactly what the Playstation BIOS handler does in this case).
pub struct Rfe {
    instruction: Instruction
}

impl Rfe {
    pub fn new(instruction: Instruction) -> impl Operation {
        Rfe {
            instruction
        }
    }
}

impl Operation for Rfe {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        // There are other instructions with the same encoding but all
        // are virtual memory related and the Playstation doesnt't
        // implement them. Still, let's make sure we're not running
        // buggy code .
        if self.instruction.0 & 0x3f != 0b010000 {
            panic!("Invalid cop0 instruction: {}", self.instruction.0);
        }

        let mode = registers.sr() & 0x3f;
        let mut sr = registers.sr();

        sr &= !0x3f;
        sr |= mode >> 2;

        registers.set_sr(sr);

        None
    }

    fn gnu(&self) -> String {
        format!("rfe")
    }
}
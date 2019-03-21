use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Then we encounter instruction 0x0100f809 which encodes a “jump and link register” (JALR):
///
/// jalr $31, $8
///
/// It’s implemented like JR except that it also stores the return address in a general purpose
/// register. Unlike JAL, JALR can store the return address in any general purpose register, not just
/// $ra:
pub struct Jarl {
    instruction: Instruction
}

impl Jarl {
    pub fn new(instruction: Instruction) -> Jarl {
        Jarl {
            instruction
        }
    }
}

impl Operation for Jarl {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let d = self.instruction.d();
        let s = self.instruction.s();

        let ra = registers.pc();

        // Store return address in ‘d‘
        registers.set_reg(d, ra);

        registers.set_pc(registers.reg(s));
    }

    fn gnu(&self) -> String {
        let d = self.instruction.d();

        let s = self.instruction.s();

        format!("JALR {} {}", d, s)
    }
}
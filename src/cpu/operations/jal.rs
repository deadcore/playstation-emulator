use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::j::J;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, RegisterIndex};

/// The next unhandled instruction should be 0x0ff00698 which is a “jump and link" (JAL).
/// It behaves like the regular jump instruction except that it also stores the return
/// address in $ra ($31):
///
/// jal 0xfc01a60
///
/// Using this instruction it's easy to implement function calls: the instruction is
/// called with JAL and can return to the caller by jumping to the value in $ra. Then
/// the control returns to the calling function. The $ra register is the link between the
/// caller and the callee.
///
/// We can reuse the regular J opcode implementation and simply add the code to store the return value in $31:
pub struct Jal {
    instruction: Instruction
}

impl Jal {
    pub fn new(instruction: Instruction) -> impl Operation {
        Jal {
            instruction
        }
    }
}

impl Operation for Jal {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, delay: &mut Delay) -> Option<Exception> {
        let ra = registers.next_pc();

        J::new(self.instruction).perform(registers, interconnect, delay);

        registers.set_reg(RegisterIndex(31), ra);
        None
    }

    fn gnu(&self) -> String {
        format!("JAL")
    }
}
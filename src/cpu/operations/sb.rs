use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Byte;

/// Store Word
pub struct Sb {
    instruction: Instruction
}

impl Sb {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sb {
            instruction
        }
    }
}

impl Operation for Sb {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) {
        if registers.sr() & 0x10000 != 0 {
            // Cache is isolated , ignore write
            //warn!("ignoring store while cache is isolated");
            return;
        }

        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);
        let v = registers.reg(t);

        interconnect.store::<Byte>(addr, v)
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("SB {}, 0x{:04x}({})", t, i, s)
    }
}
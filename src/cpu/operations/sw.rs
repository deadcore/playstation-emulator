use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Store Word
pub struct Sw {
    instruction: Instruction
}

impl Sw {
    pub fn new(instruction: Instruction) -> Sw {
        Sw {
            instruction
        }
    }
}

impl Operation for Sw {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        if registers.sr() & 0x10000 != 0 {
            // Cache is isolated , ignore write
            warn!("ignoring store while cache is isolated");
            return;
        }

        let addr = registers.reg(s).wrapping_add(i);
        let v = registers.reg(t);

        interconnect.store32(addr, v)
    }

    fn gnu(&self) -> String {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        format!("SW {}, 0x{:04x}({})", t, i, s)
    }
}
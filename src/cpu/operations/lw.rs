use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

pub struct Lw {
    instruction: Instruction
}

impl Lw {
    pub fn new(instruction: Instruction) -> Lw {
        Lw {
            instruction
        }
    }
}

/// “load word”. It decodes to:
///
/// lw $9, 0($8)
///
/// We can reuse the load32 method to fetch the data from memory:
impl Operation for Lw {
    /// Load word
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) {
        if registers.sr() & 0x10000 != 0 { // Cache is isolated , ignore write
            println!("Ignoring load while cache is isolated");
            return;
        }

        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);

        let v = interconnect.load32(addr);

        load.set(t, v);
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("LW {}, 0x{:04x}({})", t, i, s)
    }
}
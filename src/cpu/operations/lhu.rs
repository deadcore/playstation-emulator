use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::{Byte, HalfWord};

/// The next unhandled instruction is 0x961901ae which is “load halfword unsigned” (LHU):
///
/// lhu $25 , 430($16)
///
/// It’s the 16bit counterpart to LBU and it’s our first 16bit load istruction:
pub struct Lhu {
    instruction: Instruction
}

impl Lhu {
    pub fn new(instruction: Instruction) -> impl Operation {
        Lhu {
            instruction
        }
    }
}

impl Operation for Lhu {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Option<Exception> {
        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);

        // Address must be 16 bit aligned
        if addr % 2 == 0 {
            let v = interconnect.load::<HalfWord>(addr);
            load.set(t, v as u32);
            None
        } else {
            Some(Exception::LoadAddressError)
        }
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("lhu {}, 0x{:08x}({})", t, i, s)
    }
}
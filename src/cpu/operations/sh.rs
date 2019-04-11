use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::HalfWord;

pub struct Sh {
    instruction: Instruction
}

impl Sh {
    pub fn new(instruction: Instruction) -> impl Operation {
        Sh {
            instruction
        }
    }
}

/// The next unhandled instruction is 0xa5200180 which encodes
/// “store halfword" (SH). It’s used to write 16bits (a halfword)
/// to the memory:
///
/// sh $zero , 0x180($9)
///
/// The implementation is very similar to the “store word" instruction
/// except we truncate the register to 16bits and we’ll have to
/// implement a new store16 method on our interconnect12:
impl Operation for Sh {
    fn perform(&self, registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) -> Option<Exception> {
        if registers.sr() & 0x10000 != 0 {
            // Cache is isolated , ignore write
            warn!("Ignoring store while cache is isolated");
            return None;
        }

        let i = self.instruction.imm_se();
        let t = self.instruction.t();
        let s = self.instruction.s();

        let addr = registers.reg(s).wrapping_add(i);

        if addr % 2 == 0 {
            let v = registers.reg(t);

            interconnect.store::<HalfWord>(addr, v);
            None
        } else {
            Some(Exception::StoreAddressError)
        }
    }

    fn gnu(&self) -> String {
        let t = self.instruction.t();
        let s = self.instruction.s();
        let i = self.instruction.imm_se();

        format!("SH {}, 0x{:04x}({})", t, i, s)
    }
}
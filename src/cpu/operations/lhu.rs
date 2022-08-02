use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::HalfWord;

/// The next unhandled instruction is 0x961901ae which is “load halfword unsigned” (LHU):
///
/// lhu $25 , 430($16)
///
/// It's the 16bit counterpart to LBU And it's our first 16bit load istruction:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    let addr = registers.reg(s).wrapping_add(i);

    // Address must be 16 bit aligned
    if addr % 2 == 0 {
        let v = interconnect.load::<HalfWord>(addr);
        load.set(t, v as u32);
        Ok(())
    } else {
        Err(Exception::LoadAddressError)
    }
}

pub fn gnu(instruction: &Instruction) -> String {
    let t = instruction.t();
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("lhu {}, 0x{:08x}({})", t, i, s)
}

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Word;

/// Store Word
pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    if registers.sr() & 0x10000 != 0 {
        // Cache is isolated , ignore write
        warn!("ignoring store while cache is isolated");
        return Ok(());
    }

    let addr = registers.reg(s).wrapping_add(i);

    if addr % 4 == 0 {
        let v = registers.reg(t);

        interconnect.store::<Word>(addr, v);
        Ok(())
    } else {
        Err(Exception::StoreAddressError)
    }
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    format!("SW {}, 0x{:04x}({})", t, i, s)
}
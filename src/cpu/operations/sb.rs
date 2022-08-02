use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Byte;

/// Store Word

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    if registers.sr() & 0x10000 != 0 {
        // Cache is isolated , ignore write
        warn!("ignoring store while cache is isolated");
        return Ok(());
    }

    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    let addr = registers.reg(s).wrapping_add(i);
    let v = registers.reg(t);

    interconnect.store::<Byte>(addr, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    format!("SB {}, 0x{:04x}({})", t, i, s)
}
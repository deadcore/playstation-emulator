use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Word;

/// “load word". It decodes to:
///
/// lw $9, 0($8)
///
/// We can reuse the load32 method to fetch the data from memory:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    if registers.sr() & 0x10000 != 0 { // Cache is isolated , ignore write
        warn!("Ignoring load while cache is isolated");
        return Ok(());
    }

    let addr = registers.reg(s).wrapping_add(i);

    if addr % 4 == 0 {
        let v = interconnect.load::<Word>(addr);
        load.set(t, v);
        Ok(())
    } else {
        Err(Exception::LoadAddressError)
    }
}

pub fn gnu(instruction: &Instruction) -> String {
    let t = instruction.t();
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("LW {}, 0x{:04x}({})", t, i, s)
}

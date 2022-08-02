use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::HalfWord;

/// We implemented LHU not long ago And now we meet 0x87a30018 which is “load halfword” (LH):
///
/// lh $3, 24($29)
///
/// It's implemented like LHU but it sign-extends the 16bit value to fit the 32bit
/// target register:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    if registers.sr() & 0x10000 != 0 { // Cache is isolated , ignore write
        warn!("Ignoring load while cache is isolated");
        return Ok(());
    }

    let addr = registers.reg(s).wrapping_add(i);

    if addr % 2 == 0 {
        let v = interconnect.load::<HalfWord>(addr);
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

    format!("lh {}, 0x{:04x}({})", t, i, s)
}

use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::HalfWord;


/// The next unhandled instruction is 0xa5200180 which encodes
/// “store halfword" (SH). It's used to write 16bits (a halfword)
/// to the memory:
///
/// sh $zero , 0x180($9)
///
/// The implementation is very similar to the “store word" instruction
/// except we truncate the register to 16bits And we'll have to
/// implement a new store16 method on our interconnect12:
pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    if registers.sr() & 0x10000 != 0 {
        // Cache is isolated , ignore write
        warn!("Ignoring store while cache is isolated");
        return Ok(());
    }

    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    let addr = registers.reg(s).wrapping_add(i);

    if addr % 2 == 0 {
        let v = registers.reg(t);

        interconnect.store::<HalfWord>(addr, v);
        Ok(())
    } else {
        Err(Exception::StoreAddressError)
    }
}

pub fn gnu(instruction: &Instruction) -> String {
    let t = instruction.t();
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("SH {}, 0x{:04x}({})", t, i, s)
}

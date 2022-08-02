use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Byte;

/// The next unhandled instruction is 0x81efe288 which encodes “load byte" (LB). As you can guess
/// it's like LW except that it only loads 8bits from the memory:
///
/// lb $15, −7544($15)
///
/// Since the general purpose registers are always 32bit LB only loads the low 8bits of the register.
/// The byte is treated like a signed value so it's sign extended to the full 32bits. Of course like
/// LW there's a load delay of one instruction. We can implement it like this14:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    let addr = registers.reg(s).wrapping_add(i);

    let v = interconnect.load::<Byte>(addr) as i8;

    load.set(t, v as u32);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let t = instruction.t();
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("LB {}, 0x{:08x}({})", t, i, s)
}
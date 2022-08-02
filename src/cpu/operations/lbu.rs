use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;
use crate::memory::Byte;

/// After that we meet instruction 0x90ae0000 which is a “load byte unsigned" (LBU):
///
/// lbu $14, 0($5)
///
/// It's exactly like LB but without sign extension, the high 24 bits of the target
/// register are set to 0:

pub fn perform(instruction: &Instruction,  registers: &mut Registers, interconnect: &mut Interconnect, load: &mut Delay) -> Result<(), Exception> {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    let addr = registers.reg(s).wrapping_add(i);

    let v = interconnect.load::<Byte>(addr);

    load.set(t, v as u32);
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let t = instruction.t();
    let s = instruction.s();
    let i = instruction.imm_se();

    format!("LBU {}, 0x{:08x}({})", t, i, s)
}
use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// We've already met MTC0, now we encounter the reciprocal instruction: 0x40026000 encodes “move
/// from coprocessor 0" (MFC0)16:
///
/// mfc0 $2, $cop0 12
///
/// There's one important thing to note however: MFC instructions behave like memory loads And have
/// a delay slot before the value is finally stored in the target register.
///
/// Fortunately we can simply re-use our load delay slots infrastructure:
pub fn perform(instruction: &Instruction,  registers: &mut Registers, _: &mut Interconnect, delay: &mut Delay) -> Result<(), Exception> {
    let cpu_r = instruction.t();
    let cop_r = instruction.d().0;

    let v = match cop_r {
        12 => registers.sr(),
        13 => registers.cause(),
        14 => registers.epc(),
        _ => panic!("Unhandled read from cop0r{}", cop_r),
    };

    delay.set(cpu_r, v);

    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let cpu_r = instruction.t();
    let cop_r = instruction.d().0;

    format!("MFC0 {}, cop0r_{}", cpu_r, cop_r)
}
use crate::cpu::delay::Delay;
use crate::cpu::exception::Exception;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// The next unhandled instruction is 0x0061001a which is “divide" (DIV):
///
/// div $3, $1
///
/// Multiplications And divisions are a bit peculiar on the MIPS architecture: for one, the result
/// is not stored in general purpose registers but in two dedicated 32bit registers: HI And LO.
///
/// For a division LO will contain the quotient And HI the remainder of the euclidean division.
///
/// The reason for this is that divisions And multiplications are typically much slower than the
/// other instructions we've implemented so far (with the exception of loads And stores potentially,
/// due to the memory latency). While a simple ADD or SRA can be executed in a single CPU cycle, DIV
/// can take as much as 36 cycles to get the result.

pub fn perform(instruction: &Instruction, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) -> Result<(), Exception> {
    let s = instruction.s();
    let t = instruction.t();

    let n = registers.reg(s) as i32;
    let d = registers.reg(t) as i32;

    if d == 0 {
        // Division by zero, results are bogus
        registers.set_hi(n as u32);

        if n >= 0 {
            registers.set_lo(0xffffffff)
        } else {
            registers.set_lo(1);
        }
    } else if n as u32 == 0x80000000 && d == -1 {
        // Result is not representable in a 32bit // signed integer
        registers.set_hi(0);

        registers.set_lo(0x80000000);
    } else {
        registers.set_hi((n % d) as u32);
        registers.set_lo((n / d) as u32);
    }
    Ok(())
}

pub fn gnu(instruction: &Instruction) -> String {
    let s = instruction.s();
    let t = instruction.t();

    format!("DIV {}, {}", s, t)
}
use crate::cpu::delay::Delay;
use crate::cpu::interconnect::Interconnect;
use crate::cpu::operations::Operation;
use crate::cpu::registers::Registers;
use crate::instruction::Instruction;

/// Coprocessor 0 opcode
pub struct Mtc0 {
    instruction: Instruction
}

impl Mtc0 {
    pub fn new(instruction: Instruction) -> Mtc0 {
        Mtc0 {
            instruction
        }
    }
}

impl Operation for Mtc0 {
    fn perform(&self, registers: &mut Registers, _: &mut Interconnect, _: &mut Delay) {
        let cpu_r = self.instruction.t();
        let cop_r = self.instruction.d().0;

        let v = registers.reg(cpu_r);

        match cop_r {
            3 | 5 | 6 | 7 | 9 | 11 => // Breakpoints registers
                if v != 0 {
                    panic!("Unhandled write to cop0r{}: {:08x}", cop_r, v)
                },
            12 => registers.set_sr(v),
            13 => // Cause register
                if v != 0 {
                    panic!("Unhandled write to CAUSE register.")
                },
            _ => panic!("Unhandled cop0 register {}", cop_r),
        }
    }

    fn gnu(&self) -> String {
        let cpu_r = self.instruction.t();
        let cop_r = self.instruction.d().0;

        format!("MTC0 {}, cop0r_{}", cpu_r, cop_r)
    }
}
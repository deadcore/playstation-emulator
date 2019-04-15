use std::fmt::*;

/// Simple wrapper around an instruction word to provide type-safety.
#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

#[derive(Clone, Copy)]
pub struct RegisterIndex(pub u32);

pub const REGISTERS: [&str; 32] = [
    "$zero",
    "$at",
    "$v0", "$v1",
    "$a0", "$a1", "$a2", "$a3",
    "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7",
    "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7",
    "$t8", "$t9",
    "$k0", "$k1",
    "$gp",
    "$sp",
    "$fp",
    "$ra"
];

impl RegisterIndex {
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub fn name(&self) -> &str {
        REGISTERS[self.to_usize()]
    }
}

impl Display for RegisterIndex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name())
    }
}

impl Instruction {
    /// Return bits [31:26] of the instruction
    pub fn function(self) -> u32 {
        let Instruction(op) = self;

        op >> 26
    }

    /// Return bits [5:0] of the instruction
    pub fn subfunction(self) -> u32 {
        let Instruction(op) = self;

        op & 0x3f
    }

    /// Return coprocessor opcode in bits [25:21]
    pub fn cop_opcode(self) -> u32 {
        let Instruction(op) = self;

        (op >> 21) & 0x1f
    }

    /// Return register index in bits [25:21]
    pub fn s(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 21) & 0x1f)
    }

    /// Return register index in bits [20:16]
    pub fn t(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 16) & 0x1f)
    }

    /// Return register index in bits [15:11]
    pub fn d(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 11) & 0x1f)
    }

    /// Return immediate value in bits [16:0]
    pub fn imm(self) -> u32 {
        let Instruction(op) = self;

        op & 0xffff
    }

    /// Return immediate value in bits [16:0] as a sign-extended 32bit
    /// value
    pub fn imm_se(self) -> u32 {
        (self.0 & 0xffff) as i16 as u32
    }

    /// Shift Immediate values are stored in bits [10:6]
    pub fn shift(self) -> u32 {
        let Instruction(op) = self;

        (op >> 6) & 0x1f
    }

    /// Jump target stored in bits [25:0]
    pub fn imm_jump(self) -> u32 {
        let Instruction(op) = self;

        op & 0x3ffffff
    }
}
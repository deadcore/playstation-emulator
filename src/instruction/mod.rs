/// Simple wrapper around an instruction word to provide type-safety.
#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    /// Return bits [31:26] of the instruction
    pub fn function(self) -> u32 {
        let Instruction(op) = self;
        op >> 26
    }

    /// Return register index in bits [25:21]
    pub fn s(self) -> u32 {
        let Instruction(op) = self;

        (op >> 21) & 0x1f
    }

    /// Return register index in bits [20:16]
    pub fn t(self) -> u32 {
        let Instruction(op) = self;
        (op >> 16) & 0x1f
    }

    /// Return immediate value in bits [16:0]
    pub fn imm(self) -> u32 {
        let Instruction(op) = self;
        op & 0xffff
    }

    /// Return immediate value in bits [16:0] as a sign−extended 32 bit value
    pub fn imm_se(self) -> u32 {
        let Instruction(op) = self;

        let v = (op & 0xffff) as i16;

        v as u32
    }

    /// Return register index in bits [15:11]
    pub fn d(self) -> u32 {
        let Instruction(op) = self;
        (op >> 11) & 0x1f
    }

    /// Returns bits [5:0] of the instruction
    pub fn subfunction(self) -> u32 {
        let Instruction(op) = self;
        op & 0x3f
    }

    /// Shift Immediate values are stored in bits [10:6]
    pub fn shift(self) -> u32 {
        let Instruction(op) = self;
        (op >> 6) & 0x1f
    }
}
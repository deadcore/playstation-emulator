use crate::instruction::RegisterIndex;

pub struct Registers {
    /// The program counter register
    pc: u32,

    /// General purpose registers
    /// The first entry must always contain 0
    regs: [u32; 32],

    /// 2nd set of registers used to emulate the load delay slot
    /// accurately. They contain the output of the current instruction.
    out_regs: [u32; 32],

    /// Cop0 register 12: Status Register
    sr: u32,

    /// HI the remainder of the euclidean division.
    hi: u32,

    /// For a division LO will contain the quotient
    lo: u32,
}

impl Registers {
    pub fn new() -> Registers {
        let mut regs = [0xdeadbeef; 32];
        // R0 is hardwired to 0
        regs[0] = 0;

        Registers {
            pc: 0xbfc00000,
            regs,
            out_regs: regs,
            sr: 0,
            hi: 0xdeadbeef,
            lo: 0xdeadbeef
        }
    }

    pub fn reg(&self, index: RegisterIndex) -> u32 {
        self.regs[index.to_usize()]
    }

    pub fn set_reg(&mut self, index: RegisterIndex, val: u32) {
        self.out_regs[index.to_usize()] = val;
        // Make sure R0 is always 0
        self.out_regs[0] = 0;
    }

    pub fn swap_registers(&mut self) {
        // Copy the output registers as input for the
        // next instruction
        self.regs = self.out_regs;
    }

    pub fn set_sr(&mut self, val: u32) {
        self.sr = val
    }

    pub fn sr(&self) -> u32 {
        self.sr
    }

    pub fn hi(&self) -> u32 {
        self.hi
    }

    pub fn set_hi(&mut self, hi: u32) {
        self.hi = hi
    }

    pub fn lo(&self) -> u32 {
        self.lo
    }

    pub fn set_lo(&mut self, lo: u32) {
        self.lo = lo
    }

    /// Branch to immediate value 'offset'
    pub fn branch(&mut self, offset: u32) {
        // Offset immediates are always shifted two places to the
        // right since `PC` addresses have to be aligned on 32bits at
        // all times.
        let offset = offset << 2;

        self.next_pc = self.pc.wrapping_add(offset);
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Increment PC to point to the next instruction.
    /// Wrapping add means that we want the PC to wrap back to 0 in case of an overflow (i.e. 0xfffffffc + 4 => 0x00000000)
    /// Increment PC to point to the next instruction . All
    /// instructions are 32bit long.
    pub fn increment_pc(&mut self) {
        self.pc = self.pc + 4
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc
    }
}
use crate::instruction::RegisterIndex;

pub struct Registers {
    /// The program counter register
    /// points to the next instruction
    pc: u32,

    /// Next value for the PC, used to simulate the
    /// branch delay slot
    next_pc: u32,

    /// Address of the instruction currently being executed. Used for
    /// setting the EPC in exceptions .
    current_pc: u32,

    /// Cop0 register 13: Cause Register
    cause: u32,

    /// Cop0 register 14: EPC
    epc: u32,

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

        let pc = 0xbfc00000;

        Registers {
            pc,
            next_pc: pc.wrapping_add(4),
            current_pc: 0,
            regs,
            out_regs: regs,
            sr: 0,
            cause: 0,
            epc: 0,
            hi: 0xdeadbeef,
            lo: 0xdeadbeef,
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

    pub fn set_next_pc(&mut self, next_pc: u32) {
        self.next_pc = next_pc
    }

    pub fn next_pc(&self) -> u32 {
        self.next_pc
    }

    pub fn current_pc(&self) -> u32 {
        self.current_pc
    }

    pub fn set_current_pc(&mut self, current_pc: u32) {
        self.current_pc = current_pc
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

    pub fn cause(&self) -> u32 {
        self.cause
    }

    pub fn set_cause(&mut self, cause: u32) {
        self.cause = cause
    }

    pub fn epc(&self) -> u32 {
        self.epc
    }

    pub fn set_epc(&mut self, epc: u32) {
        self.epc = epc
    }

    pub fn out_regs(&self) -> [u32; 32] {
        self.out_regs
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
    /// Wrapping Add means that we want the PC to wrap back to 0 in case of an overflow (i.e. 0xfffffffc + 4 => 0x00000000)
    /// Increment PC to point to the next instruction . All
    /// instructions are 32bit long.
    pub fn increment_pc(&mut self) {
        self.pc = self.pc + 4
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc
    }
}
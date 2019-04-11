use crate::instruction::RegisterIndex;

pub struct Delay {
    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    load: (RegisterIndex, u32),

    /// Set by the current instruction if a branch occurred and the
    /// next instruction will be in the delay slot.
    branch: bool,

    /// Set if the current instruction executes in the delay slot
    delay_slot: bool,
}

impl Delay {
    pub fn new() -> Delay {
        Delay {
            load: (RegisterIndex(0), 0),
            branch: false,
            delay_slot: false,
        }
    }

    pub fn set(&mut self, addr: RegisterIndex, val: u32) {
        self.load = (addr, val)
    }

    pub fn reset(&mut self) {
        self.load = (RegisterIndex(0), 0)
    }

    pub fn value(&self) -> (RegisterIndex, u32) {
        self.load
    }

    pub fn branch(&self) -> bool {
        self.branch
    }

    pub fn set_branch(&mut self, branch: bool) {
        self.branch = branch;
    }

    pub fn delay_slot(&self) -> bool {
        self.delay_slot
    }

    pub fn set_delay_slot(&mut self, delay_slot: bool) {
        self.delay_slot = delay_slot;
    }
}
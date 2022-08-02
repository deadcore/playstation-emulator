use crate::instruction::RegisterIndex;

pub struct Delay {
    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    load_index: RegisterIndex,

    load_value: u32,

    /// Set by the current instruction if a branch occurred And the
    /// next instruction will be in the delay slot.
    branch: bool,

    /// Set if the current instruction executes in the delay slot
    delay_slot: bool,
}

impl Delay {
    pub fn new() -> Delay {
        Delay {
            load_index: RegisterIndex(0),
            branch: false,
            delay_slot: false,
            load_value: 0,
        }
    }

    pub fn set(&mut self, addr: RegisterIndex, val: u32) {
        self.load_index = addr;
        self.load_value = val;
    }

    pub fn reset(&mut self) {
        self.load_index = RegisterIndex(0);
        self.load_value = 0;
    }

    pub fn register_index(&self) -> RegisterIndex {
        self.load_index
    }

    pub fn value(&self) -> u32 {
        self.load_value
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
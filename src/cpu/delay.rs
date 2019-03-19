use crate::instruction::RegisterIndex;

pub struct Delay {
    /// Load initiated by the current instruction (will take effect
    /// after the load delay slot)
    load: (RegisterIndex, u32),
}

impl Delay {
    pub fn new() -> Delay {
        Delay {
            load: (RegisterIndex(0), 0)
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
}
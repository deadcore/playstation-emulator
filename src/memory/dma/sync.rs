/// DMA transfer synchronization mode
#[derive(Clone, Copy)]
pub enum Sync {
    /// Transfer starts when the CPU writes to the Trigger bit and
    /// transfers everything at once
    Manual = 0,
    /// Sync blocks to DMA requests
    Request = 1,
    /// Used to transfer GPU command lists
    LinkedList = 2,
}
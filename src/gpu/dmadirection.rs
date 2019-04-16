/// Requested DMA direction .
#[derive(Copy, Clone)]
pub enum DmaDirection {
    Off = 0,
    Fifo = 1,
    CpuToGp0 = 2,
    VRamToCpu = 3,
}
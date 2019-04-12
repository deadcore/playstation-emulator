pub struct Dma {
    /// DMA control register
    control: u32,
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            // Reset value taken from the Nocash PSX spec
            control: 0x07654321,
        }
    }

    /// Retrieve the value of the control register
    pub fn control(&self) -> u32 {
        self.control
    }

    pub fn set_control(&mut self, control: u32) {
        self.control = control
    }

}
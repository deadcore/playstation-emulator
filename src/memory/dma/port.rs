/// The 7 DMA ports
pub enum Port {
    /// Macroblock decoder input
    MDecIn = 0,
    /// Macroblock decoder output
    MDecOut = 1,
    /// Graphics Processing Unit
    Gpu = 2,
    /// CD-ROM drive
    CdRom = 3,
    /// Sound Processing Unit
    Spu = 4,
    /// Extension port
    Pio = 5,
    /// Used to clear the ordering table
    Otc = 6,
}

impl Port {
    pub fn from_index(index: u32) -> Port {
        match index {
            0 => Port::MDecIn,
            1 => Port::MDecOut,
            2 => Port::Gpu,
            3 => Port::CdRom,
            4 => Port::Spu,
            5 => Port::Pio,
            6 => Port::Otc,
            n => panic!("Invalid port {}", n),
        }
    }
}
use crate::memory::dma::direction::Direction;
use crate::memory::dma::port::Port;
use crate::memory::dma::step::Step;
use crate::memory::dma::sync::Sync;

/// Per−channel data
pub struct Channel {
    enable: bool,
    direction: Direction,
    step: Step,
    sync: Sync,

    /// Used to start the DMA transfer when ‘sync‘ is ‘Manual‘
    trigger: bool,

    /// If true the DMA ”chops” the transfer and lets the CPU run /// in the gaps .
    chop: bool,

    /// Chopping DMA window size (log2 number of words)
    chop_dma_sz: u8,

    /// Chopping CPU window size (log2 number of cycles)
    chop_cpu_sz: u8,

    /// Unkown 2 RW bits in configuration register
    dummy: u8,

}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            enable: false,
            direction: Direction::ToRam,
            step: Step::Increment,
            sync: Sync::Manual,
            trigger: false,
            chop: false,
            chop_dma_sz: 0,
            chop_cpu_sz: 0,
            dummy: 0,
        }
    }

    /// Retrieve the value of the control register
    pub fn control(&self) -> u32 {
        let mut r = 0;

        r |= (self.direction as u32) << 0;
        r |= (self.step as u32) << 1;
        r |= (self.chop as u32) << 8;
        r |= (self.sync as u32) << 9;
        r |= (self.chop_dma_sz as u32) << 16;
        r |= (self.chop_cpu_sz as u32) << 20;
        r |= (self.enable as u32) << 24;
        r |= (self.trigger as u32) << 28;
        r |= (self.dummy as u32) << 29;

        r
    }

    /// Set the value of the control register
    pub fn set_control(&mut self, val: u32) {
        self.direction = match val & 1 != 0 {
            true => Direction::FromRam,
            false => Direction::ToRam,
        };

        self.step = match (val >> 1) & 1 != 0 {
            true => Step::Decrement,
            false => Step::Increment,
        };

        self.chop = (val >> 8) & 1 != 0;

        self.sync = match (val >> 9) & 3 {
            0 => Sync::Manual,
            1 => Sync::Request,
            2 => Sync::LinkedList,
            n => panic!("Unknown DMA sync mode {}", n),
        };

        self.chop_dma_sz = ((val >> 16) & 7) as u8;
        self.chop_cpu_sz = ((val >> 20) & 7) as u8;

        self.enable = (val >> 24) & 1 != 0;
        self.trigger = (val >> 28) & 1 != 0;

        self.dummy = ((val >> 29) & 3) as u8;
    }
}
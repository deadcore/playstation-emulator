use crate::memory::dma::direction::Direction;
use crate::memory::dma::step::Step;
use crate::memory::dma::sync::Sync;

/// Per−channel data
pub struct Channel {
    enable: bool,
    direction: Direction,
    step: Step,
    sync: Sync,

    /// Used to start the DMA transfer when 'sync' is 'Manual'
    trigger: bool,

    /// If true the DMA ”chops” the transfer and lets the CPU run /// in the gaps .
    chop: bool,

    /// Chopping DMA window size (log2 number of words)
    chop_dma_sz: u8,

    /// Chopping CPU window size (log2 number of cycles)
    chop_cpu_sz: u8,

    /// Unkown 2 RW bits in configuration register
    dummy: u8,

    /// DMA start address
    base: u32,

    /// Size of a block in words
    block_size: u16,

    /// Block count , Only used when 'sync' is 'Request'
    block_count: u16,
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
            base: 0,
            block_size: 0,
            block_count: 0,
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

    pub fn base(&self) -> u32 {
        self.base
    }

    pub fn set_base(&mut self, val: u32) {
        self.base = val & 0xffffff;
    }

    /// Retrieve value of the Block Control register
    pub fn block_control(&self) -> u32 {
        let bs = self.block_size as u32;
        let bc = self.block_count as u32;
        (bc << 16) | bs
    }

    pub fn set_block_control(&mut self, val: u32) {
        self.block_size = val as u16;
        self.block_count = (val >> 16) as u16;
    }

    pub fn active(&self) -> bool {
        // In manual sync mode the CPU must set the ”trigger” bit
        // to start the transfer .
        let trigger = match self.sync {
            Sync::Manual => self.trigger,
            _ => true,
        };

        self.enable && trigger
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn step(&self) -> Step {
        self.step
    }

    pub fn sync(&self) -> Sync {
        self.sync
    }

    /// Return the DMA transfer size in bytes or None for linked
    /// list mode.
    pub fn transfer_size(&self) -> Option<u32> {
        let bs = self.block_size as u32;
        let bc = self.block_count as u32;

        match self.sync {
            // For manual mode only the block size is used
            Sync::Manual => Some(bs),
            // In DMA request mode we must transfer ‘bc ‘ blocks
            Sync::Request => Some(bc * bs),
            // In linked list mode the size is not known ahead of
            // time: we stop when we encounter the ”end of list”
            // marker (0xffffff)
            Sync::LinkedList => None,
        }
    }
    /// Set the channel status to ”completed” state
    pub fn done(&mut self) {
        self.enable = false;
        self.trigger = false;
        // XXX Need to set the correct value for the other fields
        // (in particular interrupts)
    }
}
use crate::memory::dma::channel::Channel;
use crate::memory::dma::port::Port;

pub mod channel;
pub mod direction;
pub mod port;
pub mod step;
pub mod sync;

pub struct Dma {
    /// DMA control register
    control: u32,

    /// master IRQ enable
    irq_en: bool,

    /// IRQ enable for individual channels
    channel_irq_en: u8,

    /// IRQ flags for individual channels
    channel_irq_flags: u8,

    /// When set the interrupt is active unconditionally (even if 'irqen' is false)
    force_irq: bool,

    /// Bits know what them back
    /// [0:5] of the interrupt registers are RW but I don't
    /// they're supposed to do so I just store them And send
    /// untouched on reads
    irq_dummy: u8,

    /// The 7 channel instances
    channels: [Channel; 7],
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            // Reset value taken from the Nocash PSX spec
            control: 0x07654321,
            irq_en: false,
            channel_irq_en: 0,
            channel_irq_flags: 0,
            force_irq: false,
            irq_dummy: 0,
            channels: [
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
            ],
        }
    }

    /// Retrieve the value of the control register
    pub fn control(&self) -> u32 {
        self.control
    }

    pub fn set_control(&mut self, control: u32) {
        self.control = control
    }

    /// Return the status of the DMA interrupt
    fn irq(&self) -> bool {
        let channel_irq = self.channel_irq_flags & self.channel_irq_en;
        self.force_irq || (self.irq_en && channel_irq != 0)
    }
    /// Retrieve the value of the interrupt register
    pub fn interrupt(&self) -> u32 {
        let mut r = 0;
        r |= self.irq_dummy as u32;
        r |= (self.force_irq as u32) << 15;
        r |= (self.channel_irq_en as u32) << 16;
        r |= (self.irq_en as u32) << 23;
        r |= (self.channel_irq_flags as u32) << 24;
        r |= (self.irq() as u32) << 31;
        r
    }
    /// Set the value of the interrupt register
    pub fn set_interrupt(&mut self, val: u32) { // Unknown what bits [5:0] do
        self.irq_dummy = (val & 0x3f) as u8;
        self.force_irq = (val >> 15) & 1 != 0;
        self.channel_irq_en = ((val >> 16) & 0x7f) as u8;
        self.irq_en = (val >> 23) & 1 != 0;

        // Writing 1 to a flag resets it
        let ack = ((val >> 24) & 0x3f) as u8;
        self.channel_irq_flags &= !ack;
    }


    /// Return a reference to a channel by port number.
    pub fn channel(&self, port: Port) -> &Channel {
        &self.channels[port as usize]
    }

    /// Return a mutable reference to a channel by port number.
    pub fn channel_mut(&mut self, port: Port) -> &mut Channel {
        &mut self.channels[port as usize]
    }
}
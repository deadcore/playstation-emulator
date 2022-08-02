use crate::bios::Bios;
use crate::gpu::Gpu;
use crate::memory::{Addressable, Word};
use crate::memory::dma::direction::Direction;
use crate::memory::dma::Dma;
use crate::memory::dma::port::Port;
use crate::memory::dma::step::Step;
use crate::memory::dma::sync::Sync;
use crate::memory::ram::Ram;

/// Global interconnect
pub struct Interconnect {
    /// Basic Input/Output memory
    bios: Bios,
    ram: Ram,
    dma: Dma,
    gpu: Gpu
}

impl Interconnect {
    pub fn new(bios: Bios, ram: Ram, gpu: Gpu) -> Interconnect {
        Interconnect {
            bios,
            ram,
            gpu,
            dma: Dma::new(),
        }
    }
    /// Interconnect: load value at `addr`
    pub fn load<A: Addressable>(&mut self, addr: u32) -> u32 {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.load::<A>(offset);
        }

        if let Some(_) = map::SCRATCH_PAD.contains(abs_addr) {
            if addr > 0xa0000000 {
                panic!("ScratchPad access through uncached memory");
            }

            panic!("Unhandled SCRATCH_PAD load at address 0x{:08x}", addr)
        }

        if let Some(offset) = map::BIOS.contains(abs_addr) {
            return self.bios.load::<A>(offset);
        }

        if let Some(offset) = map::IRQ_CONTROL.contains(abs_addr) {
            warn!("IRQ control read 0x{:x}", offset);
            return 0;
        }

        if let Some(offset) = map::DMA.contains(abs_addr) {
            return self.dma_reg(offset);
        }

        if let Some(offset) = map::GPU.contains(abs_addr) {
            trace!("GPU read {}", offset);
            return match offset {
                // GPUSTAT: set bit 26, 27 28 to signal that the GPU
                // is ready for DMA And CPU access. This way the BIOS
                // won’t dead lock waiting for an event that’ll never
                // come .
                4 => 0x1c000000,
                _ => 0,
            };
        }

        if let Some(offset) = map::TIMERS.contains(abs_addr) {
            warn!("TIMERS control read 0x{:x}", offset);
            return 0;
        }

        if let Some(_) = map::CDROM.contains(abs_addr) {
            panic!("Unhandled CDROM load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::MDEC.contains(abs_addr) {
            panic!("Unhandled MDEC load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::SPU.contains(abs_addr) {
            warn!("Unhandled read from SPU register 0x{:08x}", abs_addr);
            return 0;
        }

        if let Some(_) = map::PAD_MEMCARD.contains(abs_addr) {
            panic!("Unhandled PAD_MEMCARD load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::EXPANSION_1.contains(abs_addr) {
            // No expansion implemented
            return 0xff;
        }

        if let Some(_) = map::RAM_SIZE.contains(abs_addr) {
            panic!("Unhandled RAM_SIZE load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::MEM_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled MEM_CONTROL access ({})", A::size());
            }

            panic!("Unhandled MEM_CONTROL load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::CACHE_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled cache control access ({})", A::size());
            }

            panic!("Unhandled CACHE_CONTROL load at address 0x{:08x}", addr)
        }

        if let Some(_) = map::EXPANSION_2.contains(abs_addr) {
            panic!("Unhandled EXPANSION_2 load at address 0x{:08x}", addr)
        }

        panic!("unhandled load at address 0x{:08x}", addr);
    }

    /// Interconnect: store `val` into `addr`
    pub fn store<A: Addressable>(&mut self, addr: u32, val: u32) {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.store::<A>(offset, val);
        }

        if let Some(offset) = map::SCRATCH_PAD.contains(abs_addr) {
            if addr > 0xa0000000 {
                panic!("ScratchPad access through uncached memory");
            }

            panic!("Unhandled write to SCRATCH_PAD 0x{:x}", offset);
        }

        if let Some(offset) = map::IRQ_CONTROL.contains(abs_addr) {
            warn!("Unhandled IRQ control: 0x{:x} <− 0x{:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::DMA.contains(abs_addr) {
            return self.set_dma_reg(offset, val);
        }

        if let Some(offset) = map::GPU.contains(abs_addr) {
            return match offset {
                0 => self.gpu.gp0(val),
                4 => self.gpu.gp1(val),
                _ => panic!("GPU write {}: 0x{:08x}", offset, val),
            }
        }

        if let Some(offset) = map::TIMERS.contains(abs_addr) {
            warn!("Unhandled TIMERS control: 0x{:x} <− 0x{:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::CDROM.contains(abs_addr) {
            panic!("Unhandled write to CDROM 0x{:x}", offset);
        }

        if let Some(offset) = map::MDEC.contains(abs_addr) {
            panic!("Unhandled write to MDEC 0x{:x}", offset);
        }

        if let Some(offset) = map::SPU.contains(abs_addr) {
            warn!("Unhandled write: SPU control: 0x{:x} <− 0x{:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::PAD_MEMCARD.contains(abs_addr) {
            panic!("Unhandled write to PAD_MEMCARD 0x{:x}", offset);
        }

        if let Some(_) = map::CACHE_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled cache control access");
            }

            return;
        }

        if let Some(offset) = map::MEM_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled MEM_CONTROL access ({})", A::size());
            }

            let val = val;

            match offset {
                0 => // Expansion 1 base address
                    if val != 0x1f000000 {
                        panic!("Bad expansion 1 base address: 0x{:08x}", val);
                    },
                4 => // Expansion 2 base address
                    if val != 0x1f802000 {
                        panic!("Bad expansion 2 base address: 0x{:08x}", val);
                    },
                _ =>
                    warn!("Unhandled write to MEM_CONTROL register 0x{:x}: 0x{:08x}", offset, val),
            }
            return;
        }

        if let Some(_) = map::RAM_SIZE.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled RAM_SIZE access");
            }
            return;
        }

        if let Some(_) = map::EXPANSION_2.contains(abs_addr) {
            return;
        }

        panic!("unhandled store into address 0x{:08x}: 0x{:08x}", addr, val);
    }

    fn set_dma_reg(&mut self, offset: u32, val: u32) {
        let major = (offset & 0x70) >> 4;
        let minor = offset & 0xf;

        let active_port = {
            // Per−channel registers
            match major {
                0..=6 => {
                    let port = Port::from_index(major);
                    let channel = self.dma.channel_mut(port);
                    match minor {
                        0 => channel.set_base(val),
                        4 => channel.set_block_control(val),
                        8 => channel.set_control(val),
                        _ => panic!("Unhandled DMA write 0x{:x}: 0x{:08x}", offset, val)
                    }
                    if channel.active() {
                        Some(port)
                    } else {
                        None
                    }
                }
                7 => {
                    match minor {
                        0 => self.dma.set_control(val),
                        4 => self.dma.set_interrupt(val),
                        _ => panic!("Unhandled DMA write 0x{:x}: 0x{:08x}", offset, val)
                    }
                    None
                }
                _ => panic!("Unhandled DMA write 0x{:x}: 0x{:08x}", offset, val)
            }
        };

        if let Some(port) = active_port {
            self.do_dma(port);
        }
    }

    fn dma_reg(&self, offset: u32) -> u32 {
        let major = (offset & 0x70) >> 4;
        let minor = offset & 0xf;

        match major {
            // Per−channel registers
            0..=6 => {
                let channel = self.dma.channel(Port::from_index(major));
                match minor {
                    8 => channel.control(),
                    _ => panic!("unhandled DMA access 0x{:x}", offset)
                }
            }
            // Common DMA registers
            7 => match minor {
                0 => self.dma.control(),
                4 => self.dma.interrupt(),
                _ => panic!("unhandled DMA access 0x{:x}", offset),
            }
            _ => panic!("unhandled DMA access 0x{:x}", offset)
        }
    }

    /// Execute DMA transfer for a port
    fn do_dma(&mut self, port: Port) {
        // DMA transfer has been started, for now let's
        // process everything in one pass (i.e. no chopping or priority handling)
        match self.dma.channel(port).sync() {
            Sync::LinkedList => self.do_dma_linked_list(port),
            _ => self.do_dma_block(port)
        }
    }

    /// Emulate DMA transfer for linked list synchronization mode.
    fn do_dma_linked_list(&mut self, port: Port) {
        let channel = self.dma.channel_mut(port);

        let mut addr = channel.base() & 0x1ffffc;

        if channel.direction() == Direction::ToRam {
            panic!("Invalid DMA direction for linked list mode");
        }

        // I don’t know if the DMA even supports linked list mode
        // for anything besides the GPU
        if port != Port::Gpu {
            panic!("Attempted linked list DMA on port {}", port as u8);
        }

        loop {
            // In linked list mode, each entry starts with a

            // ”header” word. The high byte contains the number
            // of words in the ”packet” (not counting the header word )
            let header = self.ram.load::<Word>(addr);

            let mut remsz = header >> 24;

            while remsz > 0 {
                addr = (addr + 4) & 0x1ffffc;

                let command = self.ram.load::<Word>(addr);
                debug!("GPU command 0x{:08x}", command);

                // Send command to the GPU
                self.gpu.gp0(command);

                remsz -= 1;
            }

            // The end−of−table marker is usually 0xffffff but mednafen only checks for the MSB so
            // maybe that's  what he hardware does? Since this bit is not part of any valid address
            // it makes some sense.  I’ll have to test that at some point . . .
            if header & 0x800000 != 0 {
                break;
            }

            addr = header & 0x1ffffc;
        }

        channel.done();
    }

    fn do_dma_block(&mut self, port: Port) {
        let channel = self.dma.channel_mut(port);

        // Move to channel
        let increment = match channel.step() {
            Step::Increment => 4,
            Step::Decrement => -4i32 as u32,
        };

        let mut addr = channel.base();

        // Transfer size in words
        let mut remsz = match channel.transfer_size() {
            Some(n) => n,
            // Shouldn't happen since we shouldn't be reaching this
            // code in linked list mode
            None => panic!("Couldn't figure out DMA block transfer size")
        };

        while remsz > 0 {
            // Not sure what happens if address is
            // bogus... Mednafen just masks addr this way, maybe
            // that’s how the hardware behaves (i.e. the RAM
            // address wraps And the two LSB are ignored, seems
            // reasonable enough
            let cur_addr = addr & 0x1ffffc;

            match channel.direction() {
                Direction::FromRam => {
                    let src_word = self.ram.load::<Word>(cur_addr);
                    match port {
                        Port::Gpu => self.gpu.gp0(src_word),
                        _ => panic!("Unhandled DMA destination port {}", port as u8)
                    }
                }
                Direction::ToRam => {
                    let src_word = match port {
                        Port::Otc => match remsz {
                            // Last entry contains the end
                            // of table marker
                            1 => 0xffffff,
                            // Pointer to the previous entry
                            _ => addr.wrapping_sub(4) & 0x1fffff,
                        }
                        _ => panic!("Unhandled DMA source port {}", port as u8)
                    };

                    self.ram.store::<Word>(cur_addr, src_word);
                }
            }
            addr = addr.wrapping_add(increment);
            remsz -= 1;
        }
        channel.done();
    }
}

pub mod map {
    pub struct Range(pub u32, pub u32);

    impl Range {
        /// Return `Some(offset)` if addr is contained in `self`
        pub fn contains(self, addr: u32) -> Option<u32> {
            let Range(start, length) = self;

            if addr >= start && addr < start + length {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    /// Mask array used to strip the region bits of the address. The
    /// mask is selected using the 3 MSBs of the address so each entry
    /// effectively matches 512kB of the address space. KSEG2 is not
    /// touched since it doesn't share anything with the other
    /// regions.
    const REGION_MASK: [u32; 8] = [
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, // KUSEG: 2048MB
        0x7fffffff,                                     // KSEG0:  512MB
        0x1fffffff,                                     // KSEG1:  512MB
        0xffffffff, 0xffffffff,                         // KSEG2: 1024MB
    ];

    /// Mask a CPU address to remove the region bits.
    pub fn mask_region(addr: u32) -> u32 {
        // Index address space in 512MB chunks
        let index = (addr >> 29) as usize;

        addr & REGION_MASK[index]
    }

    /// Main RAM: 2MB mirrored four times over the first 8MB (probably
    /// in case they decided to use a bigger RAM later on?)
    pub const RAM: Range = Range(0x00000000, 8 * 1024 * 1024);

    /// Expansion region 1
    pub const EXPANSION_1: Range = Range(0x1f000000, 512 * 1024);

    pub const BIOS: Range = Range(0x1fc00000, 512 * 1024);

    /// ScratchPad: data cache used as a fast 1kB RAM
    pub const SCRATCH_PAD: Range = Range(0x1f800000, 1024);

    /// Memory latency And expansion mapping
    pub const MEM_CONTROL: Range = Range(0x1f801000, 36);

    /// Gamepad And memory card controller
    pub const PAD_MEMCARD: Range = Range(0x1f801040, 32);

    /// Register that has something to do with RAM configuration,
    /// configured by the BIOS
    pub const RAM_SIZE: Range = Range(0x1f801060, 4);

    /// Interrupt Control registers (status And mask)
    pub const IRQ_CONTROL: Range = Range(0x1f801070, 8);

    /// Direct Memory Access registers
    pub const DMA: Range = Range(0x1f801080, 0x80);

    pub const TIMERS: Range = Range(0x1f801100, 0x30);

    /// CDROM controller
    pub const CDROM: Range = Range(0x1f801800, 0x4);

    pub const GPU: Range = Range(0x1f801810, 8);

    pub const MDEC: Range = Range(0x1f801820, 8);

    /// SPU registers
    pub const SPU: Range = Range(0x1f801c00, 640);

    /// Expansion region 2
    pub const EXPANSION_2: Range = Range(0x1f802000, 66);

    /// Cache control register. Full address since it's in KSEG2
    pub const CACHE_CONTROL: Range = Range(0xfffe0130, 4);
}

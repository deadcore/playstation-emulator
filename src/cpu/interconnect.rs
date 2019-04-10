use crate::bios::Bios;
use crate::memory::Addressable;
use crate::memory::ram::Ram;

/// Global interconnect
pub struct Interconnect {
    /// Basic Input/Output memory
    bios: Bios,

    ram: Ram,
}

impl Interconnect {
    pub fn new(bios: Bios, ram: Ram) -> Interconnect {
        Interconnect {
            bios,
            ram,
        }
    }
    /// Interconnect: load value at `addr`
    pub fn load<A: Addressable>(&mut self, addr: u32) -> u32 {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.load::<A>(offset);
        }

        if let Some(offset) = map::SCRATCH_PAD.contains(abs_addr) {
            if addr > 0xa0000000 {
                panic!("ScratchPad access through uncached memory");
            }

            panic!("Unhandled SCRATCH_PAD load at address {:08x}", addr)
        }

        if let Some(offset) = map::BIOS.contains(abs_addr) {
            return self.bios.load::<A>(offset);
        }

        if let Some(offset) = map::IRQ_CONTROL.contains(abs_addr) {
            warn!("IRQ control read {:x}", offset);
            return 0;
        }

        if let Some(offset) = map::DMA.contains(abs_addr) {
            panic!("Unhandled DMA load at address {:08x}", addr)
        }

        if let Some(offset) = map::GPU.contains(abs_addr) {
            panic!("Unhandled GPU load at address {:08x}", addr)
        }

        if let Some(offset) = map::TIMERS.contains(abs_addr) {
            panic!("Unhandled TIMERS load at address {:08x}", addr)
        }

        if let Some(offset) = map::CDROM.contains(abs_addr) {
            panic!("Unhandled CDROM load at address {:08x}", addr)
        }

        if let Some(offset) = map::MDEC.contains(abs_addr) {
            panic!("Unhandled MDEC load at address {:08x}", addr)
        }

        if let Some(offset) = map::SPU.contains(abs_addr) {
            panic!("Unhandled SPU load at address {:08x}", addr)
        }

        if let Some(offset) = map::PAD_MEMCARD.contains(abs_addr) {
            panic!("Unhandled PAD_MEMCARD load at address {:08x}", addr)
        }

        if let Some(offset) = map::EXPANSION_1.contains(abs_addr) {
            // No expansion implemented
            return 0xff;
        }

        if let Some(_) = map::RAM_SIZE.contains(abs_addr) {
            panic!("Unhandled RAM_SIZE load at address {:08x}", addr)
        }

        if let Some(offset) = map::MEM_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled MEM_CONTROL access ({})", A::size());
            }

            let index = (offset >> 2) as usize;

            panic!("Unhandled MEM_CONTROL load at address {:08x}", addr)
        }

        if let Some(_) = map::CACHE_CONTROL.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled cache control access ({})", A::size());
            }

            panic!("Unhandled CACHE_CONTROL load at address {:08x}", addr)
        }

        if let Some(offset) = map::EXPANSION_2.contains(abs_addr) {
            panic!("Unhandled EXPANSION_2 load at address {:08x}", addr)
        }

        panic!("unhandled load at address {:08x}", addr);
    }

    /// Interconnect: store `val` into `addr`
    pub fn store<A: Addressable>(&mut self, addr: u32, val: u32) {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::RAM.contains(abs_addr) {
            self.ram.store::<A>(offset, val);
            return;
        }

        if let Some(offset) = map::SCRATCH_PAD.contains(abs_addr) {
            if addr > 0xa0000000 {
                panic!("ScratchPad access through uncached memory");
            }

            panic!("Unhandled write to SCRATCH_PAD {:x}", offset);
        }

        if let Some(offset) = map::IRQ_CONTROL.contains(abs_addr) {
            warn!("Unhandled IRQ control: {:x} <− {:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::DMA.contains(abs_addr) {
            panic!("Unhandled write to DMA {:x}", offset);
        }

        if let Some(offset) = map::GPU.contains(abs_addr) {
            panic!("Unhandled write to GPU {:x}", offset);
        }

        if let Some(offset) = map::TIMERS.contains(abs_addr) {
            warn!("Unhandled TIMERS control: {:x} <− {:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::CDROM.contains(abs_addr) {
            panic!("Unhandled write to CDROM {:x}", offset);
        }

        if let Some(offset) = map::MDEC.contains(abs_addr) {
            panic!("Unhandled write to MDEC {:x}", offset);
        }

        if let Some(offset) = map::SPU.contains(abs_addr) {
            warn!("SPU control: {:x} <− {:08x}", offset, val);
            return;
        }

        if let Some(offset) = map::PAD_MEMCARD.contains(abs_addr) {
            panic!("Unhandled write to PAD_MEMCARD {:x}", offset);
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
                    warn!("Unhandled write to MEM_CONTROL register {:x}: \
                           0x{:08x}",
                          offset, val),
            }
            return;
        }

        if let Some(_) = map::RAM_SIZE.contains(abs_addr) {
            if A::size() != 4 {
                panic!("Unhandled RAM_SIZE access");
            }
            return;
        }

        if let Some(offset) = map::EXPANSION_2.contains(abs_addr) {
            return;
        }

        panic!("unhandled store into address 0x{:08x}: {:08x}", addr, val);
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

    /// Memory latency and expansion mapping
    pub const MEM_CONTROL: Range = Range(0x1f801000, 36);

    /// Gamepad and memory card controller
    pub const PAD_MEMCARD: Range = Range(0x1f801040, 32);

    /// Register that has something to do with RAM configuration,
    /// configured by the BIOS
    pub const RAM_SIZE: Range = Range(0x1f801060, 4);

    /// Interrupt Control registers (status and mask)
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

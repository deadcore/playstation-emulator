use crate::bios::Bios;
use crate::ram::Ram;

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

    /// Store 8bit value into the memory
    pub fn store8(&mut self, addr: u32, val: u8) {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::EXPANSION2.contains(abs_addr) {
            warn!("Unhandled write to expansion 2 register {:x}", offset);
            return;
        }

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.store8(offset, val);
        }

        panic!("unhandled store8 into address {:08x}", addr);
    }

    /// Store 16bit value into the memory
    pub fn store16(&mut self, addr: u32, val: u16) {
        if addr % 2 != 0 {
            panic!("Unaligned store16 address {:08x}", addr)
        }

        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::SPU.contains(abs_addr) {
            warn!("Unhandled write to SPU register {:x}", offset);
            return;
        }

        panic!("unhandled store16 into address {:08x}", addr);
    }

    /// Store 32bit value into the memory
    pub fn store32(&mut self, addr: u32, val: u32) {
        if addr % 4 != 0 {
            panic!("Unaligned store32 address {:08x}", addr)
        }

        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::MEMCONTROL.contains(abs_addr) {
            match offset {
                0 => { // Expansion 1 base address
                    if val != 0x1f000000 {
                        panic!("Bad expansion 1 base address: 0x{:08x}", val)
                    }
                }

                4 => { // Expansion 2 base address
                    if val != 0x1f802000 {
                        panic!("Bad expansion 2 base address: 0x{:08x}", val)
                    }
                }
                _ => warn!("Unhandled write to MEMCONTROL register")
            }
            return;
        }

        if let Some(_) = map::RAMSIZE.contains(abs_addr) {
            // We ignore writes at this address
            warn!("Unhandled write to RAMSIZE register");
            return;
        }

        if let Some(_) = map::CACHECONTROL.contains(abs_addr) {
            // We ignore writes at this address
            warn!("Unhandled write to CACHECONTROL register");
            return;
        }

        if let Some(_) = map::RAM.contains(abs_addr) {
            self.ram.store32(addr, val);
            return;
        }


        if let Some(offset) = map::IRQCONTROL.contains(abs_addr) {
            warn!("IRQ control: {:x} <− {:08x}", offset, val);
            return;
        }

        panic!("unhandled store32 into address 0x{:08x}", abs_addr)
    }

    pub fn load8(&self, addr: u32) -> u8 {
        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::BIOS.contains(abs_addr) {
            return self.bios.load8(offset);
        }

        if let Some(_) = map::EXPANSION1.contains(abs_addr) {
            // No expansion implemented
            return 0xff;
        }

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.load8(offset);
        }

        panic!("unhandled load8 at address {:08x}", addr);
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned load32 address 0x{:08x}", addr)
        }

        let abs_addr = map::mask_region(addr);

        if let Some(offset) = map::BIOS.contains(abs_addr) {
            return self.bios.load32(offset);
        }

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.load32(offset);
        }

        panic!("unhandled fetch32 at address 0x{:08x}", abs_addr);
    }
}

mod map {
    pub struct Range(u32, u32);

    impl Range {
        /// Return ‘Some(offset)‘ if addr is contained in ‘self ‘
        pub fn contains(self, addr: u32) -> Option<u32> {
            let Range(start, length) = self;
            if addr >= start && addr < start + length {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const RAM: Range = Range(0x00000000, 2 * 1024 * 1024);
    pub const BIOS: Range = Range(0x1fc00000, 512 * 1024);

    /// SPU (Sound Processing Unit) registers
    pub const SPU: Range = Range(0x1f801c00, 640);

    /// Unknown registers . The name comes from mednafen.
    pub const SYS_CONTROL: Range = Range(0x1f801000, 36);

    /// Interrupt Control registers (status and mask)
    pub const IRQCONTROL: Range = Range(0x1f801070, 8);

    /// Register that has something to do with RAM configuration
    /// configured by the BIOS
    pub const RAMSIZE: Range = Range(0x1f801060, 4);

    /// Cache control register
    pub const CACHECONTROL: Range = Range(0xfffe0130, 4);

    /// Memory latency and expansion mapping
    pub const MEMCONTROL: Range = Range(0x1f801000, 36);

    /// Expansion region 1
    pub const EXPANSION1: Range = Range(0x1f000000, 512 * 1024);

    /// Expansion region 2
    pub const EXPANSION2: Range = Range(0x1f802000, 66);

    const REGION_MASK: [u32; 8] = [
        // KUSEG: 2048MB
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
        // KSEG0: 512MB
        0x7fffffff,
        // KSEG1: 512MB
        0x1fffffff,
        // KSEG2: 1024MB
        0xffffffff, 0xffffffff,
    ];

    /// Mask a CPU address to remove the region bits .
    pub fn mask_region(addr: u32) -> u32 {
        // Index address space in 512MB chunks
        let index = (addr >> 29) as usize;
        addr & REGION_MASK[index]
    }
}
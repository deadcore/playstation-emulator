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

    /// Store 16bit value into the memory
    pub fn store16(&mut self, addr: u32, _: u16) {
        if addr % 4 != 0 {
            panic!("Unaligned store16 address {:08x}", addr)
        }

        panic!("unhandled store16 into address {:08x}", addr);
    }

    /// Store 32bit value into the memory
    pub fn store32(&mut self, addr: u32, val: u32) {
        if addr % 4 != 0 {
            panic!("Unaligned store32 address {:08x}", addr)
        }

        let abs_addr = mask_region(addr);

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

        panic!("unhandled store32 into address 0x{:08x}", abs_addr)
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned load32 address 0x{:08x}", addr)
        }

        let abs_addr = mask_region(addr);

        if let Some(offset) = map::BIOS.contains(abs_addr) {
            return self.bios.load32(offset);
        }

        if let Some(offset) = map::RAM.contains(abs_addr) {
            return self.ram.load32(offset);
        }

        panic!("unhandled fetch32 at address 0x{:08x}", abs_addr);
    }
}

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

    /// Unknown registers . The name comes from mednafen.
//    pub const SYS_CONTROL: Range = Range(0x1f801000, 36);

    /// Register that has something to do with RAM configuration
    /// configured by the BIOS
    pub const RAMSIZE: Range = Range(0x1f801060, 4);

    /// Cache control register
    pub const CACHECONTROL: Range = Range(0xfffe0130, 4);

    /// Memory latency and expansion mapping
    pub const MEMCONTROL: Range = Range(0x1f801000, 36);
}
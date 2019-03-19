use crate::bios::Bios;
use crate::ram::Ram;

/// Global interconnect
pub struct Interconnect {
    /// Basic Input/Output memory
    bios: Bios,

    ram: Ram
}

impl Interconnect {
    pub fn new(bios: Bios, ram: Ram) -> Interconnect {
        Interconnect {
            bios,
            ram
        }
    }

    pub fn store32(&mut self, addr: u32, val: u32) {
        if addr % 4 != 0 {
            panic!("Unaligned store32 address {:08x}", addr)
        }

        if let Some(offset) = map::MEMCONTROL.contains(addr) {
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

        if let Some(_) = map::RAMSIZE.contains(addr) {
            // We ignore writes at this address
            warn!("Unhandled write to RAMSIZE register");
            return;
        }

        if let Some(_) = map::CACHECONTROL.contains(addr) {
            // We ignore writes at this address
            warn!("Unhandled write to CACHECONTROL register");
            return;
        }

        if let Some(_) = map::RAM.contains(addr) {
            self.ram.store32(addr, val);
            return;
        }

        panic!("unhandled store32 into address 0x{:08x}", addr)
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned load32 address 0x{:08x}", addr)
        }

        if let Some(offset) = map::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }

        if let Some(offset) = map::RAM.contains(addr) {
            return self.ram.load32(offset);
        }

        panic!("unhandled fetch32 at address 0x{:08x}", addr);
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

    pub const RAM: Range = Range(0xa0000000, 2 * 1024 * 1024);

    /// Cache control register
    pub const CACHECONTROL: Range = Range(0xfffe0130, 4);

    /// Register that has something to do with RAM configuration
    /// configured by the BIOS
    pub const RAMSIZE: Range = Range(0x1f801060, 4);
    pub const MEMCONTROL: Range = Range(0x1f801000, 36);
    pub const BIOS: Range = Range(0xbfc00000, 512 * 1024);
}
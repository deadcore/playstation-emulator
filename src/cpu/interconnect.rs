use crate::bios::Bios;

/// Global interconnect
pub struct Interconnect {
    /// Basic Input/Output memory
    bios: Bios,
}

impl Interconnect {
    pub fn new(bios: Bios) -> Interconnect {
        Interconnect {
            bios,
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
                        panic!("Bad expansion 1 base address: 0x {:08x}", val)
                    }
                }

                4 => { // Expansion 2 base address
                    if val != 0x1f802000 {
                        panic!("Bad expansion 2 base address: 0x {:08x}", val)
                    }
                }
                _ => info!("Unhandled write to MEMCONTROL register")
            }
        }

        panic!("unhandled store32 into address {:08x}", addr)
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned load32 address {:08x}", addr)
        }

        if let Some(offset) = map::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }
        panic!("unhandled fetch32 at address {:08x}", addr);
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

    pub const MEMCONTROL: Range = Range(0x1f801000, 36);

    pub const BIOS: Range = Range(0xbfc00000, 512 * 1024);
}
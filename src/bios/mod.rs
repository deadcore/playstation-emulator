use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::Read;
use std::path::Path;

/// BIOS image
pub struct Bios {
    /// BIOS memory
    data: Vec<u8>
}

/// BIOS images are always 512KB in length
const BIOS_SIZE: u64 = 512 * 1024;

impl Bios {
    /// Load a BIOS image from the file located at ‘path‘
    pub fn new(path: &Path) -> Result<Bios> {
        let file = File::open(path)?;
        let mut data = Vec::new();
        // Load the BIOS
        file.take(BIOS_SIZE).read_to_end(&mut data)?;
        if data.len() == BIOS_SIZE as usize {
            Ok(Bios {
                data
            })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid BIOS size"))
        }
    }

    /// Fetch the 32bit little endian word at ‘offset ‘
    pub fn load32(&self, offset: u32) -> u32 {
        let offset = offset as usize;
        let b0 = self.data[offset + 0] as u32;
        let b1 = self.data[offset + 1] as u32;
        let b2 = self.data[offset + 2] as u32;
        let b3 = self.data[offset + 3] as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    pub fn load8(&self, offset: u32) -> u8 {
        self.data[offset as usize]
    }
}
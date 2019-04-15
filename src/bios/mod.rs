use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::Read;
use std::path::Path;

use crate::memory::Addressable;

/// BIOS image
pub struct Bios {
    /// BIOS memory
    data: Vec<u8>
}

/// BIOS images are always 512KB in length
const BIOS_SIZE: u64 = 512 * 1024;

impl Bios {
    /// Load a BIOS image from the file located at 'path'
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

    /// fetch the little endian value at `offset`
    pub fn load<T: Addressable>(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        let mut r = 0;

        for i in 0..T::size() as usize {
            r |= (self.data[offset + i] as u32) << (8 * i)
        }

        r
    }
}
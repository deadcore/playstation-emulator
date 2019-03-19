/// Main PlayStation RAM: 2Megabytes
const RAM_SIZE: usize = 2 * 1024 * 1024;

pub struct Ram {
    data: Box<[u8; RAM_SIZE]>
}

impl Ram {
    /// Instantiate main RAM with garbage values
    pub fn new() -> Ram {
        Ram {
            data: box_array![0xca; RAM_SIZE]
        }
    }

    /// Fetch the 32bit little endian word at ‘offset ‘
    pub fn load32(&self, offset: u32) -> u32 {
        // The two MSB are ignored, the 2MB RAM is mirorred four times
        // over the first 8MB of address space
        let offset = (offset & 0x1fffff) as usize;

        let mut v = 0;

        for i in 0..4 as usize {
            v |= (self.data[offset + i] as u32) << (i * 8)
        }

        v
    }

    /// Store the 32bit little endian word ‘val ‘ into ‘offset ‘
    pub fn store32(&mut self, offset: u32, val: u32) {
        // The two MSB are ignored, the 2MB RAM is mirorred four times
        // over the first 8MB of address space
        let offset = (offset & 0x1fffff) as usize;

        for i in 0..4 as usize {
            self.data[offset + i] = (val >> (i * 8)) as u8;
        }
    }
}
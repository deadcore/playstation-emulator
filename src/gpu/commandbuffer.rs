pub struct CommandBuffer {
    /// Command buffer: the longest possible command is GP0(0x3E)
    /// which takes 12 parameters
    buffer: [u32; 12],
    /// Number of words queued in buffer
    len: u8,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer {
            buffer: [0; 12],
            len: 0,
        }
    }

    /// Clear the command buffer
    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push_word(&mut self, word: u32) {
        self.buffer[self.len as usize] = word;
        self.len += 1;
    }
}

/// Overrides the [] to allow access to the buffer
impl ::std::ops::Index<usize> for CommandBuffer {
    type Output = u32;

    fn index<'a>(&'a self, index: usize) -> &'a u32 {
        if index >= self.len as usize {
            panic!("Command buffer inde out of range {} ({})", index, self.len)
        }

        &self.buffer[index]
    }
}
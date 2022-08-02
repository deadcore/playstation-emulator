/// Video output horizontal resolution
#[derive(Copy, Clone)]
pub struct HorizontalRes(u8);

impl HorizontalRes {
    /// Create a new HorizontalRes instance from the 2 bit field ‘hr1‘
    /// And the one bit field ‘hr2‘
    pub fn from_fields(hr1: u8, hr2: u8) -> HorizontalRes {
        let hr = (hr2 & 1) | ((hr1 & 3) << 1);
        HorizontalRes(hr)
    }

    /// Retrieve value of bits [18:16] of the status register
    pub fn into_status(self) -> u32 {
        let HorizontalRes(hr) = self;
        (hr as u32) << 16
    }
}

/// Video output vertical resolution
#[derive(Copy, Clone)]
pub enum VerticalRes {
    /// 240 lines
    Y240Lines = 0,
    /// 480 lines (only available for interlaced output)
    Y480Lines = 1,
}
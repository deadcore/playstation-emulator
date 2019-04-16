/// Depth of the pixel values in a texture page
#[derive(Copy, Clone)]
pub enum TextureDepth {
    /// 4 bits per pixel
    T4Bit = 0,
    /// 8 bits per pixel
    T8Bit = 1,
    /// 15 bits per pixel
    T15Bit = 2,
}
/// Video Modes
#[derive(Copy, Clone)]
pub enum VMode {
    /// NTSC: 480i60H
    Ntsc = 0,
    /// PAL: 576i50Hz
    Pal = 1,
}
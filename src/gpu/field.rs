/// Interlaced output splits each frame in two fields
#[derive(Copy, Clone)]
pub enum Field {
    /// Top field (odd lines).
    Top = 1,
    /// Bottom field (even lines )
    Bottom = 0,
}
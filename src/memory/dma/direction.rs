/// DMA transfer direction
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    ToRam = 0,
    FromRam = 1,
}
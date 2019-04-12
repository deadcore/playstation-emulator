/// DMA transfer direction
#[derive(Clone, Copy)]
pub enum Direction {
    ToRam = 0,
    FromRam = 1,
}
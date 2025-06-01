#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Normal,
    Flat,
}

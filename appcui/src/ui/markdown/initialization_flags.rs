#[repr(u8)]
#[derive(Eq,PartialEq, Copy, Clone)]
pub enum Flags {
    None,
    ScrollBars
}
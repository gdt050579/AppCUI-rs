#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Checked,
    Unchecked,
    Unknown,
}

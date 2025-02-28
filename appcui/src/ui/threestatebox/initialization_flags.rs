#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
    Checked,
    Unchecked,
    Unknown,
}

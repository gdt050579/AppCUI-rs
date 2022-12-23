#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseButton {
    None = 0,
    Left,
    Right,
    Center
}
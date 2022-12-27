#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseWheelDirection {
    None = 0,
    Left,
    Right,
    Up,
    Down
}
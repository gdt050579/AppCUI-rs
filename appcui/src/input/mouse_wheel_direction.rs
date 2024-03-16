#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseWheelDirection {
    None = 0,
    Left,
    Right,
    Up,
    Down
}

impl MouseWheelDirection {
    pub fn name(&self)->&str {
        match self {
            MouseWheelDirection::None => "",
            MouseWheelDirection::Left => "left",
            MouseWheelDirection::Right => "right",
            MouseWheelDirection::Up => "up",
            MouseWheelDirection::Down => "down",
        }
    }
}
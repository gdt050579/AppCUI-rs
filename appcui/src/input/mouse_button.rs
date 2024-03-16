#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseButton {
    None = 0,
    Left,
    Right,
    Center
}
impl MouseButton {
    pub fn name(&self)->&str {
        match self {
            MouseButton::None => "",
            MouseButton::Left => "left",
            MouseButton::Right => "right",
            MouseButton::Center => "center",
        }
    }
}
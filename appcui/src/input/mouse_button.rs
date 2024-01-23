#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseButton {
    None = 0,
    Left,
    Right,
    Center
}
impl MouseButton {
    pub fn get_name(&self)->&str {
        match self {
            MouseButton::None => "",
            MouseButton::Left => "left",
            MouseButton::Right => "right",
            MouseButton::Center => "center",
        }
    }
}
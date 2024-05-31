use crate::graphics::Point;
use super::KeyModifier;
use super::MouseButton;
use super::MouseWheelDirection;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseEventData {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub modifier: KeyModifier
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent {
    Enter,
    Leave,
    Over(Point),
    Pressed(MouseEventData),
    Released(MouseEventData),
    DoubleClick(MouseEventData),
    Drag(MouseEventData),
    Wheel(MouseWheelDirection)
}



use super::MouseButton;
use super::MouseWheelDirection;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent {
    Enter,
    Leave,
    Over,
    Pressed,
    Released,
    DoubleClick,
    Drag,
    Wheel
}



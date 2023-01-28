use super::MouseButton;
use super::MouseEventType;
use super::MouseWheelDirection;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent {
    Enter,
    Leave,
    Over(crate::graphics::Point),

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseEvent_old {
    pub button: MouseButton,
    pub event: MouseEventType,
    pub wheel_direction: MouseWheelDirection,
    pub x: i32,
    pub y: i32,
}

impl Default for MouseEvent_old {
    fn default() -> Self {
        Self {
            button: MouseButton::None,
            event: MouseEventType::Move,
            wheel_direction: MouseWheelDirection::None,
            x: 0,
            y: 0,
        }
    }
}

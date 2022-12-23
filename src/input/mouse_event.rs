use super::MouseButton;
use super::MouseEventType;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub event: MouseEventType,
    pub x: i32,
    pub y: i32,
}


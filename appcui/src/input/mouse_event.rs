use crate::graphics::Point;
use super::KeyModifier;
use super::MouseButton;
use super::MouseWheelDirection;

/// Represents the data associated with a mouse event.
/// 
/// This struct contains information about the position of the mouse,
/// the button that was pressed, and the modifier keys (e.g., shift, ctrl, alt).
/// 
/// # Fields
/// * `x` - The x-coordinate of the mouse position.
/// * `y` - The y-coordinate of the mouse position.
/// * `button` - The button that was pressed.
/// * `modifier` - The modifier keys (e.g., shift, ctrl, alt).  
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseEventData {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub modifier: KeyModifier
}

/// Represents the type of mouse event.
/// 
/// This enum defines the possible types of mouse events that can occur.
/// 
/// # Values
/// * `Enter` - The mouse pointer has entered a control .
/// * `Leave` - The mouse pointer has left a control.
/// * `Over` - The mouse pointer is over a control.
/// * `Pressed` - A mouse button has been pressed.
/// * `Released` - A mouse button has been released.
/// * `DoubleClick` - A mouse button has been double-clicked.
/// * `Drag` - The mouse pointer is being dragged.
/// * `Wheel` - The mouse wheel has been rotated.   
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



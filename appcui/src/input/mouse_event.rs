use crate::graphics::Point;
use super::KeyModifier;
use super::MouseButton;
use super::MouseWheelDirection;

/// A mouse position, button, and modifier snapshot for press, release, drag, and click events.
///
/// Coordinates are in character cells relative to the control that receives the event.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MouseEventData {
    /// Horizontal position in character cells.
    pub x: i32,
    /// Vertical position in character cells.
    pub y: i32,
    /// Button involved in the event.
    pub button: MouseButton,
    /// Alt, Ctrl, and/or Shift held during the event.
    pub modifier: KeyModifier
}

/// A mouse action delivered to a control.
///
/// Pointer enter/leave/hover carry little or no extra data. Button actions include
/// [`MouseEventData`]. Wheel motion uses [`MouseWheelDirection`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent {
    /// The pointer entered the control.
    Enter,
    /// The pointer left the control.
    Leave,
    /// The pointer is over the control at the given cell.
    Over(Point),
    /// A mouse button was pressed.
    Pressed(MouseEventData),
    /// A mouse button was released.
    Released(MouseEventData),
    /// A mouse button was double-clicked.
    DoubleClick(MouseEventData),
    /// The pointer is dragging with a button held.
    Drag(MouseEventData),
    /// The mouse wheel moved (`↑` / `↓` / `←` / `→`).
    Wheel(MouseWheelDirection)
}



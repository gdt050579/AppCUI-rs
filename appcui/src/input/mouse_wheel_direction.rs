/// Represents the direction of a mouse wheel event.
///
/// This enum defines the possible directions for a mouse wheel event.
///
/// # Values
/// * `Left` - The wheel was rotated to the left.
/// * `Right` - The wheel was rotated to the right.
/// * `Up` - The wheel was rotated up.
/// * `Down` - The wheel was rotated down.  
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum MouseWheelDirection {
    Left,
    Right,
    Up,
    Down,
}

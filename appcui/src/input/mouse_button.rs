/// Represents the type of mouse button that was pressed.
/// 
/// This enum defines the possible types of mouse buttons that can be pressed.
/// 
/// # Values
/// * `None` - No button was pressed.
/// * `Left` - The left mouse button was pressed.
/// * `Right` - The right mouse button was pressed.
/// * `Center` - The center mouse button was pressed.   
#[derive(Copy,Clone,Debug,PartialEq, Eq)]
#[repr(u8)]
pub enum MouseButton {
    None = 0,
    Left,
    Right,
    Center
}
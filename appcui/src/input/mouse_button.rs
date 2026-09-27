/// Which mouse button was involved in an event.
///
/// [`None`](Self::None) means no button (for example a move without a press).
/// [`Center`](Self::Center) is the middle / wheel button.
#[derive(Copy,Clone,Debug,PartialEq, Eq)]
#[repr(u8)]
pub enum MouseButton {
    /// No button.
    None = 0,
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle (wheel) button.
    Center
}

#[cfg(feature = "EVENT_RECORDER")]
impl MouseButton {
    pub(crate) fn name(&self)->&'static str {
        match self {
            MouseButton::None => "None",
            MouseButton::Left => "Left",
            MouseButton::Right => "Right",
            MouseButton::Center => "Center",
        }
    }
}
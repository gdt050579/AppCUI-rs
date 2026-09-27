/// Direction of a mouse-wheel movement.
///
/// Vertical wheels typically report [`Up`](Self::Up) and [`Down`](Self::Down).
/// Tilt wheels may also report [`Left`](Self::Left) and [`Right`](Self::Right).
#[derive(Copy,Clone,Debug,PartialEq)]
#[repr(u8)]
pub enum MouseWheelDirection {
    /// Tilt or scroll left (`←`).
    Left,
    /// Tilt or scroll right (`→`).
    Right,
    /// Scroll up (`↑`).
    Up,
    /// Scroll down (`↓`).
    Down
}
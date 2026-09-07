use crate::prelude::EnumSelector;

/// Which point of a control is attached to the `(x, y)` reference in pivot layout.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumSelector)]
pub enum Pivot {
    /// Attach the control's top-left corner to `(x, y)`.
    TopLeft,
    /// Attach the control's top-right corner to `(x, y)`.
    TopRight,
    /// Attach the control's bottom-left corner to `(x, y)`.
    BottomLeft,
    /// Attach the control's bottom-right corner to `(x, y)`.
    BottomRight,
    /// Attach the midpoint of the control's bottom edge to `(x, y)`.
    BottomCenter,
    /// Attach the midpoint of the control's left edge to `(x, y)`.
    CenterLeft,
    /// Attach the midpoint of the control's right edge to `(x, y)`.
    CenterRight,
    /// Attach the midpoint of the control's top edge to `(x, y)`.
    TopCenter,
    /// Attach the control's center to `(x, y)`.
    Center,
}
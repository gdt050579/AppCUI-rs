use crate::prelude::EnumSelector;

/// Which point of a control is attached to the `(x, y)` reference in pivot layout.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumSelector)]
pub enum Pivot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    BottomCenter,
    CenterLeft,
    CenterRight,
    TopCenter,
    Center,
}
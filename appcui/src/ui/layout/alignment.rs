use crate::prelude::EnumSelector;

/// Alignment of a control inside its parent when using aligned layout (corners, edges, or center).
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumSelector)]
pub enum Alignment {
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
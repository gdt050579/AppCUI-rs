use crate::prelude::EnumSelector;

/// Alignment of a control inside its parent when using aligned layout (corners, edges, or center).
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumSelector)]
pub enum Alignment {
    /// Top-left corner of the parent.
    TopLeft,
    /// Top-right corner of the parent.
    TopRight,
    /// Bottom-left corner of the parent.
    BottomLeft,
    /// Bottom-right corner of the parent.
    BottomRight,
    /// Center of the parent's bottom edge.
    BottomCenter,
    /// Center of the parent's left edge.
    CenterLeft,
    /// Center of the parent's right edge.
    CenterRight,
    /// Center of the parent's top edge.
    TopCenter,
    /// Center of the parent.
    Center,
}
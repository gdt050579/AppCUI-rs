use crate::prelude::EnumSelector;


/// Side of the parent that a control attaches to in docked layout (`Left`, `Right`, `Top`, `Bottom`, or `Fill`).
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq, EnumSelector)]
pub enum Dock {
    /// Stretch along the left edge.
    Left,
    /// Stretch along the right edge.
    Right,
    /// Stretch along the top edge.
    Top,
    /// Stretch along the bottom edge.
    Bottom,
    /// Fill the remaining client area.
    Fill,
}

use crate::prelude::EnumSelector;


/// Side of the parent that a control attaches to in docked layout (`Left`, `Right`, `Top`, `Bottom`, or `Fill`).
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq, EnumSelector)]
pub enum Dock {
    Left,
    Right,
    Top,
    Bottom,
    Fill,
}

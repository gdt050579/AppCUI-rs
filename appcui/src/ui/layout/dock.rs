use crate::prelude::EnumSelector;


#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq, EnumSelector)]
pub enum Dock {
    Left,
    Right,
    Top,
    Bottom,
    Fill,
}

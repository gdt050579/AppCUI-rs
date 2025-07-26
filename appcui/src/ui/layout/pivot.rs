use crate::prelude::EnumSelector;

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
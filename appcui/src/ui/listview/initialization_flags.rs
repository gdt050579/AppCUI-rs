use std::cmp::Ordering;

use EnumBitFlags::EnumBitFlags;

use super::RenderMethod;
use crate::prelude::{Surface, Theme};

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    ShowGroups = 0x08,
    DisplayEmptyGroups = 0x10,
    SmallIcon = 0x20,
    LargeIcon = 0x40,
}

pub trait ListItem {
    fn paint(&self, _column_index: u32, _width: u16, _surface: &mut Surface, _theme: &Theme) {}
    fn render_method(&self, column_index: u16) -> Option<RenderMethod>;
    fn compare(&self, other: &Self, column_index: u16) -> Ordering;
}

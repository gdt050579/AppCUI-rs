use std::cmp::Ordering;

use EnumBitFlags::EnumBitFlags;

use super::RenderMethod;
use crate::prelude::{Surface, Theme};

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    Groups = 0x08,
    SmallIcon = 0x10,
    LargeIcon = 0x20,
}

pub trait ListItem {
    fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, theme: &Theme) {}
    fn render_method(&self, column_index: u16) -> Option<RenderMethod>;
    fn compare(&self, other: &Self, column_index: u16) -> Ordering;
}

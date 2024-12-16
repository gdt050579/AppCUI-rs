use std::cmp::Ordering;

use EnumBitFlags::EnumBitFlags;

use super::RenderMethod;
use crate::prelude::{CharAttribute, Column, Surface, TextAlignament, Theme};

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    ShowGroups = 0x08,
    SmallIcons = 0x10,
    LargeIcons = 0x20,
    CustomFilter = 0x40,
    NoSelection = 0x80,
}

pub trait ListItem {
    const COLUMNS_COUNT: u16 = 0;
    fn column(_index: u16) -> Column{ Column::new("", 10, TextAlignament::Left) }

    fn paint(&self, _column_index: u32, _width: u16, _surface: &mut Surface, _theme: &Theme, _attr: Option<CharAttribute>) {}
    fn render_method(&self, column_index: u16) -> Option<RenderMethod>;
    fn compare(&self, _other: &Self, _column_index: u16) -> Ordering {
        Ordering::Equal
    }
    fn matches(&self, _text: &str) -> bool {
        true
    }
}

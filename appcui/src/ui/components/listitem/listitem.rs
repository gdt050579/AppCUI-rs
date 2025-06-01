use super::super::Column;
use super::RenderMethod;
use crate::graphics::{CharAttribute, Surface, TextAlignament};
use crate::system::Theme;
use std::cmp::Ordering;

pub trait ListItem {
    fn columns_count() -> u16 {
        0
    }
    fn column(_index: u16) -> Column {
        Column::new("", 10, TextAlignament::Left)
    }

    fn paint(&self, _column_index: u32, _width: u16, _surface: &mut Surface, _theme: &Theme, _attr: Option<CharAttribute>) {}
    fn render_method(&self, column_index: u16) -> Option<RenderMethod>;
    fn compare(&self, _other: &Self, _column_index: u16) -> Ordering {
        Ordering::Equal
    }
    fn matches(&self, _text: &str) -> bool {
        true
    }
}

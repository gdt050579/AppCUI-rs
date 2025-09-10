use super::super::Column;
use crate::graphics::{Surface, TextAlignment, CharAttribute};
use crate::system::Theme;
use super::RenderMethod;
use std::cmp::Ordering;

pub trait ListItem {
    fn columns_count() -> u16 {
        0
    }
    fn column(_index: u16) -> Column {
        Column::new("", 10, TextAlignment::Left)
    }

    fn paint(&self, _column_index: u32, _width: u16, _surface: &mut Surface, _theme: &Theme, _attr: Option<CharAttribute>) {}
    fn render_method(&'_ self, column_index: u16) -> Option<RenderMethod<'_>>;
    fn compare(&self, _other: &Self, _column_index: u16) -> Ordering {
        Ordering::Equal
    }
    fn matches(&self, _text: &str) -> bool {
        true
    }
}

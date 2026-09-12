use super::super::Column;
use crate::graphics::{Surface, TextAlignment, CharAttribute};
use crate::system::Theme;
use super::RenderMethod;
use std::cmp::Ordering;

/// A row type displayed by [`ListView`](crate::ui::listview::ListView) and similar list controls.
///
/// Implement this for your item. [`columns_count`](Self::columns_count) and
/// [`column`](Self::column) describe the header, [`render_method`](Self::render_method)
/// (or [`paint`](Self::paint)) draws each cell, [`compare`](Self::compare) sorts, and
/// [`matches`](Self::matches) filters.
pub trait ListItem {
    /// Number of columns this item type exposes. Defaults to `0`.
    fn columns_count() -> u16 {
        0
    }
    /// Header definition for the column at `index`.
    fn column(_index: u16) -> Column {
        Column::new("", 10, TextAlignment::Left)
    }

    /// Optional custom paint for the cell at `column_index`. The default does nothing.
    fn paint(&self, _column_index: u32, _width: u16, _surface: &mut Surface, _theme: &Theme, _attr: Option<CharAttribute>) {}
    /// How this item should be drawn in `column_index`, or `None` to use [`paint`](Self::paint).
    fn render_method(&'_ self, column_index: u16) -> Option<RenderMethod<'_>>;
    /// Ordering versus `other` when sorting by `column_index`. Defaults to equal.
    fn compare(&self, _other: &Self, _column_index: u16) -> Ordering {
        Ordering::Equal
    }
    /// Returns whether this item matches a search `text`. Defaults to `true` (always visible).
    fn matches(&self, _text: &str) -> bool {
        true
    }
}

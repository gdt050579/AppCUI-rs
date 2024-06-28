use super::Flags;
use components::{Column, ColumnsHeader};
use listview::initialization_flags::ListItem;
use AppCUIProcMacro::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct ListView<T>
where
    T: ListItem,
{
    flags: Flags,
    data: Vec<T>,
    header: ColumnsHeader,
}

impl<T> ListView<T>
where
    T: ListItem,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            flags,
            data: Vec::new(),
            header: ColumnsHeader::with_capacity(4),
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }
}

impl<T> OnPaint for ListView<T> where T: ListItem {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.header.paint(surface, theme, &self.base);
        self.header.paint_columns(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T> where T: ListItem {}

impl<T> OnMouseEvent for ListView<T> where T: ListItem {}

use super::Flags;
use components::{Column, ColumnsHeader, ColumnsHeaderAction, ListScrollBars};
use listview::initialization_flags::ListItem;
use AppCUIProcMacro::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct ListView<T>
where
    T: ListItem,
{
    flags: Flags,
    data: Vec<T>,
    header: ColumnsHeader,
    comp: ListScrollBars,
}

impl<T> ListView<T>
where
    T: ListItem,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }

        Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            data: Vec::new(),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }
    fn sort_elements(&mut self, column_index: u16, ascendent: bool) {
        // sort elements by column index
    }
    fn autoresize_column(&mut self, column_index: u16) {
        // auto resize column
    }
}

impl<T> OnPaint for ListView<T> where T: ListItem {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.header.paint(surface, theme, &self.base);
        self.header.paint_columns(surface, theme, &self.base);
        self.comp.paint(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T> where T: ListItem {}

impl<T> OnMouseEvent for ListView<T> where T: ListItem {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let result = self.header.process_mouse_event(event);
        match result {
            ColumnsHeaderAction::Sort((index,ascendent)) => {
                self.sort_elements(index, ascendent);
                return EventProcessStatus::Processed;
            }
            ColumnsHeaderAction::AutoResize(index) => {
                self.autoresize_column(index);
                return EventProcessStatus::Processed;
            }
            ColumnsHeaderAction::ResizeColumn => {
                // nothing to do - exit with Processed
                return EventProcessStatus::Processed;
            }
            ColumnsHeaderAction::None => {}
            ColumnsHeaderAction::Repaint => {}
        }
        // process mouse event for items
        if result.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnResize for ListView<T> where T: ListItem {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.comp.resize(self.header.width() as u64, 0, &self.base);
    }
}
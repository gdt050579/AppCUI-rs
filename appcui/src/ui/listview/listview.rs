use super::Flags;
use components::{Column, ColumnsHeader, ColumnsHeaderAction, ListScrollBars};
use listview::initialization_flags::ListItem;
use AppCUIProcMacro::*;


struct Item<T> where T: ListItem {
    data: T,
    selected: bool,
}
enum Filter {
    Item(u32),
    Group(u32),
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct ListView<T>
where
    T: ListItem,
{
    flags: Flags,
    data: Vec<Item<T>>,
    filter: Vec<Filter>,
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
            filter: Vec::new(),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }
    pub fn add(&mut self, item: T) {
        self.data.push(Item {
            data: item,
            selected: false,
        });
        // refiltering is required
    }
    pub fn add_items(&mut self, items: Vec<T>) {
        self.data.reserve(items.len());
        self.filter.reserve(items.len());
        for item in items {
            self.data.push(Item {
                data: item,
                selected: false,
            });
        }
        // refiltering is required
    }
    pub fn set_frozen_columns(&mut self, count: u16) {
        self.header.set_frozen_columns(count);
        self.update_scrollbars();
    }
    fn sort_elements(&mut self, column_index: u16, ascendent: bool) {
        // sort elements by column index
    }
    fn autoresize_column(&mut self, column_index: u16) {
        // auto resize column
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.header.scroll_to(self.comp.horizontal_index() as u32);
        //self.top_view = (self.comp.vertical_index() as usize).min(self.items.len().saturating_sub(1));
        //self.update_left_position_for_items();
    }
    fn update_scrollbars(&mut self) {
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base);
        self.comp.set_indexes(self.header.scroll_pos() as u64 , 0);
    }
    fn execute_column_header_action(&mut self, action: ColumnsHeaderAction)->bool {
        match action {
            ColumnsHeaderAction::Sort((index,ascendent)) => {
                self.sort_elements(index, ascendent);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::AutoResize(index) => {
                self.autoresize_column(index);
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::ResizeColumn => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::UpdateScroll => {
                self.update_scrollbars();
                true
            }
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
        }
    }
}

impl<T> OnPaint for ListView<T> where T: ListItem {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.header.paint(surface, theme, &self.base);
        self.header.paint_columns(surface, theme, &self.base);
        self.comp.paint(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T> where T: ListItem {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let action = self.header.process_key_pressed(key);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }
        if self.comp.process_key_pressed(key, character) {
            return EventProcessStatus::Processed;
        }
        // process key for items
        match key.value() {
            key!("Ctrl+Left") | key!("Ctrl+Right") => {
                self.header.enter_resize_mode();
                return EventProcessStatus::Processed;
            }  
            _ => {} 
        }
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl<T> OnMouseEvent for ListView<T> where T: ListItem {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let action = self.header.process_mouse_event(event);
        if self.execute_column_header_action(action) {
            return EventProcessStatus::Processed;
        }

        // process mouse event for items
        if (action.should_repaint()) || (self.comp.should_repaint()) {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl<T> OnResize for ListView<T> where T: ListItem {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base);
    }
}
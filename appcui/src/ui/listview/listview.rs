use super::{Flags, Item, ListItem};
use components::{Column, ColumnsHeader, ColumnsHeaderAction, ListScrollBars};
use AppCUIProcMacro::*;

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
    top_view: usize,
    pos: usize,
}

impl<T> ListView<T>
where
    T: ListItem,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self::with_capacity(16, layout, flags)
    }
    pub fn with_capacity(capacity: usize, layout: Layout, flags: Flags) -> Self {
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
            top_view: 0,
            pos: 0,
            data: Vec::with_capacity(capacity),
            filter: Vec::with_capacity(capacity),
            header: ColumnsHeader::with_capacity(4),
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.header.add(column);
    }
    pub fn add(&mut self, item: T) {
        self.add_item(Item::from(item));
    }
    pub fn add_item(&mut self, item: Item<T>) {
        let index = self.data.len() as u32;
        self.data.push(item);
        self.filter.push(Filter::Item(index));
        // refiltering is required
    }
    pub fn add_items(&mut self, items: Vec<T>) {
        let mut index = self.data.len() as u32;
        self.data.reserve(items.len());
        self.filter.reserve(items.len());
        for item in items {
            self.data.push(Item::from(item));
            self.filter.push(Filter::Item(index));
            index += 1;
        }
        // refiltering is required
    }
    pub fn set_frozen_columns(&mut self, count: u16) {
        self.header.set_frozen_columns(count);
        self.update_scrollbars();
    }
    fn sort_elements(&mut self, _column_index: u16, _ascendent: bool) {
        // sort elements by column index
    }
    fn autoresize_column(&mut self, _column_index: u16) {
        // auto resize column
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.header.scroll_to(self.comp.horizontal_index() as u32);
        self.top_view = (self.comp.vertical_index() as usize).min(self.filter.len());
        //self.update_left_position_for_items();
    }
    fn update_scrollbars(&mut self) {
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base, 1);
        self.comp.set_indexes(self.header.scroll_pos() as u64, self.top_view as u64);
    }
    fn execute_column_header_action(&mut self, action: ColumnsHeaderAction) -> bool {
        match action {
            ColumnsHeaderAction::Sort((index, ascendent)) => {
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
            ColumnsHeaderAction::Processed => true,
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
        }
    }
    fn paint_item(&self, item: &Item<T>, y: i32, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let width = self.header.width() as i32;
        let frozen_columns = self.header.frozen_columns();
        let columns = self.header.columns();
        if columns.len() == 0 {
            return;
        }
        let min_left = if frozen_columns == 0 {
            columns[0].x
        } else {
            let c = &columns[frozen_columns as usize - 1];
            c.x + c.width as i32 + 1
        };
        // first column
        let c = &columns[0];
        let l = c.x + item.x_offset();
        let r = c.x + c.width as i32;
        let mut extra = 0;
        if (r >= 0) && (r >= min_left) && (l < width) && (c.width != 0) {
            surface.set_relative_clip(l.max(min_left), y, r.max(min_left), y);
            surface.set_origin(l, y);
            if self.flags.contains(Flags::CheckBoxes) {
                if item.is_checked() {
                    surface.write_char(
                        l,
                        0,
                        Character::with_attributes(SpecialChar::CheckMark, attr.unwrap_or(theme.symbol.checked)),
                    );
                } else {
                    surface.write_char(l, 0, Character::with_attributes('x', attr.unwrap_or(theme.symbol.unchecked)));
                }
                extra = 2;
            }
            if extra > 0 {
                surface.set_relative_clip((l + extra).max(min_left), y, r.max(min_left), y);
                surface.set_origin(l + extra, y);
            }            
            if let Some(render_method) = ListItem::render_method(item.value(), 0) {
                if !render_method.paint(surface, theme, c.alignment, c.width as u16, attr) {
                    // custom paint required
                    ListItem::paint(item.value(), 0, c.width as u16, surface, theme)
                }
            }
        }
        for (index, c) in columns.iter().skip(1).enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (r < min_left) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            surface.set_relative_clip(c.x.max(min_left), y, r.max(min_left), y);
            surface.set_origin(c.x, y);
            if let Some(render_method) = ListItem::render_method(item.value(), index as u32) {
                if !render_method.paint(surface, theme, c.alignment, c.width as u16, attr) {
                    // custom paint required
                    ListItem::paint(item.value(), index as u32, c.width as u16, surface, theme)
                }
            }
        }
    }
    fn paint_items(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.base.has_focus();
        let attr = if !self.is_enabled() {
            Some(theme.text.inactive)
        } else if !has_focus {
            Some(theme.text.normal)
        } else {
            None
        };
        let mut y = 1;
        let max_y = self.size().height as i32;
        let mut idx = self.top_view;
        let max_idx = self.filter.len();
        // very simply code
        while (y < max_y) && (idx < max_idx) {
            match self.filter[idx] {
                Filter::Group(_) => {
                    // paint group
                }
                Filter::Item(index) => {
                    let item = &self.data[index as usize];
                    self.paint_item(item, y, surface, theme, attr);
                }
            }
            if (has_focus) && (idx == self.pos) {
                surface.reset_clip();
                surface.reset_origin();
                surface.fill_horizontal_line_with_size(0, y, self.size().width, Character::with_attributes(0, theme.list_current_item.focus));
            }
            y += 1;
            idx += 1;
        }
        surface.reset_clip();
        surface.reset_origin();
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.filter.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = (self.size().height.saturating_sub(1)) as usize;
        if h == 0 {
            return;
        }

        // check the top view
        if self.top_view + h >= len {
            self.top_view = len.saturating_sub(h);
        }
        if new_pos < self.top_view {
            self.top_view = new_pos;
        } else {
            let diff = new_pos - self.top_view;
            if (diff >= h) && (h > 0) {
                self.top_view = new_pos - h + 1;
            }
        }
        // update scrollbars
        self.update_scrollbars();
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            // self.raise_event(ControlEvent {
            //     emitter: self.handle,
            //     receiver: self.event_processor,
            //     data: ControlEventData::ListBox(EventData {
            //         event_type: ListBoxEventTypes::CurrentItemChanged,
            //         index: new_pos,
            //         checked: false, // not relevant for this event
            //     }),
            // });
        }
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.filter.len().saturating_sub(self.size().height.saturating_sub(1) as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
}

impl<T> OnPaint for ListView<T>
where
    T: ListItem,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // paint columns
        self.header.paint(surface, theme, &self.base);
        // paint items
        self.paint_items(surface, theme);
        // paint separation lines (columns)
        self.header.paint_columns(surface, theme, &self.base);
        // paint scroll bars and searh bars
        self.comp.paint(surface, theme, &self.base);
    }
}

impl<T> OnKeyPressed for ListView<T>
where
    T: ListItem,
{
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
                self.update_scrollbars();
                return EventProcessStatus::Processed;
            }
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.update_position(0, true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_position(self.filter.len(), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.size().height.saturating_sub(1) as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.size().height.saturating_sub(1) as usize), true);
                self.comp.exit_edit_mode();
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

impl<T> OnMouseEvent for ListView<T>
where
    T: ListItem,
{
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
impl<T> OnResize for ListView<T>
where
    T: ListItem,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.header.resize(new_size);
        self.comp.resize(self.header.width() as u64, self.filter.len() as u64, &self.base, 1);
    }
}

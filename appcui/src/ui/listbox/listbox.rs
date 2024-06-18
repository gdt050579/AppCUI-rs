use super::Flags;
use super::Item;
use crate::ui::components::ListScrollBars;
use AppCUIProcMacro::*;

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal = true)]
pub struct ListBox {
    items: Vec<Item>,
    flags: Flags,
    top_view: usize,
    left_view: usize,
    pos: usize,
    max_chars: u32,
    comp: ListScrollBars,
}
impl ListBox {
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
            items: Vec::new(),
            top_view: 0,
            left_view: 0,
            max_chars: 0,
            pos: usize::MAX,
            flags,
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
        }
    }

    /// Adds a new item to the list by providing a string value
    pub fn add(&mut self, value: &str) {
        self.items.push(Item::new(value));
        if self.items.len() == 1 {
            self.max_chars = self.items[0].count;
            // when first item is added, we should select it
            self.update_position(0usize, false);
            self.top_view = 0; // force the view to start from the first item
        } else {
            self.max_chars = self.max_chars.max(self.items.last().unwrap().count);
            self.update_scrollbars();
        }
    }

    /// Cleras all items from the list
    #[inline(always)]
    pub fn clear(&mut self) {
        self.items.clear();
        self.top_view = 0;
        self.pos = usize::MAX;
        self.max_chars = 0;
    }

    fn update_scrollbars(&mut self) {
        self.comp.set_indexes(self.left_view as u64, self.top_view as u64);
    }
    fn update_left_position_for_items(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let last_index = (len - 1).min(self.top_view + self.size().height as usize);
        for i in self.items[self.top_view..=last_index].iter_mut() {
            i.update_left_pos(self.left_view as u32);
        }
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        if new_pos < self.top_view {
            self.top_view = new_pos;
        } else {
            let diff = new_pos - self.top_view;
            let h = self.size().height as usize;
            if diff >= h {
                self.top_view = new_pos - h + 1;
            }
        }
        // update scrollbars
        self.update_scrollbars();
        self.update_left_position_for_items();
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            // self.on_event(Event::Command(Command::new(
            //     self.ID(),
            //     self.pos as u32,
            //     EventType::Change,
            // )));
        }
    }

    fn mouse_to_pos(&self, x: i32, y: i32) -> Option<usize> {
        let size = self.size();
        if x < 0 || y < 0 || x >= size.width as i32 || y >= size.height as i32 {
            return None;
        }
        let idx = self.top_view + y as usize;
        if idx < self.items.len() {
            return Some(idx);
        }
        None
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.top_view = (self.comp.vertical_index() as usize).min(self.items.len().saturating_sub(1));
        self.left_view = (self.comp.horizontal_index() as usize).min(self.max_chars as usize);
        self.update_left_position_for_items();
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.items.len().saturating_sub(self.size().height as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
    fn find_first_item(&mut self, pos: usize) {
        let mut i = if pos >= self.items.len() { 0 } else { pos };
        let mut count = self.items.len();
        while count > 0 {
            if self.items[i].filtered {
                self.update_position(i, true);
                return;
            }
            i = (i + 1) % self.items.len();
            count -= 1;
        }
    }
    fn search(&mut self) {
        let text_to_search = self.comp.search_text();
        if text_to_search.is_empty() {
            for item in self.items.iter_mut() {
                item.filtered = true;
            }
            self.comp.clear_match_count();
        } else {
            let mut count = 0usize;
            for item in self.items.iter_mut() {
                item.filtered = item.text().contains(text_to_search);
                if item.filtered {
                    count += 1;
                }
            }
            self.comp.set_match_count(count);
            if count > 0 {
                self.find_first_item(self.pos);
            }
        }
    }
}
impl OnPaint for ListBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        if has_focus && (self.flags.contains_one(Flags::ScrollBars | Flags::SearchBar)) {
            self.comp.paint(surface, theme, self);
            if self.flags.contains(Flags::ScrollBars) {
                surface.reduce_clip_by(0, 0, 1, 1);
            } else {
                surface.reduce_clip_by(0, 0, 0, 1);
            }
        }
        let attr = match () {
            _ if !self.is_active() => theme.text.inactive,
            _ if has_focus => theme.text.focused,
            _ => theme.text.normal,
        };

        let mut y = 0;
        let mut idx = self.top_view;
        let count = self.items.len();
        let h = self.size().height as i32;
        let w = self.size().width as i32;
        if self.flags.contains(Flags::CheckBoxes) {
            let ch_checked = Character::with_attributes(
                SpecialChar::CheckMark,
                match () {
                    _ if !self.is_active() => theme.text.inactive,
                    _ if has_focus => theme.symbol.checked,
                    _ => theme.text.normal,
                },
            );
            let ch_unchecked = Character::with_attributes(
                'x',
                match () {
                    _ if !self.is_active() => theme.text.inactive,
                    _ if has_focus => theme.symbol.unchecked,
                    _ => theme.text.normal,
                },
            );
            while (y < h) && (idx < count) {
                let item = &self.items[idx];
                if item.checked {
                    surface.write_char(0, y, ch_checked);
                } else {
                    surface.write_char(0, y, ch_unchecked);
                }
                surface.write_string(2, y, item.text(), if item.filtered { attr } else { theme.text.inactive }, false);
                if has_focus && (idx == self.pos) {
                    surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.list_current_item.focus));
                }
                y += 1;
                idx += 1;
            }
        } else {
            while (y < h) && (idx < count) {
                surface.write_string(
                    0,
                    y,
                    self.items[idx].text(),
                    if self.items[idx].filtered { attr } else { theme.text.inactive },
                    false,
                );
                if has_focus && (idx == self.pos) {
                    surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.list_current_item.focus));
                }
                y += 1;
                idx += 1;
            }
        }
    }
}

impl OnKeyPressed for ListBox {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.comp.process_key_pressed(key, character) {
            self.search();
            return EventProcessStatus::Processed;
        }
        match key.value() {
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
            key!("Left") => {
                self.left_view = self.left_view.saturating_sub(1);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Left") => {
                self.left_view = 0;
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = (self.left_view + 1).min(self.max_chars.saturating_sub(w) as usize);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = self.max_chars.saturating_sub(w) as usize;
                self.update_left_position_for_items();
                self.update_scrollbars();
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
                self.update_position(self.items.len(), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Space") => {
                if self.flags.contains(Flags::CheckBoxes) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                    }
                    return EventProcessStatus::Processed;
                }
            }
            key!("Enter") => {
                if self.comp.is_in_edit_mode() {
                    self.find_first_item(self.pos+1);
                    return EventProcessStatus::Processed;
                } else if self.flags.contains(Flags::CheckBoxes) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                    }
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl OnMouseEvent for ListBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let response = match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(d) | MouseEvent::DoubleClick(d) => {
                if let Some(pos) = self.mouse_to_pos(d.x, d.y) {
                    self.update_position(pos, true);
                }
                if (d.x == 0) && (self.flags.contains(Flags::CheckBoxes)) {
                    if let Some(item) = self.items.get_mut(self.pos) {
                        item.checked = !item.checked;
                    }
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(evn) => {
                match evn {
                    MouseWheelDirection::Up => self.move_scroll_to(self.top_view.saturating_sub(1)),
                    MouseWheelDirection::Down => self.move_scroll_to(self.top_view.saturating_add(1)),
                    _ => {}
                }
                EventProcessStatus::Processed
            }
        };
        // if one of the components require a repaint, than we should repaint even if the canvas required us to ignore the event
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}
impl OnResize for ListBox {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let extra = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
        self.comp.resize(self.max_chars as u64 + extra, self.items.len() as u64, &self.base);
        self.update_position(self.pos, false);
    }
}

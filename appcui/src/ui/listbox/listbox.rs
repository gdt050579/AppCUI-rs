use AppCUIProcMacro::*;

use super::Flags;
use super::Item;

#[CustomControl(overwrite = OnPaint+OnKeyPressed, internal = true)]
pub struct ListBox {
    items: Vec<Item>,
    flags: Flags,
    start_view: usize,
    pos: usize,
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
            start_view: 0,
            pos: usize::MAX,
            flags,
        }
    }

    /// Adds a new item to the list by providing a string value
    pub fn add(&mut self, value: &str) {
        self.items.push(Item::new(value));
        if self.items.len() == 1 {
            // when first item is added, we should select it
            self.update_position(0usize, false);
        }
    }

    /// Cleras all items from the list
    #[inline(always)]
    pub fn clear(&mut self) {
        self.items.clear();
        self.start_view = 0;
        self.pos = usize::MAX;
    }

    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        if new_pos < self.start_view {
            self.start_view = new_pos;
        } else {
            let diff = new_pos - self.start_view;
            let h = self.size().height as usize;
            if diff >= h {
                self.start_view = new_pos - h + 1;
            }
        }
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
}
impl OnPaint for ListBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        let attr = match () {
            _ if !self.is_active() => theme.text.inactive,
            _ if has_focus => theme.text.focused,
            _ => theme.text.normal,
        };
        let mut y = 0;
        let mut idx = self.start_view;
        let count = self.items.len();
        let h = self.size().height as i32;
        let w = self.size().width as i32;
        while (y < h) && (idx < count) {
            surface.write_string(0, y, &self.items[idx].value, attr, false);
            if has_focus && (idx == self.pos) {
                surface.fill_horizontal_line(0, y, w - 1, Character::with_attributes(0, theme.list_current_item.focus));
            }
            y += 1;
            idx += 1;
        }
    }
}

impl OnKeyPressed for ListBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.update_position(0, true);
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_position(self.items.len(), true);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                return EventProcessStatus::Processed;
            }

            _ => {}
        }
        EventProcessStatus::Ignored
    }
}

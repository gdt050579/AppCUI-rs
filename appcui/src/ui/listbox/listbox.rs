use super::Flags;
use super::Item;
use crate::ui::components::ComponentsToolbar;
use crate::ui::components::ScrollBar;
use AppCUIProcMacro::*;

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal = true)]
pub struct ListBox {
    items: Vec<Item>,
    flags: Flags,
    top_view: usize,
    left_view: usize,
    pos: usize,
    max_chars: u32,
    components: ComponentsToolbar,
    horizontal_scrollbar: Handle<ScrollBar>,
    vertical_scrollbar: Handle<ScrollBar>,
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
        let mut lbox = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            items: Vec::new(),
            top_view: 0,
            left_view: 0,
            max_chars: 0,
            pos: usize::MAX,
            flags,
            components: ComponentsToolbar::with_capacity(if flags.contains_one(Flags::ScrollBars | Flags::SearchBar) {
                3
            } else {
                0
            }),
            horizontal_scrollbar: Handle::None,
            vertical_scrollbar: Handle::None,
        };
        if flags.contains(Flags::ScrollBars) {
            lbox.horizontal_scrollbar = lbox.components.add(ScrollBar::new(0, false));
            lbox.vertical_scrollbar = lbox.components.add(ScrollBar::new(0, true));
        }
        lbox
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
        if let Some(s) = self.components.get_mut(self.horizontal_scrollbar) {
            s.set_index(self.left_view as u64);
        }
        if let Some(s) = self.components.get_mut(self.vertical_scrollbar) {
            s.set_index(self.top_view as u64);
        }
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
        if let (Some(horiz), Some(vert)) = (
            self.components.get(self.horizontal_scrollbar),
            self.components.get(self.vertical_scrollbar),
        ) {
            self.top_view = (vert.get_index() as usize).min(self.items.len().saturating_sub(1));
            self.left_view = (horiz.get_index() as usize).min(self.max_chars as usize);
            self.update_left_position_for_items();
        }
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.items.len().saturating_sub(self.size().height as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
}
impl OnPaint for ListBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        if has_focus && (self.flags.contains_one(Flags::ScrollBars | Flags::SearchBar)) {
            self.components.paint(surface, theme, self);
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
        while (y < h) && (idx < count) {
            surface.write_string(0, y, self.items[idx].text(), attr, false);
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
            key!("Left") => {
                self.left_view = self.left_view.saturating_sub(1);
                self.update_left_position_for_items();
                self.update_scrollbars();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                self.left_view = (self.left_view + 1).min(self.max_chars.saturating_sub(self.size().width) as usize);
                self.update_left_position_for_items();
                self.update_scrollbars();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
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
impl OnMouseEvent for ListBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let res = self.components.on_mouse_event(event);
        if res.should_update() {
            self.update_scroll_pos_from_scrollbars();
        }
        if !res.should_pass_to_control() {
            if res.should_repaint() {
                return EventProcessStatus::Processed;
            } else {
                return EventProcessStatus::Ignored;
            }
        }
        let response = match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(d) | MouseEvent::DoubleClick(d) => {
                if let Some(pos) = self.mouse_to_pos(d.x, d.y) {
                    self.update_position(pos, true);
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
        if res.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}
impl OnResize for ListBox {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.components.on_resize(&self.base);
        if let Some(s) = self.components.get_mut(self.horizontal_scrollbar) {
            s.update_count(new_size.width as u64, self.max_chars as u64);
        }
        if let Some(s) = self.components.get_mut(self.vertical_scrollbar) {
            s.update_count(new_size.height as u64, self.items.len() as u64);
        }
        self.update_position(self.pos, false);
    }
}

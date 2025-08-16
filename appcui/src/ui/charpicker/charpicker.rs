use super::set::Set;
use crate::prelude::*;
use crate::ui::charpicker::events::EventData;

#[derive(Copy, Clone, Eq, PartialEq)]
enum MousePos {
    None,
    Char(u32),
    HoverLeftButton,
    PressLeftButton,
    HoverRightButton,
    PressRightButton,
    HoverNone,
    PressNone,
}

struct Navigation {
    chars_per_width: i32,
    height: i32,
    set_index: u32,
    start_view_index: u32,
    current_index: u32,
    computed_column_index: u32,
    mouse_pos: MousePos,
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct CharPicker {
    character: Option<char>,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    nav: Navigation,
    sets: Vec<Set>,
}
impl CharPicker {
    pub fn new(initial_char: Option<char>, layout: Layout) -> Self {
        Self::inner_new(initial_char, layout, vec![Set::with_interval("Unicode", 1, 0xE01EF).unwrap()])
    }
    pub fn with_set(initial_char: Option<char>, layout: Layout, set: Set) -> Self {
        Self::inner_new(initial_char, layout, vec![set])
    }
    fn inner_new(initial_char: Option<char>, layout: Layout, sets: Vec<Set>) -> Self {
        let mut cp = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            nav: Navigation {
                chars_per_width: 1,
                set_index: 0,
                start_view_index: 0,
                current_index: 0,
                height: 1,
                computed_column_index: 0,
                mouse_pos: MousePos::None,
            },
            character: None,
            sets,
        };
        if let Some(ch) = initial_char {
            cp.goto(ch, false, false);
        }
        cp.set_size_bounds(11, 1, u16::MAX, 1);
        cp
    }
    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }
    pub fn clear_sets(&mut self) {
        self.sets.clear();
    }
    pub fn select_char(&mut self, character: char) {
        self.goto(character, false, true);
    }
    pub fn unselect_char(&mut self) {
        self.character = None;
    }
    fn emit_change_char_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::CharPicker(EventData {
                code: self.character.unwrap_or(0 as char),
            }),
        });
    }

    fn update_view(&mut self, update_char: bool, emit_event: bool) {
        if !self.sets.is_empty() {
            if self.nav.set_index as usize >= self.sets.len() {
                // reset to first
                self.nav.set_index = 0;
                self.nav.current_index = 0;
            }
            let count = self.sets[self.nav.set_index as usize].count();
            // I know a set has at least one character
            // make sure tha current index is valid
            self.nav.current_index = self.nav.current_index.min(count.saturating_sub(1));
            if self.nav.current_index < self.nav.start_view_index {
                self.nav.start_view_index = self.nav.current_index.saturating_sub(self.nav.computed_column_index);
            } else {
                let displayed_chars = (self.nav.chars_per_width * self.nav.height) as u32;
                if self.nav.current_index >= self.nav.start_view_index + displayed_chars {
                    self.nav.start_view_index = self.nav.current_index.saturating_sub(displayed_chars - 1)
                        + (self.nav.chars_per_width as u32).saturating_sub(self.nav.computed_column_index + 1);
                    self.nav.start_view_index = self.nav.start_view_index.min(count.saturating_sub(displayed_chars));
                }
            }
            self.nav.computed_column_index = (self.nav.current_index.saturating_sub(self.nav.start_view_index)) % (self.nav.chars_per_width as u32);
        }
        if update_char {
            let new_char = if self.sets.is_empty() {
                None
            } else {
                self.sets[self.nav.set_index as usize].char(self.nav.current_index)
            };
            let old_char = self.character;
            self.character = new_char;
            if (emit_event) && (old_char != self.character) {
                self.emit_change_char_event();
            }
        }
    }

    fn move_scroll_view_by(&mut self, dir: i32) {
        if self.sets.is_empty() {
            return;
        }
        let ofs = (self.nav.chars_per_width * dir.abs()) as u32;
        self.nav.start_view_index = if dir < 0 {
            self.nav.start_view_index.saturating_sub(ofs)
        } else {
            self.nav.start_view_index + ofs
        };
        let page_size = self.nav.chars_per_width as u32 * (self.expanded_size().height.saturating_sub(5));
        let count = self.sets[self.nav.set_index as usize].count();
        self.nav.start_view_index = self.nav.start_view_index.min(count.saturating_sub(page_size));
    }

    fn goto(&mut self, ch: char, emit_event: bool, update_view: bool) {
        let mut result = None;
        for (set_idx, s) in self.sets.iter().enumerate() {
            if let Some(idx) = s.index_of(ch) {
                result = Some((set_idx as u32, idx));
                break;
            }
        }
        if let Some((set_idx, idx)) = result {
            self.nav.set_index = set_idx;
            self.nav.current_index = idx;
            let old_char = self.character;
            self.character = Some(ch);

            if (emit_event) && (self.character != old_char) {
                self.emit_change_char_event();
            }
            if update_view {
                self.update_view(true, false);
            }
        }
    }
    fn paint_extanded(&self, surface: &mut Surface, theme: &Theme) {
        // if you end up at this point , it is assume thet self.sets are not empty !!!
        let size = self.expanded_size();
        let space_char = Character::with_attributes(' ', theme.menu.text.normal);
        let col = theme.menu.text.normal;
        let set = &self.sets[self.nav.set_index as usize];
        surface.fill_rect(
            Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            space_char,
        );
        surface.draw_rect(
            Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            LineType::Single,
            col,
        );
        let none_attr = match self.nav.mouse_pos {
            MousePos::HoverNone => theme.menu.text.hovered,
            MousePos::PressNone => theme.menu.text.pressed_or_selectd,
            _ => theme.menu.text.normal,
        };
        surface.write_string(2, size.height as i32 - 2 + self.expanded_panel_y, "[None]", none_attr, false);
        // draw Set Name
        let format = TextFormatBuilder::new()
            .align(TextAlignment::Center)
            .position(size.width as i32 / 2, self.expanded_panel_y + 1)
            .attribute(theme.menu.hotkey.normal)
            .chars_count(set.name_chars_count() as u16)
            .build();
        surface.set_relative_clip(4, self.expanded_panel_y + 1, (size.width as i32) - 5, self.expanded_panel_y + 1);
        surface.write_text(set.name(), &format);
        surface.reset_clip();
        surface.draw_horizontal_line(1, self.expanded_panel_y + 2, size.width as i32 - 2, LineType::Single, col);
        // left button
        let but_color = if self.nav.set_index == 0 {
            theme.menu.text.inactive
        } else {
            match self.nav.mouse_pos {
                MousePos::HoverLeftButton => theme.menu.text.hovered,
                MousePos::PressLeftButton => theme.menu.text.pressed_or_selectd,
                _ => theme.menu.text.normal,
            }
        };
        surface.write_string(1, self.expanded_panel_y + 1, " \u{25C4} ", but_color, false);
        // right button
        let but_color = if (self.nav.set_index + 1) as usize == self.sets.len() {
            theme.menu.text.inactive
        } else {
            match self.nav.mouse_pos {
                MousePos::HoverRightButton => theme.menu.text.hovered,
                MousePos::PressRightButton => theme.menu.text.pressed_or_selectd,
                _ => theme.menu.text.normal,
            }
        };
        surface.write_string(size.width as i32 - 4, self.expanded_panel_y + 1, " \u{25BA} ", but_color, false);
        let mut y = 0;
        let mut x = 0;
        let mut idx = self.nav.start_view_index;
        let count = set.count();
        let ofs_y = 3 + self.expanded_panel_y;
        let mouse_idx = match self.nav.mouse_pos {
            MousePos::Char(idx) => idx,
            _ => u32::MAX,
        };
        while (idx < count) && (y < self.nav.height) {
            let ch = set.char(idx).unwrap_or('?');
            surface.write_char(x * 3 + 2, y + ofs_y, Character::with_attributes(ch, col));
            if idx == self.nav.current_index {
                surface.fill_horizontal_line_with_size(
                    x * 3 + 1,
                    y + ofs_y,
                    3,
                    Character::with_attributes(0 as char, theme.menu.text.pressed_or_selectd),
                );
            } else if idx == mouse_idx {
                surface.fill_horizontal_line_with_size(x * 3 + 1, y + ofs_y, 3, Character::with_attributes(0 as char, theme.menu.text.hovered));
            }
            x += 1;
            if x >= self.nav.chars_per_width {
                x = 0;
                y += 1;
            }
            idx += 1;
        }
    }
    fn compute_mouse_pos(&self, x: i32, y: i32) -> MousePos {
        if !self.is_expanded() || self.sets.is_empty() {
            MousePos::None
        } else {
            let size = self.expanded_size();
            let w = size.width as i32;
            // check buttons
            if y == (self.expanded_panel_y + 1) {
                if (x >= 1) && (x <= 3) {
                    if self.nav.set_index == 0 {
                        MousePos::None
                    } else {
                        MousePos::HoverLeftButton
                    }
                } else if (x >= w - 4) && (x <= w - 1) {
                    if (self.nav.set_index + 1) as usize == self.sets.len() {
                        MousePos::None
                    } else {
                        MousePos::HoverRightButton
                    }
                } else {
                    MousePos::None
                }
            } else if (y == size.height as i32 - 2 + self.expanded_panel_y) && (x >= 2) && (x <= 7) {
                MousePos::HoverNone
            } else {
                // check character
                let ofs_y = 3 + self.expanded_panel_y;
                if (y >= ofs_y) && (y < size.height as i32 - 2 + self.expanded_panel_y) && (x > 0) && (x < w - 1) {
                    let px = (x - 1) / 3;
                    if px >= self.nav.chars_per_width {
                        MousePos::None
                    } else {
                        let idx = (px + (y - ofs_y) * self.nav.chars_per_width) as u32 + self.nav.start_view_index;
                        if idx >= self.sets[self.nav.set_index as usize].count() {
                            MousePos::None
                        } else {
                            MousePos::Char(idx)
                        }
                    }
                } else {
                    MousePos::None
                }
            }
        }
    }
    fn goto_set(&mut self, set_index: u32) {
        if self.sets.is_empty() {
            return;
        }
        let new_index = set_index.min((self.sets.len() - 1) as u32);
        if new_index == self.nav.set_index {
            return;
        }
        self.nav.set_index = new_index;
        self.nav.current_index = 0;
        self.nav.start_view_index = 0;
        self.update_view(true, true);
    }
}
impl OnPaint for CharPicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let size = self.size();
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);
        // normal bar
        surface.fill_horizontal_line_with_size(0, self.header_y_ofs, size.width.saturating_sub(4), space_char);
        if let Some(character) = self.character {
            surface.write_char(1, self.header_y_ofs, Character::with_attributes(character, col_text));
            let mut arr: [u8; 9] = [b'(', b'U', b'+', b'0', b'0', b'0', b'0', b'0', b')'];
            let mut code = character as u32;
            let mut pos = 7;
            while (code > 0) && (pos > 2) {
                let r = (code % 16) as u8;
                if r < 10 {
                    arr[pos] = 48 + r;
                } else {
                    arr[pos] = 55 + r;
                }
                pos = pos - 1;
                code = code >> 4;
            }
            // paint code
            if size.width > 12 {
                if size.width < 17 {
                    surface.write_ascii(3, self.header_y_ofs, &arr[3..8], col_text, false);
                } else {
                    surface.write_ascii(3, self.header_y_ofs, arr.as_slice(), col_text, false);
                }
            }
        } else {
            surface.write_ascii(1, self.header_y_ofs, "None".as_bytes(), col_text, false);
        }
        // drop button
        let px = (size.width - 3) as i32;
        surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
        surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        // assuming the control is expanded
        if self.is_expanded() && !self.sets.is_empty() {
            self.paint_extanded(surface, theme);
        }
    }
}
impl OnDefaultAction for CharPicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            self.expand(Size::new(11, 7), Size::new(self.size().width, 9));
        }
    }
}
impl OnExpand for CharPicker {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (self.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
        self.nav.chars_per_width = ((self.expanded_size().width.saturating_sub(2) / 3) as i32).max(1);
        self.nav.height = self.expanded_size().height.saturating_sub(5) as i32;
        self.nav.mouse_pos = MousePos::None;
        self.update_view(false, false);
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
        self.nav.chars_per_width = 1; // Up/Down will go one char
        self.nav.height = 1;
        self.nav.mouse_pos = MousePos::None;
        self.update_view(false, false);
    }
}
impl OnKeyPressed for CharPicker {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let expanded = self.is_expanded();

        match key.value() {
            key!("Escape") => {
                if expanded {
                    self.pack();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                return EventProcessStatus::Processed;
            }
            key!("Up") => {
                self.nav.current_index = self.nav.current_index.saturating_sub(self.nav.chars_per_width as u32);
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.nav.current_index += self.nav.chars_per_width as u32;
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.nav.current_index = self.nav.current_index.saturating_sub(1);
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                self.nav.current_index += 1;
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.nav.current_index = 0;
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.nav.current_index = u32::MAX;
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                let dif = (self.nav.chars_per_width * (self.expanded_size().height.saturating_sub(5) as i32)) as u32;
                self.nav.current_index = self.nav.current_index.saturating_sub(dif);
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                let dif = (self.nav.chars_per_width * (self.expanded_size().height.saturating_sub(5) as i32)) as u32;
                self.nav.current_index = self.nav.current_index + dif;
                self.update_view(true, true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") | key!("Alt+Left") => {
                self.goto_set(self.nav.set_index.saturating_sub(1));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Alt+Right") => {
                self.goto_set(self.nav.set_index + 1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Up") | key!("Alt+Up") => {
                self.move_scroll_view_by(-1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") | key!("Alt+Down") => {
                self.move_scroll_view_by(1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                if let Some(txt) = RuntimeManager::get().backend().clipboard_text() {
                    if txt.len() > 0 {
                        let ch = txt.chars().next();
                        if let Some(ch) = ch {
                            self.goto(ch, true, true);
                        }
                    }
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                if let Some(ch) = self.character {
                    let mut buf: [u8;16] = [0;16];                    
                    RuntimeManager::get().backend_mut().set_clipboard_text(ch.encode_utf8(&mut buf));
                }
                return EventProcessStatus::Processed;
            }            
            _ => {}
        }
        if character >= 31 as char {
            self.goto(character, true, true);
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for CharPicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Processed,

            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(p) => {
                let mpos = self.compute_mouse_pos(p.x, p.y);
                if mpos != self.nav.mouse_pos {
                    self.nav.mouse_pos = mpos;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(data) => {
                if data.y == self.header_y_ofs {
                    self.on_default_action();
                } else {
                    let mpos = self.compute_mouse_pos(data.x, data.y);
                    match mpos {
                        MousePos::HoverLeftButton => self.nav.mouse_pos = MousePos::PressLeftButton,
                        MousePos::HoverRightButton => self.nav.mouse_pos = MousePos::PressRightButton,
                        MousePos::HoverNone => self.nav.mouse_pos = MousePos::PressNone,
                        _ => self.nav.mouse_pos = mpos,
                    }
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                if self.is_expanded() {
                    let mpos = self.compute_mouse_pos(data.x, data.y);
                    match mpos {
                        MousePos::Char(index) => {
                            self.nav.current_index = index;
                            self.update_view(true, true);
                        }
                        MousePos::HoverLeftButton => self.goto_set(self.nav.set_index.saturating_sub(1)),
                        MousePos::HoverRightButton => self.goto_set(self.nav.set_index + 1),
                        MousePos::HoverNone => {
                            if self.character.is_some() {
                                self.character = None;
                                self.emit_change_char_event();
                            }
                        }
                        _ => (),
                    }
                    self.nav.mouse_pos = mpos;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Left => self.goto_set(self.nav.set_index.saturating_sub(1)),
                    MouseWheelDirection::Right => self.goto_set(self.nav.set_index + 1),
                    MouseWheelDirection::Up => self.move_scroll_view_by(-1),
                    MouseWheelDirection::Down => self.move_scroll_view_by(1),
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

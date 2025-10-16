use super::{
    events::{EventData, TextFieldEventsType},
    CharClass, Flags, Selection,
};
use crate::prelude::*;
use crate::utils::GlyphParser;

struct Cursor {
    pos: usize,
    start: usize,
    end: usize,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnFocus, internal=true)]
pub struct TextField {
    cursor: Cursor,
    selection: Selection,
    glyphs: String,
    drag_started: bool,
    flags: Flags,
}
impl TextField {
    /// Creates a new TextField control with the specified text, layout and flags.
    /// The flags can be a combination of the following values:
    /// * `Flags::Readonly` - if set, the text field will be readonly
    /// * `Flags::ProcessEnter` - if set, the text field will process the Enter key and raise an event
    /// * `Flags::DisableAutoSelectOnFocus` - if set, the text field will not select all text when it gets focus
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut textfield = TextField::new("Hello World",
    ///                                    layout!("x:1,y:1,w:20,h:1"),
    ///                                    textfield::Flags::None);
    /// ```
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            cursor: Cursor { pos: 0, start: 0, end: 0 },
            selection: Selection::NONE,
            glyphs: String::from(text),
            drag_started: false,
            flags,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.cursor.pos = obj.glyphs.len();
        obj
    }

    /// Returns **true** if the TextField control is readonly, **false** otherwise.
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(Flags::Readonly)
    }

    /// Returns the text of the TextField control.
    #[inline(always)]
    pub fn text(&self) -> &str {
        &self.glyphs
    }

    /// Sets the text of the TextField control.
    #[inline(always)]
    pub fn set_text(&mut self, text: &str) {
        self.cursor = Cursor { pos: 0, start: 0, end: 0 };
        self.selection = Selection::NONE;
        self.glyphs.clear();
        self.glyphs.push_str(text);
        self.move_cursor_to(self.glyphs.len(), false, true);
    }

    fn update_scroll_view(&mut self, force_end_update: bool) {
        if (self.cursor.pos >= self.cursor.start) && (self.cursor.pos < self.cursor.end) {
            // nothing to do --> curent pos is already in the view window
            if force_end_update {
                let sz = self.size();
                let visible_glyphs = ((sz.width as usize) - 2) * (sz.height as usize);
                self.cursor.end = self.glyphs.next_pos(self.cursor.start, visible_glyphs);
            }
            return;
        }
        let sz = self.size();
        let visible_glyphs = (if sz.width > 2 {
            ((sz.width as usize) - 2) * (sz.height as usize)
        } else {
            0
        })
        .max(1);

        if self.cursor.pos < self.cursor.start {
            // scroll to the left
            self.cursor.start = self.cursor.pos;
            self.cursor.end = self.glyphs.next_pos(self.cursor.pos, visible_glyphs);
        } else {
            // scroll to the right
            self.cursor.start = self.glyphs.previous_pos(self.cursor.pos, visible_glyphs - 1);
            // we add ONE to the end pot to satisfy (self.cursor.pos < self.cursor.end) condition
            self.cursor.end = self.cursor.pos;
        }
    }
    fn move_cursor_with(&mut self, no_of_glyphs: i32, select: bool) {
        let new_poz = if no_of_glyphs >= 0 {
            self.glyphs.next_pos(self.cursor.pos, no_of_glyphs as usize)
        } else {
            self.glyphs.previous_pos(self.cursor.pos, (-no_of_glyphs) as usize)
        };
        self.move_cursor_to(new_poz, select, false);
    }
    fn move_cursor_to(&mut self, new_offset: usize, select: bool, force_end_update: bool) {
        let current_pos = self.cursor.pos;
        self.cursor.pos = new_offset.min(self.glyphs.len());
        self.update_scroll_view(force_end_update);
        if select {
            self.selection.update(current_pos, self.cursor.pos);
        } else {
            self.selection = Selection::NONE;
        }
    }
    fn move_to_next_word(&mut self, select: bool) {
        if let Some(char_class) = self.glyphs.glyph(self.cursor.pos).map(|c| CharClass::from(c.0)) {
            let mut pos = self.cursor.pos;
            let mut new_char_class = char_class;
            // skip current class
            while let Some((c, size)) = self.glyphs.glyph(pos) {
                if CharClass::from(c) != char_class {
                    new_char_class = CharClass::from(c);
                    break;
                }
                pos += size as usize;
            }
            if (new_char_class != char_class) && (new_char_class == CharClass::Space) {
                // skip the spaces until we reach a new char class
                while let Some((c, size)) = self.glyphs.glyph(pos) {
                    if CharClass::from(c) != new_char_class {
                        break;
                    }
                    pos += size as usize;
                }
            }
            pos = pos.min(self.glyphs.len());
            self.move_cursor_to(pos, select, false);
        }
    }
    fn move_to_previous_word(&mut self, select: bool) {
        if let Some(char_class) = self.glyphs.previous_glyph(self.cursor.pos).map(|c| CharClass::from(c.0)) {
            let mut pos = self.cursor.pos;
            let mut new_char_class = char_class;
            // skip current class
            while let Some((c, size)) = self.glyphs.previous_glyph(pos) {
                if CharClass::from(c) != char_class {
                    new_char_class = CharClass::from(c);
                    break;
                }
                if size as usize <= pos {
                    pos -= size as usize;
                } else {
                    pos = 0;
                    break;
                }
            }
            if (new_char_class != char_class) && (char_class == CharClass::Space) {
                // skip the the current class until I rech the start of it
                while let Some((c, size)) = self.glyphs.previous_glyph(pos) {
                    if CharClass::from(c) != new_char_class {
                        break;
                    }
                    if size as usize <= pos {
                        pos -= size as usize;
                    } else {
                        pos = 0;
                        break;
                    }
                }
            }
            self.move_cursor_to(pos, select, false);
        }
    }
    fn copy_text(&mut self) {
        if !self.selection.is_empty() {
            RuntimeManager::get()
                .backend_mut()
                .set_clipboard_text(&self.glyphs[self.selection.start..self.selection.end]);
        }
    }
    // true if the text was changed, false otherwise
    fn paste_text(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        let mut text_was_modified = false;
        if !self.selection.is_empty() {
            text_was_modified = self.delete_selection();
        }
        if let Some(txt) = RuntimeManager::get().backend().clipboard_text() {
            self.glyphs.insert_str(self.cursor.pos, &txt);
            text_was_modified |= !txt.is_empty();
            self.move_cursor_to(self.cursor.pos + txt.len(), false, true);
        }
        text_was_modified
    }
    // true if the text was changed, false otherwise
    fn cut_text(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if !self.selection.is_empty() {
            RuntimeManager::get()
                .backend_mut()
                .set_clipboard_text(&self.glyphs[self.selection.start..self.selection.end]);
            self.delete_selection()
        } else {
            false
        }
    }
    // true if the text was changed, false otherwise
    fn convert_selection_or_word(&mut self, callback: fn(text: &str) -> String) -> bool {
        if self.is_readonly() {
            return false;
        }
        if self.selection.is_empty() {
            self.select_word(self.cursor.pos);
        }

        if !self.selection.is_empty() {
            let s = callback(&self.glyphs[self.selection.start..self.selection.end]);
            let text_changed = s != self.glyphs[self.selection.start..self.selection.end];
            self.glyphs.replace_range(self.selection.start..self.selection.end, &s);
            let start = self.selection.start;
            let count = s.count_glyphs();
            self.selection = Selection::NONE;
            self.cursor.pos = start;
            self.move_cursor_with(count as i32, true);
            text_changed
        } else {
            false
        }
    }

    fn select_all(&mut self) {
        self.selection = Selection::NONE;
        self.selection.update(0, self.glyphs.len());
        self.move_cursor_to(self.glyphs.len(), true, false);
    }
    // true if the text was changed, false otherwise
    fn delete_selection(&mut self) -> bool {
        if !self.selection.is_empty() {
            let new_pos = self.selection.start;
            self.glyphs.replace_range(self.selection.start..self.selection.end, "");
            self.selection = Selection::NONE;
            self.move_cursor_to(new_pos, false, true);
            true
        } else {
            false
        }
    }
    // true if the text was changed, false otherwise
    fn delete_current_character(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if self.selection.is_empty() {
            let next_pos = self.glyphs.next_pos(self.cursor.pos, 1);
            if self.cursor.pos < next_pos {
                self.glyphs.replace_range(self.cursor.pos..next_pos, "");
                self.update_scroll_view(true);
                return true;
            }
            false
        } else {
            self.delete_selection()
        }
    }
    // true if the text was changed, false otherwise
    fn delete_previous_character(&mut self) -> bool {
        if self.is_readonly() {
            return false;
        }
        if self.selection.is_empty() {
            let prev_pos = self.glyphs.previous_pos(self.cursor.pos, 1);
            if prev_pos < self.cursor.pos {
                let end_pos = self.cursor.pos;
                self.glyphs.replace_range(prev_pos..end_pos, "");
                self.move_cursor_to(prev_pos, false, true);
                return true;
            }
            false
        } else {
            self.delete_selection()
        }
    }
    fn add_char(&mut self, character: char) -> bool {
        if self.is_readonly() {
            return false;
        }
        if !self.selection.is_empty() {
            self.delete_selection();
        }
        self.glyphs.insert(self.cursor.pos, character);
        self.move_cursor_with(1, false);
        true
    }
    fn select_word(&mut self, offset: usize) {
        if let Some((start, end)) = self.glyphs.word_range(offset, |c| CharClass::from(c) == CharClass::Word) {
            self.selection = Selection::NONE;
            self.move_cursor_to(start, false, true);
            self.move_cursor_to(end, true, true);
        }
    }
    fn mouse_pos_to_glyph_offset(&self, x: i32, y: i32, within_control: bool) -> Option<usize> {
        let sz = self.size();
        let w = sz.width as i32;
        let h = sz.height as i32;
        if within_control && ((x < 1) || (x >= w - 1) || (y < 0) || (y >= h)) {
            return None;
        }
        let glyphs_count = (x - 1) + y * (w - 2);
        match glyphs_count.cmp(&0) {
            std::cmp::Ordering::Less => Some(self.glyphs.previous_pos(self.cursor.start, (-glyphs_count) as usize)),
            std::cmp::Ordering::Equal => Some(self.cursor.start),
            std::cmp::Ordering::Greater => Some(self.glyphs.next_pos(self.cursor.start, glyphs_count as usize)),
        }
    }

    fn notify_text_changed(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TextField(EventData {
                evtype: TextFieldEventsType::OnTextChanged,
            }),
        });
    }
}
impl OnResize for TextField {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        // we need to compute the end scroll based on the new size
        let visible_chars = if new_size.width > 2 {
            ((new_size.width - 2) as usize) * (new_size.height as usize)
        } else {
            0
        };
        self.cursor.end = self.glyphs.next_pos(self.cursor.start, visible_chars);
        // check if the current cursor is within the scroll view and if not update the scroll view
        self.update_scroll_view(false);
    }
}
impl OnPaint for TextField {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));
        // paint
        let show_cursor = self.has_focus();
        let sz = self.size();
        let w = (sz.width - 1) as i32;
        let mut count = (sz.width - 2) * sz.height;
        let mut pos = self.cursor.start;
        let mut x = 1;
        let mut y = 0;
        let mut ch = Character::with_attributes(' ', attr);
        let mut ch_selected = Character::with_attributes(' ', theme.editor.pressed_or_selected);
        while let Some((code, glyph_size)) = self.glyphs.glyph(pos) {
            if (show_cursor) && self.selection.contains(pos) {
                ch_selected.code = code;
                surface.write_char(x, y, ch_selected);
            } else {
                ch.code = code;
                surface.write_char(x, y, ch);
            }
            if show_cursor && (pos == self.cursor.pos) {
                surface.set_cursor(x, y);
            }
            x += 1;
            if x >= w {
                x = 1;
                y += 1;
            }
            pos += glyph_size as usize;
            count -= 1;
            if count == 0 {
                break;
            }
        }
        // if it is the last char
        if show_cursor && (pos == self.cursor.pos) {
            // if the cursor is located on the fist line outside the view --> put it on the last char but on previous line
            if (y == sz.height as i32) && (x == 1) {
                surface.set_cursor(sz.width as i32 - 1, sz.height as i32 - 1);
            } else {
                surface.set_cursor(x, y);
            }
        }
    }
}
impl OnKeyPressed for TextField {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                self.move_cursor_with(-1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.move_cursor_with(-((self.size().width as i32) - 2), key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.move_cursor_with((self.size().width as i32) - 2, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.move_cursor_to(0, key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.move_cursor_to(self.glyphs.len(), key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                self.move_to_previous_word(key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                self.move_to_next_word(key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            // clipboard
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                self.copy_text();
                return EventProcessStatus::Processed;
            }
            // start checking if the text was changed
            key!("Ctrl+X") | key!("Shift+Del") => {
                if self.cut_text() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                if self.paste_text() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+U") => {
                if self.convert_selection_or_word(|s| s.to_uppercase()) {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+U") => {
                if self.convert_selection_or_word(|s| s.to_lowercase()) {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+A") => {
                self.select_all();
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                if self.delete_current_character() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Back") => {
                if self.delete_previous_character() {
                    self.notify_text_changed();
                }
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                if self.flags.contains(Flags::ProcessEnter) {
                    self.raise_event(ControlEvent {
                        emitter: self.handle,
                        receiver: self.event_processor,
                        data: ControlEventData::TextField(EventData {
                            evtype: TextFieldEventsType::OnValidate,
                        }),
                    });
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }
        if (character as u32) > 0 {
            if self.add_char(character) {
                self.notify_text_changed();
            }
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnFocus for TextField {
    fn on_focus(&mut self) {
        if !self.flags.contains(Flags::DisableAutoSelectOnFocus) {
            self.select_all();
        }
    }
}
impl OnMouseEvent for TextField {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.drag_started = false;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                if let Some(new_pos) = self.mouse_pos_to_glyph_offset(data.x, data.y, true) {
                    self.move_cursor_to(new_pos, false, false);
                    self.drag_started = true;
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.drag_started = false;
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) => {
                if let Some(ofs) = self.mouse_pos_to_glyph_offset(data.x, data.y, true) {
                    self.select_word(ofs);
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.drag_started {
                    if let Some(new_pos) = self.mouse_pos_to_glyph_offset(data.x, data.y, false) {
                        self.move_cursor_to(new_pos, true, true);
                    }
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}

use super::events::{EventData, TextFieldEventsType};
use super::Flags;
use super::CharClass;
use crate::prelude::*;
use crate::utils::GlyphParser;

struct Cursor {
    pos: usize,
    start: usize,
    end: usize,
}
struct Selection {
    start: usize,
    end: usize,
    origin: usize,
}
impl Selection {
    const NONE: Selection = Selection {
        start: usize::MAX,
        end: usize::MAX,
        origin: usize::MAX,
    };
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.origin == usize::MAX
    }
    #[inline(always)]
    fn update(&mut self, start: usize, end: usize) {
        if self.is_empty() {
            self.origin = start;
            self.end = start.max(end);
            self.start = start.min(end);
        } else {
            self.start = self.origin.min(end);
            self.end = self.origin.max(end);
        }
    }
    #[inline(always)]
    fn contains(&self, pos: usize) -> bool {
        (pos >= self.start) && (pos < self.end)
    }
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct TextField {
    cursor: Cursor,
    selection: Selection,
    glyphs: String,
    flags: Flags,
}
impl TextField {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            cursor: Cursor { pos: 0, start: 0, end: 0 },
            selection: Selection::NONE,
            glyphs: String::from(text),
            flags,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.cursor.pos = obj.glyphs.len();
        obj
    }
    #[inline(always)]
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(Flags::Readonly)
    }

    #[inline(always)]
    pub fn text(&self) -> &str {
        &self.glyphs
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
        let visible_glyphs = ((sz.width as usize) - 2) * (sz.height as usize);

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
            while let Some((c,size)) = self.glyphs.glyph(pos) {
                if CharClass::from(c) != char_class {
                    new_char_class = CharClass::from(c);
                    break;
                }
                pos+=size as usize;
            }
            if (new_char_class != char_class) && (new_char_class == CharClass::Space) {
                // skip the spaces until we reach a new char class
                while let Some((c,size)) = self.glyphs.glyph(pos) {
                    if CharClass::from(c) != new_char_class {
                        break;
                    }
                    pos+=size as usize;
                }                
            }
            pos = pos.min(self.glyphs.len());
            self.move_cursor_to(pos, select, false);
        }
    }
    fn move_to_previous_word(&mut self, select: bool) {
        todo!()
    }
    fn copy_text(&mut self) {
        if !self.selection.is_empty() {
            todo!()
        }
    }
    fn paste_text(&mut self) {
        if self.is_readonly() {
            return;
        }
        if !self.selection.is_empty() {
            self.delete_selection();
        }
        todo!()
    }
    fn cut_text(&mut self) {
        if self.is_readonly() {
            return;
        }
        if !self.selection.is_empty() {
            // copy text
            todo!();
            self.delete_selection();
        }
    }
    fn convert_to_upper(&mut self) {
        if self.is_readonly() {
            return;
        }
        todo!()
    }
    fn convert_to_lower(&mut self) {
        if self.is_readonly() {
            return;
        }
        todo!()
    }
    fn select_all(&mut self) {
        self.selection = Selection::NONE;
        self.selection.update(0, self.glyphs.len());
        self.move_cursor_to(self.glyphs.len(), true, false);
    }
    fn delete_selection(&mut self) {
        if !self.selection.is_empty() {
            let new_pos = self.selection.start;
            self.glyphs.replace_range(self.selection.start..self.selection.end, "");
            self.selection = Selection::NONE;
            self.move_cursor_to(new_pos, false, true);
        }
    }
    fn delete_current_character(&mut self) {
        if self.is_readonly() {
            return;
        }
        if self.selection.is_empty() {
            let next_pos = self.glyphs.next_pos(self.cursor.pos, 1);
            if self.cursor.pos < next_pos {
                self.glyphs.replace_range(self.cursor.pos..next_pos, "");
                self.update_scroll_view(true);
            }
        } else {
            self.delete_selection();
        }
    }
    fn delete_previous_character(&mut self) {
        if self.is_readonly() {
            return;
        }
        if self.selection.is_empty() {
            let prev_pos = self.glyphs.previous_pos(self.cursor.pos, 1);
            if prev_pos < self.cursor.pos {
                let end_pos = self.cursor.pos;
                self.glyphs.replace_range(prev_pos..end_pos, "");
                self.move_cursor_to(prev_pos, false, true);
            }
        } else {
            self.delete_selection();
        }
    }
    fn add_char(&mut self, character: char) {
        if self.is_readonly() {
            return;
        }
        if !self.selection.is_empty() {
            self.delete_selection();
        }
        self.glyphs.insert(self.cursor.pos, character);
        self.move_cursor_with(1, false);
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
        let mut ch_selected = Character::with_attributes(' ', theme.editor.pressed_or_selectd);
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
        match key.get_compact_code() {
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
            key!("Ctrl+X") | key!("Shift+Del") => {
                self.cut_text();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                self.paste_text();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+U") => {
                self.convert_to_upper();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+U") => {
                self.convert_to_lower();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+A") => {
                self.select_all();
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.delete_current_character();
                return EventProcessStatus::Processed;
            }
            key!("Back") => {
                self.delete_previous_character();
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
            self.add_char(character);
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for TextField {}

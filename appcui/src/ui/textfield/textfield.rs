use super::Flags;
use crate::prelude::*;
use crate::ui::textfield::events::EventData;
use crate::utils::Glyphs;

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
    fn has_selection(&self) -> bool {
        self.origin != usize::MAX
    }
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct TextField {
    cursor: Cursor,
    selection: Selection,
    glyphs: Glyphs,
    flags: Flags,
}
impl TextField {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            cursor: Cursor { pos: 0, start: 0, end: 0 },
            selection: Selection::NONE,
            glyphs: Glyphs::from(text),
            flags,
        }
    }
    fn move_cursor_with(&mut self, no_of_glyphs: i32, select: bool) {
        let new_poz = if no_of_glyphs>0 {
            self.glyphs.next_pos(self.cursor.pos, no_of_glyphs as usize)
        } else {
            0
        };
        self.move_cursor_to(new_poz, select);
    }
    fn move_cursor_to(&mut self, new_offset: usize, select: bool) {
        todo!()
    }
    fn move_to_next_word(&mut self, select: bool) {
        todo!()
    }
    fn move_to_previous_word(&mut self, select: bool) {
        todo!()
    }
    fn copy_text(&mut self) { todo!() }
    fn paste_text(&mut self) { todo!() }
    fn cut_text(&mut self) { todo!() }
    fn convert_to_upper(&mut self) { todo!() }
    fn convert_to_lower(&mut self) { todo!() }
    fn select_all(&mut self) { todo!() }
    fn delete_current_character(&mut self) { todo!() }
    fn delete_previous_character(&mut self) { todo!() }
    fn add_char(&mut self, character: char) { todo!() }
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
        while let Some((code, glyph_size)) = self.glyphs.character(pos) {
            ch.code = code;
            surface.write_char(x, y, ch);
            if show_cursor && (pos == self.cursor.pos) {
                surface.set_cursor(x, y);
            }
            x += 1;
            if x >= w {
                x = 0;
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
            surface.set_cursor(x, y);
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
                self.move_cursor_with(-1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.move_cursor_with(-((self.size().width as i32) - 2), key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.move_cursor_with(((self.size().width as i32) - 2), key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.move_cursor_to(0, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.move_cursor_to(self.glyphs.len(), key.modifier.contains(KeyModifier::Shift));
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
                    // send validation event
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }
        if (character as u32)>0 {
            self.add_char(character);
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for TextField {}

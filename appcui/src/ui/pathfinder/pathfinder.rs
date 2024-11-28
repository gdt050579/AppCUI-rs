use std::u16;

use crate::prelude::textfield::selection::Selection;

use super::initialization_flags::Flags;
use super::finder::Finder;
use crate::prelude::*;

struct TextArea {
    cursor: usize,
    start: usize,
    end: usize,
    path: String,
    char_count: u32,
    selection: Selection,
    is_readonly: bool,
    width: u32,
}

struct ResultsArea {
    paths: Vec<String>,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    width: u32,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnExpand+OnFocus, internal=true)]
pub struct PathFinder {
    flags: Flags,
    text_area: TextArea,
    results_area: ResultsArea,
    finder: Finder,    
}

impl TextArea {
    const PADDING_LEFT: u16 = 1;
    const PADDING_RIGHT: u16 = 1;
    const PADDING: u16 = Self::PADDING_LEFT + Self::PADDING_RIGHT;
    const PATH_CHAR_SEPARTOR: SpecialChar = SpecialChar::TriangleRight;
    const PATH_CHAR_DOTS: SpecialChar = SpecialChar::ThreePointsHorizontal;

    #[inline(always)]
    fn update_char_count(&mut self, update_size: i32) {
        if self.char_count as i32 + update_size > 0 {
            self.char_count = (self.char_count as i32 + update_size) as u32;
        } else {
            self.char_count = 0;
        }
    }

    fn update_text_area_view(&mut self, force_end_update: bool) {
        if (self.cursor >= self.start) && (self.cursor < self.end) {
            if force_end_update {
                let visible_glyphs = (self.width as usize) - Self::PADDING as usize;
                self.end = self.path.next_pos(self.start, visible_glyphs);
            }
            return;
        }
        let visible_glyphs = (if self.width > Self::PADDING as u32 {
            (self.width as usize) - Self::PADDING as usize
        } else {
            0
        })
        .max(1);

        if self.cursor < self.start {
            self.start = self.cursor;
            self.end = self.path.next_pos(self.cursor, visible_glyphs);
        } else {
            // scroll to the right
            self.start = self.path.previous_pos(self.cursor, visible_glyphs - Self::PADDING_RIGHT as usize);
            // we add ONE to the end pot to satisfy (self.pos < self.end) condition
            self.end = self.cursor;
        }
    }

    fn move_cursor_with(&mut self, no_of_glyphs: i32, select: bool) {
        let new_poz = if no_of_glyphs >= 0 {
            self.path.next_pos(self.cursor, no_of_glyphs as usize)
        } else {
            self.path.previous_pos(self.cursor, (-no_of_glyphs) as usize)
        };
        self.move_cursor_to(new_poz, select, false);
    }

    fn move_cursor_to(&mut self, new_offset: usize, select: bool, force_end_update: bool) {
        let current_pos = self.cursor;
        self.cursor = new_offset.min(self.path.len());
        self.update_text_area_view(force_end_update);
        if select {
            self.selection.update(current_pos, self.cursor);
        } else {
            self.selection = Selection::NONE;
        }
    }

    fn move_cursor_at_end(&mut self) {
        self.move_cursor_to(self.path.len(), false, false);
    }

    fn delete_selection(&mut self) {
        if !self.selection.is_empty() {
            let new_pos = self.selection.start;
            self.path.replace_range(self.selection.start..self.selection.end, "");
            self.selection = Selection::NONE;
            self.move_cursor_to(new_pos, false, true);
        }
    }

    fn delete_current_character(&mut self) {
        if self.is_readonly {
            return;
        }
        if self.selection.is_empty() {
            let next_pos = self.path.next_pos(self.cursor, 1);
            if self.cursor < next_pos {
                self.path.replace_range(self.cursor..next_pos, "");
                self.update_text_area_view(true);
            }
        } else {
            self.delete_selection();
        }
    }

    fn delete_previous_character(&mut self) {
        if self.is_readonly {
            return;
        }
        if self.selection.is_empty() {
            let prev_pos = self.path.previous_pos(self.cursor, 1);
            if prev_pos < self.cursor {
                let end_pos = self.cursor;
                self.path.replace_range(prev_pos..end_pos, "");
                self.move_cursor_to(prev_pos, false, true);
            }
        } else {
            self.delete_selection();
        }
    }

    fn get_out_of_focus_char(&self, ch: char, is_middle: bool) -> char {
        if is_middle {
            return char::from(Self::PATH_CHAR_DOTS);
        }
        match ch {
            '\\' => char::from(Self::PATH_CHAR_SEPARTOR),
            '/' => char::from(Self::PATH_CHAR_SEPARTOR),
            _ => ch,
        }
    }
}

impl ResultsArea {
    const PATH_FINDER_VISIBLE_RESULTS: u16 = 5;
    const PATH_FINDER_RESULTS_Y_OFFSET: u16 = 2;
    const PADDING_LEFT: u16 = 1;
    const PADDING_RIGHT: u16 = 1;
    const PADDING: u16 = Self::PADDING_LEFT + Self::PADDING_RIGHT;
}

impl PathFinder {
    pub fn new(file_path: &str, layout: Layout, flags: Flags) -> Self {
        let mut c = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            text_area: TextArea {
                path: String::from(file_path),
                char_count: file_path.chars().count() as u32,
                cursor: 0,
                start: 0,
                end: 0,
                selection: Selection::NONE,
                is_readonly: flags.contains(Flags::ReadOnly),
                width: 0,                
            },
            results_area: ResultsArea {
                paths: vec![],
                header_y_ofs: 0,
                expanded_panel_y: 1,
                width: 0,
            },
            finder: Finder::new(flags.contains(Flags::ReadOnly)),            
        };
        c.set_size_bounds(4, 1, u16::MAX, u16::MAX);
        c
    }

    fn paint_textbox_in_focus(&self, surface: &mut Surface, attr: CharAttribute, attr_selected: CharAttribute) {
        let sz = self.size();
        let w = (sz.width - 1) as i32;
        let mut count = (sz.width - 2) * sz.height;
        let mut pos = self.text_area.start;
        let mut x = 1;
        let mut y = 0;
        let mut ch = Character::with_attributes(' ', attr);
        let mut ch_selected = Character::with_attributes(' ', attr_selected);
        while let Some((code, glyph_size)) = self.text_area.path.glyph(pos) {
            if self.text_area.selection.contains(pos) {
                ch_selected.code = code;
                surface.write_char(x, y, ch_selected);
            } else {
                ch.code = code;
                surface.write_char(x, y, ch);
            }
            if pos == self.text_area.cursor {
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
        if pos == self.text_area.cursor {
            // if the cursor is located on the fist line outside the view --> put it on the last char but on previous line
            if (y == sz.height as i32) && (x == 1) {
                surface.set_cursor(sz.width as i32 - 1, sz.height as i32 - 1);
            } else {
                surface.set_cursor(x, y);
            }
        }
    }

    fn paint_textbox_out_of_focus(&self, surface: &mut Surface, attr: CharAttribute) {
        // out of focus, draw trimmed path
        let string_fits = self.text_area.char_count < self.size().width;
        let mut start = 0;
        let mut end = self.text_area.path.len();
        while self.text_area.path.glyph(end).is_none() {
            end -= 1;
        }

        let mut left = TextArea::PADDING_LEFT;
        let mut right = (self.size().width - TextArea::PADDING as u32).min(self.text_area.char_count) as u16;

        while left <= right {
            let ch_left = self.text_area.path.glyph(start).unwrap();
            let ch_right = self.text_area.path.glyph(end).unwrap();

            let mut ch = Character::with_attributes(' ', attr);
            ch.code = self.text_area.get_out_of_focus_char(ch_left.0, false);
            surface.write_char(left as i32, 0, ch);
            ch.code = self.text_area.get_out_of_focus_char(ch_right.0, (right - left < 2) && !string_fits);
            surface.write_char(right as i32, 0, ch);

            left += 1;
            right -= 1;
            start = self.text_area.path.next_pos(start, 1);
            end = self.text_area.path.previous_pos(end, 1);
        }
    }

    fn paint_results_area(&self, surface: &mut Surface, attr: CharAttribute, _attr_selected: CharAttribute) {
        let size = self.expanded_size();
        surface.fill_rect(
            Rect::with_size(0, self.results_area.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            Character::with_attributes(' ', attr),
        );
        surface.draw_rect(
            Rect::with_size(0, self.results_area.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
            LineType::Single,
            attr,
        );

        let mut y = self.results_area.expanded_panel_y + 1;
        for path in &self.results_area.paths {
            if y > ResultsArea::PATH_FINDER_VISIBLE_RESULTS as i32 {
                break;
            }
            // paint fitting part of the current path
            let mut offset = 0;
            let mut count_chars = 0;
            while count_chars < self.results_area.width - ResultsArea::PADDING as u32 {
                let old_offset = offset;
                offset = path.next_pos(offset, 1);
                if offset == old_offset {
                    //end of string
                    break;
                }
                count_chars += 1;
            }
            surface.write_string(ResultsArea::PADDING_LEFT as i32, y, &path[..offset], attr, false);
            y += 1;
        }
    }    

    fn expand_results_area(&self) {
        println!("[expand_results_area][PRE expand_results_area] is_expanded = {}, has_focus = {}", self.is_expanded(), self.has_focus());
        if !self.is_expanded() {
            self.expand(
                Size::new(self.size().width, (ResultsArea::PATH_FINDER_VISIBLE_RESULTS + ResultsArea::PATH_FINDER_RESULTS_Y_OFFSET) as u32),
                Size::new(self.size().width, (ResultsArea::PATH_FINDER_VISIBLE_RESULTS + ResultsArea::PATH_FINDER_RESULTS_Y_OFFSET) as u32),
            );
        }        
    }
}

impl OnPaint for PathFinder {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        println!("[on_paint] is_expanded = {}, has_focus = {}", self.is_expanded(), self.has_focus());
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));
        // paint
        if self.has_focus() {
            self.paint_textbox_in_focus(surface, attr, theme.editor.pressed_or_selectd);            
            if self.is_expanded() {
                self.paint_results_area(surface, attr, theme.editor.pressed_or_selectd);            
            }
        } else {
            self.paint_textbox_out_of_focus(surface, attr);
        }
    }
}

impl OnKeyPressed for PathFinder {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Backspace") => {
                self.text_area.delete_previous_character();
                self.text_area.update_char_count(-1);
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.text_area.delete_current_character();
                return EventProcessStatus::Processed;
            }
            key!("Left") | key!("Shift+Left") => {
                self.text_area.move_cursor_with(-1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.text_area.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.text_area.move_cursor_to(0, key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.text_area
                    .move_cursor_to(self.text_area.path.len(), key.modifier.contains(KeyModifier::Shift), false);
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                return EventProcessStatus::Processed;
            }
            key!("Esc") => {
                if self.is_expanded() {
                    self.pack();
                    return EventProcessStatus::Processed;
                }
            }
            _ => {
                if character > 0 as char {
                    self.text_area.path.push(character);
                    self.text_area.update_char_count(1);
                    self.text_area.move_cursor_with(1, key.modifier.contains(KeyModifier::Shift));

                    // TODO: delete this, is for debug purposes                    
                    self.results_area.paths.push(self.text_area.path.clone());                    
                    self.expand_results_area();                    
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for PathFinder {}

impl OnResize for PathFinder {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

impl OnExpand for PathFinder {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        println!("[on_expand] is_expanded = {}, has_focus = {}", self.is_expanded(), self.has_focus());
        match direction {
            ExpandedDirection::OnTop => {
                self.results_area.expanded_panel_y = -1;
                self.results_area.header_y_ofs = (self.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.results_area.expanded_panel_y = 1;
                self.results_area.header_y_ofs = 0;
            }
        }
    }

    fn on_pack(&mut self) {
        println!("[on_pack] is_expanded = {}, has_focus = {}", self.is_expanded(), self.has_focus());
    }
}

impl OnFocus for PathFinder {
    fn on_focus(&mut self) {
        self.text_area.width = self.size().width;
        self.text_area.move_cursor_at_end();
        self.results_area.width = self.size().width;
        self.results_area.paths.clear();
        self.pack();
    }

    fn on_lose_focus(&mut self) {}
}

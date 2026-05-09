use super::{CharClass, Document, Flags, Selection, Line, ViewPort};
use crate::prelude::*;
use ropey::RopeSlice;

struct PaintData {
    margin_width: i32,
    y: i32,
    width: i32,
    horizontal_scroll: i32,
    attr: CharAttribute,
    selection: CharAttribute,
}
#[derive(Default)]
struct MarginSize {
    line_number_width: u8,
    line_number_x: u8,
    width: u8,
}

#[derive(Default)]
struct Cursor {
    column: u32,
    line: u32,
    pos: usize,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Editor {
    document: Document,
    tab_align: i32,
    start_line: u32,
    visible_lines: u32,
    horizontal_scroll: u32,
    cursor: Cursor,
    flags: Flags,
    margin: MarginSize,
    view: ViewPort,
    selection: Selection,
}

impl Editor {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut editor = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            document: Document::new(text),
            tab_align: 4,
            start_line: 0,
            horizontal_scroll: 0,
            cursor: Cursor::default(),
            visible_lines: 0,
            flags,
            margin: MarginSize::default(),
            view: ViewPort::new(16),
            selection: Selection::NONE,
        };
        editor.update_margin();
        editor
    }
    fn update_margin(&mut self) {
        if self.flags.contains(Flags::ShowLineNumbers) {
            let cnt = self.document.lines_count();
            self.margin.line_number_width = if cnt == 0 { 1 } else { (cnt.ilog10() + 1) as u8 };
        } else {
            self.margin.line_number_width = 0;
        }
        let width = self.margin.line_number_width;
        if width > 0 {
            self.margin.line_number_x = 1;
            self.margin.width = width + 2; // one extra space before the text and one extra space after the textfs
        } else {
            self.margin.line_number_x = 0;
        }
    }
    fn position_to_virtual_column(&self, position: usize) -> u32 {
        let line_idx = self.document.position_to_line(position);
        let line_start = self.document.line_to_char(line_idx);
        let line = self.document.line(line_idx);
        let align = self.tab_align as u32;
        let mut x = 0u32;
        for (i, ch) in line.chars().enumerate() {
            if line_start + i >= position {
                break;
            }
            if ch == '\t' {
                x += align - (x % align);
            } else if ch != '\n' {
                x += 1;
            }
        }
        x
    }
    fn coordinates_to_position(&self, line: u32, column: u32) -> usize {
        let ln = self.document.line(line);
        let start = self.document.line_to_char(line);
        let mut x = 0;
        let align = self.tab_align as u32;
        for (i, ch) in ln.chars().enumerate() {
            if x >= column {
                return start + i;
            }
            if ch == '\t' {
                x += align - (x % align);
            } else {
                x += 1;
            }
            if ch == '\n' {
                break;
            }
        }
        let len = ln.len_chars().saturating_sub(1);
        return start + len;
    }

    fn goto_position(&mut self, position: usize, select: bool) {
        let new_pos = position.min(self.document.chars_count());
        let new_current_line = self.document.position_to_line(new_pos);
        if new_current_line < self.start_line {
            self.start_line = new_current_line;
        } else if new_current_line >= self.start_line + self.visible_lines {
            self.start_line = new_current_line - self.visible_lines + 1;
        }
        let sz = self.view.size();
        let visible_width = (sz.width as i32 - self.margin.width as i32).max(0);
        let vcol = self.position_to_virtual_column(new_pos) as u32;

        // if vcol < self.horizontal_scroll {
        //     self.horizontal_scroll = vcol;
        // } else if vcol >= self.horizontal_scroll + visible_width {
        //     self.horizontal_scroll = vcol - visible_width + 1;
        // }
        if select {
            self.selection.update(self.cursor.pos, new_pos);
        } else {
            self.selection.clear();
        }
        self.cursor.pos = new_pos;
        self.cursor.line = new_current_line;
        self.update_view();
    }
    fn move_to_line(&mut self, delta: i32, select: bool) {
        let lines_count = self.document.lines_count();
        if lines_count == 0 {
            return;
        }

        let target_line = (self.cursor.line as i32 + delta).clamp(0, lines_count as i32 - 1) as u32;
        let target_pos = self.coordinates_to_position(target_line, self.cursor.column);
        self.goto_position(target_pos, select);
    }
    fn move_to_next_word(&mut self, select: bool) {
        let mut iter = self.document.chars_iter(self.cursor.pos);
        let Some(first) = iter.next() else {
            return;
        };
        let char_class = CharClass::from(first);
        let mut pos = self.cursor.pos + 1;
        let mut new_char_class = char_class;
        // skip current class
        for c in iter.by_ref() {
            let cc = CharClass::from(c);
            if cc != char_class {
                new_char_class = cc;
                break;
            }
            pos += 1;
        }
        if (new_char_class != char_class) && (new_char_class == CharClass::Space) {
            iter.prev();
            for c in iter.by_ref() {
                if CharClass::from(c) != new_char_class {
                    break;
                }
                pos += 1;
            }
        }
        pos = pos.min(self.document.chars_count());
        self.goto_position(pos, select);
    }
    fn move_to_previous_word(&mut self, select: bool) {
        if self.cursor.pos == 0 {
            return;
        }
        let mut iter = self.document.chars_iter(self.cursor.pos);
        let Some(first) = iter.prev() else {
            return;
        };
        let char_class = CharClass::from(first);
        let mut pos = self.cursor.pos - 1;
        let mut new_char_class = char_class;
        // skip current class
        while let Some(c) = iter.prev() {
            let cc = CharClass::from(c);
            if cc != char_class {
                new_char_class = cc;
                break;
            }
            pos -= 1;
        }
        if (new_char_class != char_class) && (char_class == CharClass::Space) {
            iter.next();
            while let Some(c) = iter.prev() {
                if CharClass::from(c) != new_char_class {
                    break;
                }
                pos -= 1;
            }
        }
        self.goto_position(pos, select);
    }
    fn paint_line_number(&self, surface: &mut Surface, x: i32, y: i32, line_number: u32, attr: CharAttribute) {
        let mut buffer = [b' '; 12];
        let mut pos = 11;
        let mut v = line_number;
        loop {
            buffer[pos] = (v % 10 + 48) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
            pos -= 1;
        }
        surface.write_ascii(x, y, &buffer[12 - self.margin.line_number_width as usize..], attr, false);
    }
    fn paint_line(&self, surface: &mut Surface, p: &PaintData, line: &RopeSlice, mut start_pos: usize) -> Option<u32> {
        let mut virtual_x = 0i32;
        let mut current_column = None;
        for ch in line.chars() {
            let x = virtual_x - p.horizontal_scroll;
            if x >= p.width {
                break;
            }
            if x >= 0 {
                // actual paint
                let chr = Character::with_attributes(
                    if ch as u32 >= 32 { ch } else { ' ' },
                    if self.selection.contains(start_pos) { p.selection } else { p.attr },
                );
                surface.write_char(x + p.margin_width, p.y, chr);
                if start_pos == self.cursor.pos {
                    surface.set_cursor(x + p.margin_width, p.y);
                    current_column = Some(virtual_x as u32);
                }
            }
            if ch == '\n' {
                break;
            }
            // paint character
            if ch == '\t' {
                virtual_x += self.tab_align - (virtual_x % self.tab_align);
            } else {
                virtual_x += 1;
            }
            start_pos += 1;
        }
        current_column
    }
    fn update_view(&mut self) {
        let theme = self.theme();
        let col_normal = theme.editor.normal;
        let mut line_idx = self.start_line;
        let end_idx = (line_idx + self.view.size().height).min(self.document.lines_count() as u32);
        self.view.reset();
        while line_idx < end_idx {
            self.view.update_line(line_idx - self.start_line, line_idx, &self.document, col_normal, self.tab_align as u32);
            line_idx += 1;
        }
        /* 
        if self.view.is_none() {
            return;
        }
        let mut surface = self.view.take().unwrap();
        let theme = self.theme();
        let col_normal = theme.editor.normal;
        let col_inactive = theme.editor.inactive;
        let sz = surface.size();
        surface.clear(Character::with_attributes(' ', col_normal));
        surface.hide_cursor();
        if self.margin.width > 0 {
            let r = Rect::new(0, 0, self.margin.width as i32 - 1, sz.height as i32 - 1);
            surface.fill_rect(r, Character::with_attributes(' ', col_inactive));
        }
        let mut p = PaintData {
            margin_width: self.margin.width as i32,
            y: 0,
            width: (sz.width as i32 - self.margin.width as i32).max(0),
            horizontal_scroll: self.horizontal_scroll,
            attr: theme.editor.normal,
            selection: theme.editor.pressed_or_selected,
        };
        let mut it = self.document.lines_starting_from(self.start_line);
        let mut start_pos = self.document.line_to_char(self.start_line);
        let mut line_number = self.start_line;
        while let Some(line) = it.next() {
            if p.width > 0 {
                if let Some(cc) = self.paint_line(&mut surface, &p, &line, start_pos) {
                    self.cursor.column = cc;
                }
            }
            if self.margin.width > 0 {
                self.paint_line_number(&mut surface, self.margin.line_number_x as i32, p.y, line_number, col_inactive);
            }
            line_number += 1;
            start_pos += line.len_chars();
            p.y += 1;
            if p.y >= sz.height as i32 {
                break;
            }
        }
        self.visible_lines = line_number - self.start_line;
        // important - must be the last
        self.view = Some(surface);
        */
    }
}

impl OnPaint for Editor {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        let x = self.margin.width as i32;
        let w = self.view.size().width;
        let h = self.view.size().height as i32;
        let mut y = 0;
        for line in self.view.lines() {
            surface.write_chars(x, y, line.visible_chars(self.horizontal_scroll, self.horizontal_scroll + w));
            y += 1;
            if y >= h {
                break;
            }
        }
    }
}

impl OnKeyPressed for Editor {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                self.goto_position(self.cursor.pos.saturating_sub(1), select);
                EventProcessStatus::Processed
            }
            key!("Right") | key!("Shift+Right") => {
                self.goto_position(self.cursor.pos.saturating_add(1), select);
                EventProcessStatus::Processed
            }
            key!("Up") | key!("Shift+Up") => {
                self.move_to_line(-1, select);
                EventProcessStatus::Processed
            }
            key!("Down") | key!("Shift+Down") => {
                self.move_to_line(1, select);
                EventProcessStatus::Processed
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                self.move_to_line(-(self.visible_lines as i32).max(1), select);
                EventProcessStatus::Processed
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.move_to_line((self.visible_lines as i32).max(1), select);
                EventProcessStatus::Processed
            }

            // view movement
            key!("Ctrl+Up") => {
                self.start_line = self.start_line.saturating_sub(1);
                self.update_view();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") => {
                self.start_line = (self.start_line + 1).min(self.document.lines_count() as u32);
                self.update_view();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                self.move_to_previous_word(select);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                self.move_to_next_word(select);
                EventProcessStatus::Processed
            }
            key!("Home") | key!("Shift+Home") => {
                let line = self.cursor.line;
                self.goto_position(self.document.line_to_char(line), select);
                EventProcessStatus::Processed
            }
            key!("End") | key!("Shift+End") => {
                let line = self.cursor.line;
                self.goto_position(self.document.line_end_position(line), select);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Home") | key!("Ctrl+Shift+Home") => {
                self.goto_position(0, select);
                EventProcessStatus::Processed
            }
            key!("Ctrl+End") | key!("Ctrl+Shift+End") => {
                let end = self.document.chars_count();
                self.goto_position(end, select);
                EventProcessStatus::Processed
            }

            // clipboard+selectin
            key!("Ctrl+A") => {
                let end = self.document.chars_count();
                self.selection.set(0, end);
                self.goto_position(end, true);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnMouseEvent for Editor {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnResize for Editor {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.view.resize(new_size);
        self.update_view();
    }
}

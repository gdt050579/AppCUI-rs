use super::{CharClass, Document, Flags, Selection, ViewPort};
use crate::prelude::*;

#[derive(Default)]
struct MarginSize {
    line_number_width: u8,
    line_number_x: u8,
    width: u8,
    visible: bool,
}

#[derive(Default)]
struct Cursor {
    column: u32,
    line: u32,
    pos: usize,
    visible: bool,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Editor {
    document: Document,
    tab_align: i32,
    start_line: u32,
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
        if select {
            self.selection.update(self.cursor.pos, new_pos);
        } else {
            self.selection.clear();
        }
        let pos_info = self.document.char_to_pos_info(new_pos);
        if !self.view.contains_line(pos_info.line_index) {
            if pos_info.line_index < self.view.first_line() {
                self.start_line = pos_info.line_index;
            } else if pos_info.line_index > self.view.last_line() {
                let h = self.view.size().height.saturating_sub(1);
                self.start_line = pos_info.line_index.saturating_sub(h);
            }
            self.update_view();
        }
        self.cursor.pos = new_pos;
        self.cursor.line = pos_info.line_index;
        if self.view.contains_line(pos_info.line_index) {
            if let Some(x_offset) = self.view.x_offset(pos_info.line_index, pos_info.rel_offset as usize) {
                self.cursor.visible = true;
                // update horizontal scroll
                let w = self.view.size().width;
                if x_offset < self.horizontal_scroll {
                    self.horizontal_scroll = x_offset;
                }
                if x_offset >= self.horizontal_scroll + w {
                    self.horizontal_scroll = (x_offset - w) + 1;
                }
                self.cursor.column = x_offset - self.horizontal_scroll;
            } else {
                self.cursor.column = 0;
                self.cursor.visible = false;
            }
        } else {
            self.cursor.visible = false;
        }
    }
    fn move_to_line(&mut self, delta: i32, select: bool) {
        let lines_count = self.document.lines_count();
        if lines_count == 0 {
            return;
        }
        let new_line = (self.cursor.line as i32 + delta).clamp(0, lines_count as i32 - 1) as u32;
        let new_pos = self.coordinates_to_position(new_line, self.cursor.column);
        self.goto_position(new_pos, select);
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
    fn update_view(&mut self) {
        let theme = self.theme();
        let col_normal = theme.editor.normal;
        let mut line_idx = self.start_line;
        let end_idx = (line_idx + self.view.size().height).min(self.document.lines_count() as u32);
        self.view.reset();
        while line_idx < end_idx {
            self.view
                .update_line(line_idx - self.start_line, line_idx, &self.document, col_normal, self.tab_align as u32);
            line_idx += 1;
        }
    }
}

impl OnPaint for Editor {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let x = if self.margin.visible { self.margin.width as i32 } else { 0 };
        let w = self.view.size().width;
        let h = self.view.size().height as i32;
        let mut y = 0;
        surface.clear(Character::with_attributes(' ', theme.editor.normal));
        for line in self.view.lines() {
            if self.margin.visible {
                self.paint_line_number(surface, 0, y, line.line_number(), theme.editor.inactive);
            }
            surface.write_chars(x, y, line.visible_chars(self.horizontal_scroll, self.horizontal_scroll + w));
            y += 1;
            if y >= h {
                break;
            }
        }
        if self.has_focus() {
            // TODO: paint selection
            // show cursor
            if self.cursor.visible {
                let y = self.cursor.line.saturating_sub(self.start_line) as i32;
                surface.set_cursor(self.cursor.column as i32 + x, y);
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
                self.move_to_line(-(self.view.size().height as i32), select);
                EventProcessStatus::Processed
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.move_to_line(self.view.size().height as i32, select);
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
        if new_size.width <= self.margin.width as u32 {
            self.margin.visible = false;
            self.view.resize(new_size);
        } else {
            self.margin.visible = true;
            self.view
                .resize(Size::new(new_size.width.saturating_sub(self.margin.width as u32), new_size.height));
        }
        self.update_view();
    }
}

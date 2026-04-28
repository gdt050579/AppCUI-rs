use ropey::RopeSlice;

use super::{Document, Flags};
use crate::prelude::*;

struct PaintData {
    margin_width: i32,
    y: i32,
    width: i32,
    horizontal_scroll: i32,
    attr: CharAttribute,
}
#[derive(Default)]
struct MarginSize {
    line_number_width: u8,
    line_number_x: u8,
    width: u8,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Editor {
    document: Document,
    tab_align: i32,
    start_line: u32,
    pos: usize,
    flags: Flags,
    margin: MarginSize,
}

impl Editor {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut editor = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            document: Document::new(text),
            tab_align: 4,
            start_line: 0,
            pos: 0,
            flags,
            margin: MarginSize::default(),
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
    fn paint_line(&self, surface: &mut Surface, p: &PaintData, line: &RopeSlice, mut start_pos: usize) {
        let mut virtual_x = 0i32;
        for ch in line.chars() {
            let x = virtual_x - p.horizontal_scroll;
            if x >= p.width {
                break;
            }
            if x >= 0 {
                // actual paint
                let chr = Character::with_attributes(if ch as u32 >= 32 { ch } else { ' ' }, p.attr);
                surface.write_char(x+p.margin_width, p.y, chr);
                if start_pos == self.pos {
                    surface.set_cursor(x+p.margin_width, p.y);
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
    }
}

impl OnPaint for Editor {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        surface.clear(Character::with_attributes(' ', theme.editor.normal));
        if self.margin.width > 0 {
            let r = Rect::new(0, 0, self.margin.width as i32 - 1, sz.height as i32 - 1);
            surface.fill_rect(r, Character::with_attributes(' ', theme.editor.inactive));
        }
        let mut p = PaintData {
            margin_width: self.margin.width as i32,
            y: 0,
            width: (sz.width as i32 - self.margin.width as i32).max(0),
            horizontal_scroll: 0,
            attr: theme.editor.normal,
        };
        let mut it = self.document.lines_starting_from(self.start_line);
        let mut start_pos = self.document.line_to_char(self.start_line);
        let mut line_number = self.start_line;
        while let Some(line) = it.next() {
            if p.width > 0 {
                self.paint_line(surface, &p, &line, start_pos);
            }
            if self.margin.width > 0 {
                self.paint_line_number(surface, self.margin.line_number_x as i32, p.y, line_number, theme.editor.inactive);
            }
            line_number += 1;
            start_pos += line.len_chars();
            p.y += 1;
            if p.y >= sz.height as i32 {
                break;
            }
        }
    }
}

impl OnKeyPressed for Editor {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for Editor {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

use crate::prelude::*;
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::{Parser, Span};

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct MarkdownComposer {
    text: Vec<char>,
    parser: Parser,
    cursor: usize,
    spans: Vec<Span>,
}

impl MarkdownComposer {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: Vec::new(),
            parser: Parser::new(),
            cursor: 0,
            spans: Vec::new(),
        };
        mc
    }
    fn set_spans(&mut self) {
        let width = self.size().width;
        self.spans = self.parser.parse(&self.text, width as u16);
    }
}

impl OnPaint for MarkdownComposer {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {

        let backgorund_theme = Color::Black;
        let width = self.size().width;
        let normal_attr = CharAttribute::new(Color::White, Color::Black, CharFlags::None);
        surface.clear(Character::with_attributes(' ', normal_attr));
        
        let cursor = self.cursor.min(self.text.len());
        
        let mut cursor_col: i32 = 0;
        let mut cursor_row: i32 = 0;
        let mut cursor_from: usize = 0;
        
        for span in &self.spans {
            let Span { start, end, x_pos, y_pos, color, flags } = *span;
            let start = start as usize;
            let end = end as usize;
        
            let char_attr = CharAttribute::new(color, backgorund_theme, flags);
            let piece = self.text[start..end].iter().collect::<String>();
            surface.write_string(x_pos as i32, y_pos as i32, &piece, char_attr, false);
        
            if cursor >= start {
                let offset = cursor.min(end) - start;
                cursor_col = x_pos as i32 + offset as i32;
                cursor_row = y_pos as i32;
                cursor_from = cursor.min(end);
            }
        }
        
        for &ch in &self.text[cursor_from..cursor] {
            if ch == '\n' {
                cursor_col = 0;
                cursor_row += 1;
            }
        }
        
        surface.set_cursor(cursor_col, cursor_row);
    }
}

impl OnResize for MarkdownComposer {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.set_spans();
    }
}

impl OnMouseEvent for MarkdownComposer {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnKeyPressed for MarkdownComposer {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.code {
            KeyCode::Enter => {
                self.text.insert(self.cursor, '\n');
                self.cursor += 1;
                self.set_spans();
                return EventProcessStatus::Processed;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                    self.set_spans();
                }
                return EventProcessStatus::Processed;
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.set_spans();
                }
                return EventProcessStatus::Processed;
            }
            KeyCode::Right => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                    self.set_spans();
                }
                return EventProcessStatus::Processed;
            }
            _ => {}
        }

        if character != '\0' {
            self.text.insert(self.cursor, character);
            self.cursor += 1;
            self.set_spans();
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

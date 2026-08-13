use crate::prelude::*;
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::{Parser, Span, SpanType};

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct MarkdownComposer {
    text: String,
    surface: Surface,
    parser: Parser,
    cursor: u32,
    cursor_x: u32,
    cursor_y: u32,
    anchor: Option<u32>,
}

impl MarkdownComposer {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: String::new(),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
        };
        mc
    }

    pub fn from(text: &str, layout: Layout, flags: Flags) -> Self {
        let mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: text.to_string(),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
        };
        mc
    }

    fn update_surface(&mut self) {
        let width = self.size().width.max(1);
        let height = self.size().height.max(1);

        if self.surface.size() != Size::new(width, height) {
            self.surface = Surface::new(width, height);
        }

        self.parser.parse(&self.text, width);

        let (cursor_x, cursor_y) = self.parser.position(&self.text, self.cursor);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        let foreground = self.theme().editor.normal.foreground;
        let background = self.theme().editor.normal.background;

        self.surface.clear(Character::with_attributes(
            ' ',
            CharAttribute::new(foreground, background, CharFlags::None),
        ));

        Self::paint_normal(&self.text, self.parser.spans(), &mut self.surface, foreground, background);
    }

    fn span_attr(span_type: SpanType, foreground: Color, background: Color) -> CharAttribute {
        let mut foreground = foreground;
        let mut flags = CharFlags::None;

        if span_type.contains(SpanType::Bold) {
            flags |= CharFlags::Bold;
        }
        if span_type.contains(SpanType::Italic) {
            flags |= CharFlags::Italic;
        }
        if span_type.contains_one(SpanType::Link | SpanType::Email) {
            foreground = Color::Aqua;
            flags |= CharFlags::Underline;
        }

        CharAttribute::new(foreground, background, flags)
    }

    fn paint_normal(text: &str, spans: &[Span], surface: &mut Surface, foreground: Color, background: Color) {
        let bytes = text.as_bytes();

        for span in spans {
            let attr = Self::span_attr(span.span_type, foreground, background);

            let mut x = span.x_pos as i32;
            let y = span.y_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::char_len_from_first_byte(bytes[i]);
                let ch = Parser::char_at(bytes, i, len);

                surface.write_char(x, y, Character::with_attributes(ch, attr));

                x += Parser::char_width_from_byte_len(len);
                i += len;
            }
        }
    }

    fn selection(&self) -> Option<(u32, u32)> {
        let anchor = self.anchor?;
        if anchor == self.cursor {
            return None;
        }
        Some((anchor.min(self.cursor), anchor.max(self.cursor)))
    }

    fn remove_selection(&mut self) -> bool {
        let Some((start, end)) = self.selection() else {
            return false;
        };
        self.text.replace_range(start as usize..end as usize, "");
        self.cursor = start;
        self.anchor = None;
        true
    }

    fn insert(&mut self, character: char) {
        self.remove_selection();
        self.text.insert(self.cursor as usize, character);
        self.cursor += character.len_utf8() as u32;
        self.update_surface();
    }

    fn insert_text(&mut self, added: &str) {
        self.remove_selection();
        self.text.insert_str(self.cursor as usize, added);
        self.cursor += added.len() as u32;
        self.update_surface();
    }

    fn delete_previous(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let previous = Parser::prev_offset(&self.text, self.cursor);
        if previous == self.cursor {
            return;
        }
        self.text.replace_range(previous as usize..self.cursor as usize, "");
        self.cursor = previous;
        self.update_surface();
    }

    fn delete_current(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let next = Parser::next_offset(&self.text, self.cursor);
        if next == self.cursor {
            return;
        }
        self.text.replace_range(self.cursor as usize..next as usize, "");
        self.update_surface();
    }

    fn copy_selection(&self) -> bool {
        let Some((start, end)) = self.selection() else {
            return false;
        };
        Clipboard::set_text(&self.text[start as usize..end as usize]);
        true
    }

    fn cut_selection(&mut self) {
        if self.copy_selection() && self.remove_selection() {
            self.update_surface();
        }
    }

    fn paste(&mut self) {
        let Some(pasted) = Clipboard::text() else {
            return;
        };
        if pasted.is_empty() {
            return;
        }
        if pasted.contains('\r') {
            let cleaned: String = pasted.chars().filter(|&c| c != '\r').collect();
            self.insert_text(&cleaned);
        } else {
            self.insert_text(&pasted);
        }
    }

    fn move_to(&mut self, offset: u32, select: bool) {
        if select {
            if self.anchor.is_none() {
                self.anchor = Some(self.cursor);
            }
        } else {
            self.anchor = None;
        }

        if offset == self.cursor {
            return;
        }
        self.cursor = offset;
        let (cursor_x, cursor_y) = self.parser.position(&self.text, self.cursor);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;
    }

    fn paint_selection(&self, surface: &mut Surface, theme: &Theme) {
        let Some((start, end)) = self.selection() else {
            return;
        };

        let attr = theme.editor.pressed_or_selected;
        let bytes = self.text.as_bytes();

        for span in self.parser.spans() {
            if span.end <= start || span.start >= end {
                continue;
            }

            let mut x = span.x_pos as i32;
            let y = span.y_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::char_len_from_first_byte(bytes[i]);

                if i >= start as usize && i < end as usize {
                    let ch = Parser::char_at(bytes, i, len);
                    surface.write_char(x, y, Character::with_attributes(ch, attr));
                }

                x += Parser::char_width_from_byte_len(len);
                i += len;
            }
        }
    }
}

impl OnPaint for MarkdownComposer {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
        self.paint_selection(surface, theme);

        if self.has_focus() {
            surface.set_cursor(self.cursor_x as i32, self.cursor_y as i32);
        } else {
            surface.hide_cursor();
        }
    }
}

impl OnResize for MarkdownComposer {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.update_surface();
    }
}

impl OnMouseEvent for MarkdownComposer {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(data) => {
                let offset = self.parser.offset_at(&self.text, data.x.max(0) as u32, data.y.max(0) as u32);
                self.move_to(offset, false);
                self.anchor = Some(offset);
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                let offset = self.parser.offset_at(&self.text, data.x.max(0) as u32, data.y.max(0) as u32);
                self.move_to(offset, true);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for MarkdownComposer {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);

        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                let offset = self.parser.prev_visible_offset(&self.text, self.cursor);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                let offset = self.parser.next_visible_offset(&self.text, self.cursor);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                if self.cursor_y > 0 {
                    let offset = self.parser.offset_at(&self.text, self.cursor_x, self.cursor_y - 1);
                    self.move_to(offset, select);
                }
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                let offset = self.parser.offset_at(&self.text, self.cursor_x, self.cursor_y + 1);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                let offset = self.parser.offset_at(&self.text, 0, self.cursor_y);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                let offset = self.parser.offset_at(&self.text, u32::MAX, self.cursor_y);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Home") | key!("Ctrl+Shift+Home") => {
                self.move_to(0, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+End") | key!("Ctrl+Shift+End") => {
                let offset = self.text.len() as u32;
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+A") => {
                self.anchor = Some(0);
                let offset = self.text.len() as u32;
                self.move_to(offset, true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                self.copy_selection();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+X") | key!("Shift+Delete") => {
                self.cut_selection();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+V") | key!("Shift+Insert") => {
                self.paste();
                return EventProcessStatus::Processed;
            }
            key!("Backspace") => {
                self.delete_previous();
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.delete_current();
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                self.insert('\n');
                return EventProcessStatus::Processed;
            }
            _ => {}
        }

        if (character as u32) > 0 {
            self.insert(character);
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

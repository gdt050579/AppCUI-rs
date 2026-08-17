use crate::prelude::*;
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::Parser;
use crate::ui::markdown_composer::parser::{Span, SpanType};

const WHEEL_ROWS: u32 = 3;
const BULLET: char = '\u{2022}';
const QUOTE_BAR: char = '\u{2502}';

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct MarkdownComposer {
    text: String,
    surface: Surface,
    parser: Parser,
    cursor_offset: u32,
    cursor_x: u32,
    cursor_y: u32,
    anchor: Option<u32>,
    first_row: u32,
    rows: u32,
}

impl MarkdownComposer {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: String::new(),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor_offset: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
            first_row: 0,
            rows: 1,
        };
        mc
    }

    pub fn from(text: &str, layout: Layout, flags: Flags) -> Self {
        let mc = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            text: Self::normalize_newlines(text),
            parser: Parser::new(),
            surface: Surface::new(1, 1),
            cursor_offset: 0,
            cursor_x: 0,
            cursor_y: 0,
            anchor: None,
            first_row: 0,
            rows: 1,
        };
        mc
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Self::normalize_newlines(text);
        self.cursor_offset = 0;
        self.anchor = None;
        self.first_row = 0;
        self.update_surface();
    }

    pub fn show_markers(&self) -> bool {
        self.parser.show_markers()
    }

    pub fn set_show_markers(&mut self, show_markers: bool) {
        if self.parser.show_markers() == show_markers {
            return;
        }
        self.parser.set_show_markers(show_markers);
        self.update_surface();
    }

    fn normalize_newlines(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\r' {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push('\n');
            } else {
                out.push(ch);
            }
        }

        out
    }

    fn update_surface(&mut self) {
        let width = self.size().width.max(1);
        let height = self.size().height.max(1);

        if self.surface.size() != Size::new(width, height) {
            self.surface = Surface::new(width, height);
        }

        self.parser.parse(&self.text, width);
        self.rows = self.parser.rows(&self.text);

        let (cursor_x, cursor_y) = self.parser.get_position_from_offset(&self.text, self.cursor_offset);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        self.clamp_first_row();
        self.ensure_visible();
        self.redraw();
    }

    fn redraw(&mut self) {
        let foreground = self.theme().editor.normal.foreground;
        let background = self.theme().editor.normal.background;

        self.surface.clear(Character::with_attributes(
            ' ',
            CharAttribute::new(foreground, background, CharFlags::None),
        ));

        Self::paint_normal(
            &self.text,
            self.parser.spans(),
            &mut self.surface,
            foreground,
            background,
            self.first_row,
        );
    }

    fn clamp_first_row(&mut self) {
        let height = self.size().height.max(1);
        self.first_row = self.first_row.min(self.rows.saturating_sub(height));
    }

    fn ensure_visible(&mut self) {
        let height = self.size().height.max(1);

        if self.cursor_y < self.first_row {
            self.first_row = self.cursor_y;
        } else if self.cursor_y >= self.first_row + height {
            self.first_row = self.cursor_y - height + 1;
        }
    }

    fn scroll_to(&mut self, row: u32) {
        let height = self.size().height.max(1);
        let row = row.min(self.rows.saturating_sub(height));

        if row != self.first_row {
            self.first_row = row;
            self.redraw();
        }
    }

    fn span_attr(span_type: SpanType, foreground: Color, background: Color) -> CharAttribute {
        let mut foreground = foreground;
        let mut background = background;
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
        if span_type.contains(SpanType::Code) {
            foreground = Color::Yellow;
            background = Color::DarkBlue;
        } else if span_type.contains(SpanType::CodeBlock) {
            foreground = Color::Silver;
            background = Color::DarkBlue;
        }

        CharAttribute::new(foreground, background, flags)
    }

    fn code_block_attr() -> CharAttribute {
        CharAttribute::new(Color::Silver, Color::DarkBlue, CharFlags::None)
    }

    fn quote_bar_attr(background: Color) -> CharAttribute {
        CharAttribute::new(Color::Gray, background, CharFlags::None)
    }

    fn paint_normal(
        text: &str,
        spans: &[Span],
        surface: &mut Surface,
        foreground: Color,
        background: Color,
        first_row: u32,
    ) {
        let bytes = text.as_bytes();
        let size = surface.size();
        let height = size.height as i32;
        let width = size.width as i32;

        for (index, span) in spans.iter().enumerate() {
            if !span.span_type.contains(SpanType::CodeBlock) {
                continue;
            }

            let last_on_row = match spans.get(index + 1) {
                Some(next) => next.y_pos != span.y_pos,
                None => true,
            };
            if !last_on_row {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            surface.fill_horizontal_line(
                span.x_pos as i32,
                y,
                width - 1,
                Character::with_attributes(' ', Self::code_block_attr()),
            );
        }

        for span in spans {
            if !span.span_type.contains(SpanType::Quote) {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            surface.write_char(
                0,
                y,
                Character::with_attributes(QUOTE_BAR, Self::quote_bar_attr(background)),
            );
        }

        for span in spans {
            if span.span_type.contains(SpanType::QuoteMark) {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            let attr = Self::span_attr(span.span_type, foreground, background);
            let bullet = span.span_type.contains(SpanType::Bullet);

            let mut x = span.x_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::get_char_len(bytes[i]);
                let ch = if bullet {
                    BULLET
                } else {
                    Parser::get_char(bytes, i, len)
                };

                surface.write_char(x, y, Character::with_attributes(ch, attr));

                x += Parser::get_char_width(len);
                i += len;
            }
        }
    }

    fn paint_selection(&self, surface: &mut Surface, theme: &Theme) {
        let Some((start, end)) = self.selection() else {
            return;
        };

        let attr = theme.editor.pressed_or_selected;
        let bytes = self.text.as_bytes();
        let height = surface.size().height as i32;

        for span in self.parser.spans() {
            if span.end <= start || span.start >= end {
                continue;
            }

            let y = span.y_pos as i32 - self.first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            let bullet = span.span_type.contains(SpanType::Bullet);
            let quote_mark = span.span_type.contains(SpanType::QuoteMark);
            let mut x = span.x_pos as i32;
            let mut i = span.start as usize;

            while i < span.end as usize {
                let len = Parser::get_char_len(bytes[i]);

                if i >= start as usize && i < end as usize {
                    let ch = if bullet {
                        BULLET
                    } else if quote_mark {
                        QUOTE_BAR
                    } else {
                        Parser::get_char(bytes, i, len)
                    };
                    surface.write_char(x, y, Character::with_attributes(ch, attr));
                }

                x += Parser::get_char_width(len);
                i += len;
            }
        }
    }

    fn selection(&self) -> Option<(u32, u32)> {
        let limit = self.text.len() as u32;
        let anchor = self.anchor?.min(limit);
        let cursor_offset = self.cursor_offset.min(limit);

        if anchor == cursor_offset {
            return None;
        }
        Some((anchor.min(cursor_offset), anchor.max(cursor_offset)))
    }

    fn remove_selection(&mut self) -> bool {
        let removed = match self.selection() {
            Some((start, end)) => {
                self.text.replace_range(start as usize..end as usize, "");
                self.cursor_offset = start;
                true
            }
            None => false,
        };

        self.anchor = None;
        removed
    }

    fn word_at(&self, offset: u32) -> (u32, u32) {
        let bytes = self.text.as_bytes();
        let offset = (offset as usize).min(bytes.len());

        let mut start = offset;
        while start > 0 && !bytes[start - 1].is_ascii_whitespace() {
            start = Parser::prev_offset(&self.text, start as u32) as usize;
        }

        let mut end = offset;
        while end < bytes.len() && !bytes[end].is_ascii_whitespace() {
            end = Parser::next_offset(&self.text, end as u32) as usize;
        }

        (start as u32, end as u32)
    }

    fn insert(&mut self, character: char) {
        self.remove_selection();
        self.text.insert(self.cursor_offset as usize, character);
        self.cursor_offset += character.len_utf8() as u32;
        self.update_surface();
    }

    fn insert_text(&mut self, added: &str) {
        self.remove_selection();
        self.text.insert_str(self.cursor_offset as usize, added);
        self.cursor_offset += added.len() as u32;
        self.update_surface();
    }

    fn delete_previous(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let previous = Parser::prev_offset(&self.text, self.cursor_offset);
        if previous == self.cursor_offset {
            return;
        }
        self.text.replace_range(previous as usize..self.cursor_offset as usize, "");
        self.cursor_offset = previous;
        self.update_surface();
    }

    fn delete_current(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let next = Parser::next_offset(&self.text, self.cursor_offset);
        if next == self.cursor_offset {
            return;
        }
        self.text.replace_range(self.cursor_offset as usize..next as usize, "");
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
            let normalized = Self::normalize_newlines(&pasted);
            self.insert_text(&normalized);
        } else {
            self.insert_text(&pasted);
        }
    }

    fn move_to(&mut self, offset: u32, select: bool) {
        if select {
            if self.anchor.is_none() {
                self.anchor = Some(self.cursor_offset);
            }
        } else {
            self.anchor = None;
        }

        if offset == self.cursor_offset {
            return;
        }
        self.cursor_offset = offset;
        let (cursor_x, cursor_y) = self.parser.get_position_from_offset(&self.text, self.cursor_offset);
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        let first_row = self.first_row;
        self.ensure_visible();
        if self.first_row != first_row {
            self.redraw();
        }
    }

    fn offset_at_screen(&self, x: i32, y: i32) -> u32 {
        let height = self.size().height.max(1) as i32;
        let y = y.clamp(0, height - 1) as u32 + self.first_row;
        self.parser.get_offset_from_position(&self.text, x.max(0) as u32, y)
    }

    fn scroll_towards(&mut self, y: i32) {
        let height = self.size().height.max(1) as i32;

        if y < 0 {
            let row = self.first_row.saturating_sub((-y) as u32);
            self.scroll_to(row);
        } else if y >= height {
            let row = self.first_row + (y - height + 1) as u32;
            self.scroll_to(row);
        }
    }
}

impl OnPaint for MarkdownComposer {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
        self.paint_selection(surface, theme);

        if self.has_focus() {
            surface.set_cursor(self.cursor_x as i32, self.cursor_y as i32 - self.first_row as i32);
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
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, false);
                self.anchor = Some(offset);
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) => {
                let offset = self.offset_at_screen(data.x, data.y);
                let (start, end) = self.word_at(offset);
                self.anchor = Some(start);
                self.move_to(end, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                self.scroll_towards(data.y);
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Up => {
                        let row = self.first_row.saturating_sub(WHEEL_ROWS);
                        self.scroll_to(row);
                    }
                    MouseWheelDirection::Down => {
                        let row = self.first_row + WHEEL_ROWS;
                        self.scroll_to(row);
                    }
                    _ => return EventProcessStatus::Ignored,
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for MarkdownComposer {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        let height = self.size().height.max(1);

        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                let offset = self.parser.prev_visible_offset(&self.text, self.cursor_offset);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                let offset = self.parser.next_visible_offset(&self.text, self.cursor_offset);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                if self.cursor_y > 0 {
                    let offset = self.parser.get_offset_from_position(&self.text, self.cursor_x, self.cursor_y - 1);
                    self.move_to(offset, select);
                }
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                let offset = self.parser.get_offset_from_position(&self.text, self.cursor_x, self.cursor_y + 1);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                let row = self.cursor_y.saturating_sub(height);
                let offset = self.parser.get_offset_from_position(&self.text, self.cursor_x, row);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                let row = (self.cursor_y + height).min(self.rows.saturating_sub(1));
                let offset = self.parser.get_offset_from_position(&self.text, self.cursor_x, row);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                let offset = self.parser.get_offset_from_position(&self.text, 0, self.cursor_y);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                let offset = self.parser.get_offset_from_position(&self.text, u32::MAX, self.cursor_y);
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
            key!("Ctrl+Up") => {
                let row = self.first_row.saturating_sub(1);
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") => {
                let row = self.first_row + 1;
                self.scroll_to(row);
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

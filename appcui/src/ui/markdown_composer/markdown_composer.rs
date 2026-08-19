use crate::graphics::SpecialChar;
use crate::prelude::*;
use crate::ui::markdown_composer::parser::{Span, SpanType};
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::List;
use crate::ui::markdown_composer::Parser;

const WHEEL_ROWS: u32 = 3;
const BULLET: SpecialChar = SpecialChar::CircleFilled;
const QUOTE_BAR: SpecialChar = SpecialChar::BoxVerticalSingleLine;
const POPUP_ROWS: u32 = 4;
const POPUP_MIN_WIDTH: u32 = 12;
const POPUP_MAX_WIDTH: u32 = 40;

struct Popup {
    list: usize,
    start: u32,
    matches: Vec<u32>,
    index: u32,
    first: u32,
}

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed+OnExpand, internal=true)]
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
    lists: Vec<List>,
    popup: Option<Popup>,
    expanded: bool,
    expanded_offset: i32,
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
            lists: Vec::new(),
            popup: None,
            expanded: false,
            expanded_offset: 0,
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
            lists: Vec::new(),
            popup: None,
            expanded: false,
            expanded_offset: 0,
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

        Self::paint_normal(&self.text, self.parser.spans(), &mut self.surface, foreground, background, self.first_row);
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

    pub fn add_list(&mut self, list: List) {
        self.popup_close_list();
        match self.lists.iter().position(|item| item.trigger() == list.trigger()) {
            Some(index) => self.lists[index] = list,
            None => self.lists.push(list),
        }
    }

    pub fn list(&self, trigger: char) -> Option<&List> {
        self.lists.iter().find(|list| list.trigger() == trigger)
    }

    pub fn list_mut(&mut self, trigger: char) -> Option<&mut List> {
        self.popup_close_list();
        self.lists.iter_mut().find(|list| list.trigger() == trigger)
    }

    fn popup_find_list(&self, trigger: char) -> Option<usize> {
        self.lists.iter().position(|list| list.trigger() == trigger)
    }

    fn popup_is_list_start(&self, offset: u32) -> bool {
        if offset == 0 {
            return true;
        }
        let bytes = self.text.as_bytes();
        let previous = Parser::prev_offset(&self.text, offset) as usize;
        if previous >= bytes.len() {
            return false;
        }
        let len = Parser::get_char_len(bytes[previous]);
        Parser::get_char(bytes, previous, len).is_whitespace()
    }

    fn popup_open_list(&mut self, trigger: char, start: u32) {
        let Some(list) = self.popup_find_list(trigger) else {
            return;
        };

        self.popup = Some(Popup {
            list,
            start,
            matches: Vec::new(),
            index: 0,
            first: 0,
        });
        self.popup_match_items();
    }

    fn popup_close_list(&mut self) {
        self.popup = None;
        self.pack();
    }

    fn popup_match_items(&mut self) {
        let Some(mut popup) = self.popup.take() else {
            return;
        };

        let from = popup.start as usize + self.lists[popup.list].trigger().len_utf8();
        let to = self.cursor_offset as usize;

        if from > to || to > self.text.len() || self.text[from..to].contains('\n') {
            self.popup_close_list();
            return;
        }

        let filter = self.text[from..to].to_lowercase();
        popup.matches.clear();

        {
            let list = &self.lists[popup.list];
            for index in 0..list.len() {
                let matched = match list.item(index) {
                    Some(item) => filter.is_empty() || item.to_lowercase().contains(&filter),
                    None => false,
                };
                if matched {
                    popup.matches.push(index);
                }
            }
        }

        if popup.matches.is_empty() {
            self.popup_close_list();
            return;
        }

        popup.index = popup.index.min(popup.matches.len() as u32 - 1);
        if popup.index < popup.first {
            popup.first = popup.index;
        } else if popup.index >= popup.first + POPUP_ROWS {
            popup.first = popup.index - POPUP_ROWS + 1;
        }

        self.popup = Some(popup);
        self.popup_expand_list();
    }

    fn popup_move_selection(&mut self, delta: i32) {
        let Some(popup) = self.popup.as_mut() else {
            return;
        };

        let count = popup.matches.len() as i32;
        if count == 0 {
            return;
        }

        popup.index = (popup.index as i32 + delta).clamp(0, count - 1) as u32;

        if popup.index < popup.first {
            popup.first = popup.index;
        } else if popup.index >= popup.first + POPUP_ROWS {
            popup.first = popup.index - POPUP_ROWS + 1;
        }
    }

    fn popup_insert_item(&mut self) {
        let Some(popup) = self.popup.take() else {
            return;
        };

        let mut replacement = String::new();
        if let Some(&item) = popup.matches.get(popup.index as usize) {
            let list = &self.lists[popup.list];
            if let Some(value) = list.item(item) {
                replacement.push(list.trigger());
                replacement.push_str(value);
            }
        }

        self.popup_close_list();
        if replacement.is_empty() {
            return;
        }

        let start = popup.start as usize;
        let end = (self.cursor_offset as usize).min(self.text.len()).max(start);
        self.text.replace_range(start..end, &replacement);
        self.cursor_offset = (start + replacement.len()) as u32;
        self.anchor = None;
        self.update_surface();
    }

    fn popup_get_list_rect(&self) -> Option<(i32, i32, u32, u32)> {
        let popup = self.popup.as_ref()?;
        let list = self.lists.get(popup.list)?;

        let mut text_width = 0;
        for &item in &popup.matches {
            if let Some(value) = list.item(item) {
                text_width = text_width.max(value.chars().count() as u32);
            }
        }

        let width = (text_width + 4).clamp(POPUP_MIN_WIDTH, POPUP_MAX_WIDTH);
        let height = (popup.matches.len() as u32).min(POPUP_ROWS) + 2;

        let size = self.size();
        let (trigger_x, trigger_y) = self.parser.get_position_from_offset(&self.text, popup.start);
        let x = (trigger_x as i32).clamp(0, (size.width as i32 - width as i32).max(0));
        let row = trigger_y as i32 - self.first_row as i32;

        let below = row + 1;
        if below + (height as i32) <= size.height as i32 {
            return Some((x, below, width, height));
        }

        let above = row - height as i32;
        if above >= 0 {
            return Some((x, above, width, height));
        }

        Some((x, size.height as i32, width, height))
    }

    fn popup_get_paint_rect(&self) -> Option<(i32, i32, u32, u32)> {
        let (x, y, width, height) = self.popup_get_list_rect()?;
        if self.expanded {
            return Some((x, y, width, height));
        }
        let limit = (self.size().height as i32 - height as i32).max(0);
        Some((x, y.min(limit), width, height))
    }

    fn popup_expand_list(&mut self) {
        let Some((_, y, _, _)) = self.popup_get_list_rect() else {
            self.pack();
            return;
        };

        let full = POPUP_ROWS + 2;
        let needed = (y + full as i32 - self.size().height as i32).max(0) as u32;

        if needed == 0 {
            self.pack();
            return;
        }

        let size = self.size();
        let expanded = Size::new(size.width, size.height + needed);
        self.expand(expanded, expanded);
    }

    fn popup_paint_list(&self, surface: &mut Surface, theme: &Theme) {
        let Some(popup) = self.popup.as_ref() else {
            return;
        };
        let Some((x, y, width, height)) = self.popup_get_paint_rect() else {
            return;
        };
        let Some(list) = self.lists.get(popup.list) else {
            return;
        };

        let y = y + self.expanded_offset;
        let normal = theme.menu.text.normal;
        let selected = theme.menu.text.pressed_or_selected;
        let frame = Rect::with_size(x, y, width as u16, height as u16);

        surface.fill_rect(frame, Character::with_attributes(' ', normal));
        surface.draw_rect(frame, LineType::Single, normal);

        let inner = width - 2;
        let visible = (popup.matches.len() as u32 - popup.first).min(POPUP_ROWS);

        for row in 0..visible {
            let index = popup.first + row;
            let Some(&item) = popup.matches.get(index as usize) else {
                continue;
            };
            let Some(value) = list.item(item) else {
                continue;
            };

            let attr = if index == popup.index { selected } else { normal };
            let line = y + 1 + row as i32;

            surface.fill_horizontal_line_with_size(x + 1, line, inner, Character::with_attributes(' ', attr));

            let format = TextFormatBuilder::new()
                .position(x + 2, line)
                .attribute(attr)
                .align(TextAlignment::Left)
                .wrap_type(WrapType::SingleLineWrap((inner - 2) as u16))
                .build();
            surface.write_text(value, &format);
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
        if span_type.contains(SpanType::Quote) {
            foreground = Color::Gray;
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
        }

        CharAttribute::new(foreground, background, flags)
    }

    fn code_block_attr(background: Color) -> CharAttribute {
        CharAttribute::new(Color::Silver, background, CharFlags::None)
    }

    fn quote_bar_attr(background: Color) -> CharAttribute {
        CharAttribute::new(Color::Gray, background, CharFlags::None)
    }

    fn span_width(bytes: &[u8], span: &Span) -> u32 {
        let mut width = 0;
        let mut visible = 0;
        let mut i = span.start as usize;

        while i < span.end as usize {
            let len = Parser::get_char_len(bytes[i]);
            let ch = Parser::get_char(bytes, i, len);

            if ch != '\n' {
                width += Parser::get_char_width(len) as u32;
                if !ch.is_whitespace() {
                    visible = width;
                }
            }

            i += len;
        }

        visible
    }

    fn paint_code_blocks(bytes: &[u8], spans: &[Span], surface: &mut Surface, background: Color, first_row: u32) {
        let height = surface.size().height as i32;
        let attr = Self::code_block_attr(background);

        let mut index = 0;
        while index < spans.len() {
            if !spans[index].span_type.contains(SpanType::CodeBlock) {
                index += 1;
                continue;
            }

            let start = index;
            while index < spans.len() && spans[index].span_type.contains(SpanType::CodeBlock) {
                index += 1;
            }

            let mut top = u32::MAX;
            let mut bottom = 0;
            let mut left = u32::MAX;
            let mut right = 0;

            for span in &spans[start..index] {
                top = top.min(span.y_pos);
                bottom = bottom.max(span.y_pos);
                left = left.min(span.x_pos);
                right = right.max(span.x_pos + Self::span_width(bytes, span));
            }

            if top == 0 || left == 0 || right <= left {
                continue;
            }

            let busy = spans.iter().any(|span| span.y_pos == top - 1 || span.y_pos == bottom + 1);
            if busy {
                continue;
            }

            let outer_top = top as i32 - first_row as i32 - 1;
            let outer_bottom = bottom as i32 - first_row as i32 + 1;
            if outer_bottom < 0 || outer_top >= height {
                continue;
            }

            surface.draw_rect(Rect::new(left as i32 - 1, outer_top, right as i32, outer_bottom), LineType::Single, attr);
        }
    }

    fn paint_normal(text: &str, spans: &[Span], surface: &mut Surface, foreground: Color, background: Color, first_row: u32) {
        let bytes = text.as_bytes();
        let size = surface.size();
        let height = size.height as i32;

        for span in spans {
            if !span.span_type.contains(SpanType::Quote) {
                continue;
            }

            let y = span.y_pos as i32 - first_row as i32;
            if y < 0 || y >= height {
                continue;
            }

            surface.write_char(0, y, Character::with_attributes(QUOTE_BAR, Self::quote_bar_attr(background)));
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
                let ch = if bullet { BULLET.into() } else { Parser::get_char(bytes, i, len) };

                surface.write_char(x, y, Character::with_attributes(ch, attr));

                x += Parser::get_char_width(len);
                i += len;
            }
        }

        Self::paint_code_blocks(bytes, spans, surface, background, first_row);
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

            let y = span.y_pos as i32 - self.first_row as i32 + self.expanded_offset;
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
                        BULLET.into()
                    } else if quote_mark {
                        QUOTE_BAR.into()
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
        self.popup_match_items();
    }

    fn insert_text(&mut self, added: &str) {
        self.popup_close_list();
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
        self.popup_match_items();
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
        self.popup_close_list();
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
        surface.draw_surface(0, self.expanded_offset, &self.surface);
        self.paint_selection(surface, theme);
        self.popup_paint_list(surface, theme);

        if self.has_focus() {
            surface.set_cursor(self.cursor_x as i32, self.cursor_y as i32 - self.first_row as i32 + self.expanded_offset);
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

        if self.popup.is_some() {
            match key.value() {
                key!("Escape") => {
                    self.popup_close_list();
                    return EventProcessStatus::Processed;
                }
                key!("Up") => {
                    self.popup_move_selection(-1);
                    return EventProcessStatus::Processed;
                }
                key!("Down") => {
                    self.popup_move_selection(1);
                    return EventProcessStatus::Processed;
                }
                key!("Enter") | key!("Tab") => {
                    self.popup_insert_item();
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
        }

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
            let trigger = self.popup.is_none() && self.popup_find_list(character).is_some();
            self.insert(character);

            if trigger {
                let start = self.cursor_offset - character.len_utf8() as u32;
                if self.popup_is_list_start(start) {
                    self.popup_open_list(character, start);
                }
            }
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

impl OnExpand for MarkdownComposer {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnBottom => {
                self.expanded = true;
                self.expanded_offset = 0;
            }
            ExpandedDirection::OnTop => {
                self.expanded = false;
                self.expanded_offset = 0;
                self.pack();
            }
        }
    }

    fn on_pack(&mut self) {
        self.expanded = false;
        self.expanded_offset = 0;
    }
}

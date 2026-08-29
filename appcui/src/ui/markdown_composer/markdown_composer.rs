use crate::graphics::SpecialChar;
use crate::prelude::*;
use crate::ui::markdown_composer::emoji::EMOJIS;
use crate::ui::markdown_composer::parser::{Span, SpanType};
use crate::ui::markdown_composer::Flags;
use crate::ui::markdown_composer::Parser;
use crate::ui::markdown_composer::{List, ListFlags};

const WHEEL_ROWS: u32 = 3;
const BULLET: SpecialChar = SpecialChar::CircleFilled;
const QUOTE_BAR: SpecialChar = SpecialChar::BoxVerticalSingleLine;
const POPUP_ROWS: u32 = 4;
const POPUP_MIN_HEIGHT: u32 = 3;
const POPUP_MIN_WIDTH: u32 = 12;
const POPUP_MAX_WIDTH: u32 = 40;

struct Popup {
    list: usize,
    start: u32,
    matches: Vec<u32>,
    index: u32,
    first: u32,
}

impl Popup {
    fn scroll_to_index(&mut self) {
        if self.index < self.first {
            self.first = self.index;
        } else if self.index >= self.first + POPUP_ROWS {
            self.first = self.index - POPUP_ROWS + 1;
        }
    }
}

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed+OnExpand+OnFocus, internal=true)]
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
    packed_origin: i32,
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
            packed_origin: 0,
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
            packed_origin: 0,
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

    pub fn add_list(&mut self, trigger: char, items: &[&str], flags: ListFlags) {
        self.popup_close_list();
        let list = List::with_items(trigger, items, flags);
        match self.lists.iter().position(|item| item.trigger() == trigger) {
            Some(index) => self.lists[index] = list,
            None => self.lists.push(list),
        }
    }

    pub fn add_list_with_values(&mut self, trigger: char, items: &[(&str, &str)], flags: ListFlags) {
        self.popup_close_list();
        let list = List::with_values(trigger, items, flags);
        match self.lists.iter().position(|item| item.trigger() == trigger) {
            Some(index) => self.lists[index] = list,
            None => self.lists.push(list),
        }
    }

    pub fn add_emoji_list(&mut self, trigger: char) {
        self.add_list_with_values(trigger, EMOJIS, ListFlags::RemoveTrigger);
    }

    pub fn list(&self, trigger: char) -> Option<&[String]> {
        self.lists.iter().find(|list| list.trigger() == trigger).map(|list| list.items())
    }

    pub fn remove_list(&mut self, trigger: char) -> bool {
        self.popup_close_list();
        match self.lists.iter().position(|list| list.trigger() == trigger) {
            Some(index) => {
                self.lists.remove(index);
                true
            }
            None => false,
        }
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
            log!("POPUP", "open: nu exista lista pentru declansatorul '{}'", trigger);
            return;
        };

        log!(
            "POPUP",
            "open: trigger='{}' lista={} start={} cursor={} size={}x{}",
            trigger,
            list,
            start,
            self.cursor_offset,
            self.size().width,
            self.size().height
        );

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
        log!(
            "POPUP",
            "close: era_deschis={} expanded={} offset={}",
            self.popup.is_some(),
            self.expanded,
            self.expanded_offset
        );
        self.popup = None;
        self.pack();
    }

    fn popup_match_items(&mut self) {
        let Some((list, start)) = self.popup.as_ref().map(|popup| (popup.list, popup.start)) else {
            return;
        };

        let from = start as usize + self.lists[list].trigger().len_utf8();
        let to = self.cursor_offset as usize;

        if from > to || to > self.text.len() || self.text[from..to].contains('\n') {
            log!(
                "POPUP",
                "match: filtru invalid -> inchid (from={} to={} len={})",
                from,
                to,
                self.text.len()
            );
            self.popup_close_list();
            return;
        }

        let filter = self.text[from..to].to_lowercase();
        let mut matches = Vec::new();

        for index in 0..self.lists[list].len() {
            let matched = match self.lists[list].item(index) {
                Some(item) => filter.is_empty() || item.to_lowercase().contains(&filter),
                None => false,
            };
            if matched {
                matches.push(index);
            }
        }

        if matches.is_empty() {
            log!("POPUP", "match: filtru='{}' -> 0 potriviri, inchid", filter);
            self.popup_close_list();
            return;
        }

        if let Some(popup) = self.popup.as_mut() {
            popup.matches = matches;
            popup.index = popup.index.min(popup.matches.len() as u32 - 1);
            popup.scroll_to_index();
            log!(
                "POPUP",
                "match: filtru='{}' potriviri={} index={} first={}",
                filter,
                popup.matches.len(),
                popup.index,
                popup.first
            );
        }

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

        popup.index = (popup.index as i32).saturating_add(delta).clamp(0, count - 1) as u32;
        popup.scroll_to_index();
        log!("POPUP", "move: delta={} index={} first={} din {}", delta, popup.index, popup.first, count);
    }

    fn popup_insert_item(&mut self) {
        let Some(popup) = self.popup.as_ref() else {
            return;
        };

        let start = popup.start as usize;

        let Some(&item) = popup.matches.get(popup.index as usize) else {
            self.popup_close_list();
            return;
        };
        let list = &self.lists[popup.list];
        let Some(value) = list.value(item) else {
            self.popup_close_list();
            return;
        };

        let mut replacement = String::new();
        if !list.flags().contains(ListFlags::RemoveTrigger) {
            replacement.push(list.trigger());
        }
        replacement.push_str(value);

        self.popup_close_list();

        let end = (self.cursor_offset as usize).min(self.text.len()).max(start);
        log!("POPUP", "insert: '{}' peste [{}..{}]", replacement, start, end);
        self.text.replace_range(start..end, &replacement);
        self.cursor_offset = (start + replacement.len()) as u32;
        self.anchor = None;
        self.update_surface();
    }

    fn popup_get_trigger_row(&self) -> i32 {
        let Some(popup) = self.popup.as_ref() else {
            return 0;
        };
        let (_, trigger_y) = self.parser.get_position_from_offset(&self.text, popup.start);
        trigger_y as i32 - self.first_row as i32
    }

    fn popup_get_list_size(&self) -> Option<(u32, u32)> {
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
        Some((width, height))
    }

    fn popup_fit(row: i32, bottom: i32, height: u32) -> Option<(i32, u32)> {
        let below = row + 1;
        let space_below = if below >= 0 { bottom - below } else { 0 };
        let space_above = row.min(bottom);

        log!(
            "POPUP-FIT",
            "row={} bottom={} height={} loc_jos={} loc_sus={}",
            row,
            bottom,
            height,
            space_below,
            space_above
        );

        if height as i32 <= space_below {
            log!("POPUP-FIT", "-> intreaga JOS la y={}", below);
            return Some((below, height));
        }
        if height as i32 <= space_above {
            log!("POPUP-FIT", "-> intreaga SUS la y={}", row - height as i32);
            return Some((row - height as i32, height));
        }

        if space_below >= space_above && space_below >= POPUP_MIN_HEIGHT as i32 {
            log!("POPUP-FIT", "-> micsorata JOS la y={} h={}", below, space_below);
            return Some((below, space_below as u32));
        }
        if space_above >= POPUP_MIN_HEIGHT as i32 {
            log!("POPUP-FIT", "-> micsorata SUS la y={} h={}", row - space_above, space_above);
            return Some((row - space_above, space_above as u32));
        }

        let height = height.min(bottom.max(0) as u32);
        if height < POPUP_MIN_HEIGHT {
            log!("POPUP-FIT", "-> NU INCAPE deloc (bottom={})", bottom);
            return None;
        }
        let y = below.clamp(0, bottom - height as i32);
        log!("POPUP-FIT", "-> ULTIMA SOLUTIE, peste declansator, y={} h={}", y, height);
        Some((y, height))
    }

    fn popup_get_list_rect(&self) -> Option<Rect> {
        let popup = self.popup.as_ref()?;
        let (width, height) = self.popup_get_list_size()?;

        let size = self.size();
        let (trigger_x, _) = self.parser.get_position_from_offset(&self.text, popup.start);
        let x = (trigger_x as i32).clamp(0, (size.width as i32 - width as i32).max(0));
        let row = self.popup_get_trigger_row() + self.expanded_offset;

        let bottom = if self.expanded {
            self.expanded_size().height as i32
        } else {
            size.height as i32
        };

        log!(
            "POPUP-RECT",
            "trigger_row={} offset={} row_banda={} expanded={} bottom={} h_control={}",
            self.popup_get_trigger_row(),
            self.expanded_offset,
            row,
            self.expanded,
            bottom,
            size.height
        );

        let (y, height) = Self::popup_fit(row, bottom, height)?;
        log!("POPUP-RECT", "=> x={} y={} w={} h={}", x, y, width, height);
        Some(Rect::with_size(x, y, width as u16, height as u16))
    }

    fn popup_fits_inside(&self) -> bool {
        let Some((_, height)) = self.popup_get_list_size() else {
            return true;
        };

        let size = self.size();
        let row = self.popup_get_trigger_row();
        let below = row + 1;

        let fits_below = below >= 0 && below + height as i32 <= size.height as i32;
        let fits_above = row - height as i32 >= 0;

        fits_below || fits_above
    }

    fn popup_expand_list(&mut self) {
        let Some((_, height)) = self.popup_get_list_size() else {
            self.pack();
            return;
        };

        let size = self.size();
        let row = self.popup_get_trigger_row();
        let below = row + 1;

        log!(
            "POPUP-EXPAND",
            "row={} below={} h_lista={} h_control={} incape_in_control={}",
            row,
            below,
            height,
            size.height,
            self.popup_fits_inside()
        );

        if self.popup_fits_inside() {
            log!("POPUP-EXPAND", "-> incape in control, pack()");
            self.pack();
            return;
        }

        let least = (below + POPUP_MIN_HEIGHT as i32).max(size.height as i32 + 1);
        let wanted = POPUP_ROWS as i32 + 2;
        let full = (below + wanted).max(size.height as i32 + wanted - row).max(least);

        let minimum = Size::new(size.width, least as u32);
        let prefered = Size::new(size.width, full as u32);
        self.packed_origin = self.screen_origin.y;
        log!(
            "POPUP-EXPAND",
            "-> expand(min={}x{}, pref={}x{}) focus={} deja_expandat={}",
            minimum.width,
            minimum.height,
            prefered.width,
            prefered.height,
            self.has_focus(),
            self.expanded
        );
        self.expand(minimum, prefered);
    }

    fn popup_get_first_visible(&self, rows: u32) -> u32 {
        let Some(popup) = self.popup.as_ref() else {
            return 0;
        };
        if rows > 0 && popup.index >= popup.first + rows {
            popup.index - rows + 1
        } else {
            popup.first
        }
    }

    fn popup_contains(&self, x: i32, y: i32) -> bool {
        match self.popup_get_list_rect() {
            Some(frame) => frame.contains(Point::new(x, y)),
            None => false,
        }
    }

    fn popup_get_item_at(&self, x: i32, y: i32) -> Option<u32> {
        let popup = self.popup.as_ref()?;
        let frame = self.popup_get_list_rect()?;

        if !frame.contains(Point::new(x, y)) {
            return None;
        }

        let rows = frame.height() - 2;
        let row = y - frame.top() - 1;
        if row < 0 || row >= rows as i32 {
            return None;
        }

        let index = self.popup_get_first_visible(rows) + row as u32;
        if (index as usize) < popup.matches.len() {
            Some(index)
        } else {
            None
        }
    }

    fn popup_paint_list(&self, surface: &mut Surface, theme: &Theme) {
        let Some(popup) = self.popup.as_ref() else {
            return;
        };
        let Some(frame) = self.popup_get_list_rect() else {
            return;
        };
        let Some(list) = self.lists.get(popup.list) else {
            return;
        };

        let normal = theme.menu.text.normal;
        let selected = theme.menu.text.pressed_or_selected;
        let x = frame.left();
        let y = frame.top();

        surface.fill_rect(frame, Character::with_attributes(' ', normal));
        surface.draw_rect(frame, LineType::Single, normal);

        let inner = frame.width() - 2;
        let rows = frame.height() - 2;

        let first = self.popup_get_first_visible(rows);
        let visible = (popup.matches.len() as u32 - first).min(rows);
        log!(
            "POPUP-PAINT",
            "frame=({}, {}, {}x{}) first={} index={} vizibile={}",
            frame.left(),
            frame.top(),
            frame.width(),
            frame.height(),
            first,
            popup.index,
            visible
        );

        let mut format = TextFormatBuilder::new()
            .position(x + 2, y + 1)
            .attribute(normal)
            .align(TextAlignment::Left)
            .wrap_type(WrapType::SingleLineWrap((inner - 2) as u16))
            .build();

        for row in 0..visible {
            let index = first + row;
            let Some(&item) = popup.matches.get(index as usize) else {
                continue;
            };
            let Some(value) = list.item(item) else {
                continue;
            };

            let line = y + 1 + row as i32;
            let attr = if index == popup.index {
                surface.fill_horizontal_line_with_size(x + 1, line, inner, Character::with_attributes(' ', selected));
                selected
            } else {
                normal
            };

            format.set_position(x + 2, line);
            format.set_attribute(attr);
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

    fn next_word(&self, offset: u32) -> u32 {
        let bytes = self.text.as_bytes();
        let len = bytes.len();
        let mut i = (offset as usize).min(len);

        if i >= len {
            return len as u32;
        }

        if bytes[i].is_ascii_whitespace() {
            while i < len && bytes[i].is_ascii_whitespace() {
                i = Parser::next_offset(&self.text, i as u32) as usize;
            }
            return i as u32;
        }

        while i < len && !bytes[i].is_ascii_whitespace() {
            i = Parser::next_offset(&self.text, i as u32) as usize;
        }
        while i < len && bytes[i].is_ascii_whitespace() && bytes[i] != b'\n' {
            i = Parser::next_offset(&self.text, i as u32) as usize;
        }

        i as u32
    }

    fn prev_word(&self, offset: u32) -> u32 {
        let bytes = self.text.as_bytes();
        let mut i = (offset as usize).min(bytes.len());

        while i > 0 {
            let previous = Parser::prev_offset(&self.text, i as u32) as usize;
            if !bytes[previous].is_ascii_whitespace() {
                break;
            }
            i = previous;
        }
        while i > 0 {
            let previous = Parser::prev_offset(&self.text, i as u32) as usize;
            if bytes[previous].is_ascii_whitespace() {
                break;
            }
            i = previous;
        }

        i as u32
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

    fn delete_previous_word(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let start = self.prev_word(self.cursor_offset);
        if start == self.cursor_offset {
            return;
        }
        self.text.replace_range(start as usize..self.cursor_offset as usize, "");
        self.cursor_offset = start;
        self.update_surface();
        self.popup_match_items();
    }

    fn delete_next_word(&mut self) {
        if self.remove_selection() {
            self.update_surface();
            return;
        }
        let end = self.next_word(self.cursor_offset);
        if end == self.cursor_offset {
            return;
        }
        self.text.replace_range(self.cursor_offset as usize..end as usize, "");
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
        let y = (y - self.expanded_offset).clamp(0, height - 1) as u32 + self.first_row;
        self.parser.get_offset_from_position(&self.text, x.max(0) as u32, y)
    }

    fn scroll_towards(&mut self, y: i32) {
        let height = self.size().height.max(1) as i32;
        let y = y - self.expanded_offset;

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
        if self.expanded {
            log!(
                "POPUP-ORIGIN",
                "paint: origin.y={} offset={} => randul 0 al textului pe ecran = {}",
                self.screen_origin.y,
                self.expanded_offset,
                self.screen_origin.y + self.expanded_offset
            );
        }
        if self.expanded {
            let bottom = self.screen_origin.y + self.expanded_offset + self.size().height as i32 - 1;
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                bottom.max(self.screen_clip.bottom),
            );
            surface.reset_clip();
        }

        surface.draw_surface(0, self.expanded_offset, &self.surface);
        self.paint_selection(surface, theme);
        self.popup_paint_list(surface, theme);

        if self.expanded {
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                self.screen_clip.bottom,
            );
            surface.reset_clip();
        }

        if self.has_focus() {
            surface.set_cursor(self.cursor_x as i32, self.cursor_y as i32 - self.first_row as i32 + self.expanded_offset);
        } else {
            surface.hide_cursor();
        }
    }
}

impl OnResize for MarkdownComposer {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.popup_close_list();
        self.update_surface();
    }
}

impl OnMouseEvent for MarkdownComposer {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(data) => {
                if self.popup_contains(data.x, data.y) {
                    log!("POPUP-MOUSE", "click in lista la ({}, {})", data.x, data.y);
                    if let Some(index) = self.popup_get_item_at(data.x, data.y) {
                        if let Some(popup) = self.popup.as_mut() {
                            popup.index = index;
                        }
                        self.popup_insert_item();
                    }
                    return EventProcessStatus::Processed;
                }
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, false);
                self.anchor = Some(offset);
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) => {
                if self.popup_contains(data.x, data.y) {
                    return EventProcessStatus::Processed;
                }
                let offset = self.offset_at_screen(data.x, data.y);
                let (start, end) = self.word_at(offset);
                self.anchor = Some(start);
                self.move_to(end, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.popup_contains(data.x, data.y) {
                    return EventProcessStatus::Processed;
                }
                self.scroll_towards(data.y);
                let offset = self.offset_at_screen(data.x, data.y);
                self.move_to(offset, true);
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let Some(index) = self.popup_get_item_at(point.x, point.y) else {
                    return EventProcessStatus::Ignored;
                };
                let Some(popup) = self.popup.as_mut() else {
                    return EventProcessStatus::Ignored;
                };
                if popup.index == index {
                    return EventProcessStatus::Ignored;
                }
                log!("POPUP-MOUSE", "hover ({}, {}) -> index={}", point.x, point.y, index);
                popup.index = index;
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(direction) => {
                if self.popup.is_some() {
                    match direction {
                        MouseWheelDirection::Up => self.popup_move_selection(-1),
                        MouseWheelDirection::Down => self.popup_move_selection(1),
                        _ => return EventProcessStatus::Ignored,
                    }
                    return EventProcessStatus::Processed;
                }
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
                key!("PageUp") => {
                    self.popup_move_selection(-(POPUP_ROWS as i32));
                    return EventProcessStatus::Processed;
                }
                key!("PageDown") => {
                    self.popup_move_selection(POPUP_ROWS as i32);
                    return EventProcessStatus::Processed;
                }
                key!("Home") => {
                    self.popup_move_selection(i32::MIN);
                    return EventProcessStatus::Processed;
                }
                key!("End") => {
                    self.popup_move_selection(i32::MAX);
                    return EventProcessStatus::Processed;
                }
                key!("Enter") | key!("Tab") | key!("Right") => {
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
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                let offset = self.prev_word(self.cursor_offset);
                self.move_to(offset, select);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                let offset = self.next_word(self.cursor_offset);
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
            key!("Ctrl+PageUp") => {
                let row = self.first_row.saturating_sub(height);
                self.scroll_to(row);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+PageDown") => {
                let row = self.first_row + height;
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
            key!("Ctrl+Backspace") => {
                self.delete_previous_word();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Delete") => {
                self.delete_next_word();
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
        self.expanded = true;
        self.expanded_offset = (self.packed_origin - self.screen_origin.y).max(0);
        let name = match direction {
            ExpandedDirection::OnBottom => "OnBottom",
            ExpandedDirection::OnTop => "OnTop",
        };
        log!(
            "POPUP-EXPAND",
            "on_expand({}): banda={}x{} control={}x{} => offset={}",
            name,
            self.expanded_size().width,
            self.expanded_size().height,
            self.size().width,
            self.size().height,
            self.expanded_offset
        );
        log!(
            "POPUP-ORIGIN",
            "origin_inainte={} origin.y={} clip=[{}..{}] editorul cade la {}..{}",
            self.packed_origin,
            self.screen_origin.y,
            self.screen_clip.top,
            self.screen_clip.bottom,
            self.screen_origin.y + self.expanded_offset,
            self.screen_origin.y + self.expanded_offset + self.size().height as i32 - 1
        );
    }

    fn on_pack(&mut self) {
        self.expanded = false;
        self.expanded_offset = 0;

        if self.popup.is_some() && !self.popup_fits_inside() {
            log!("POPUP-EXPAND", "on_pack(): strans din afara, lista nu mai incape -> inchid");
            self.popup_close_list();
        } else {
            log!("POPUP-EXPAND", "on_pack(): offset -> 0, lista ramane");
        }
    }
}

impl OnFocus for MarkdownComposer {
    fn on_lose_focus(&mut self) {
        self.popup_close_list();
    }
}

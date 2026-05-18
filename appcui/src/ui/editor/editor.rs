use super::events::*;
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
    /// Visual column on screen (post-wrap, post-horizontal-scroll). 0-based from text area left edge.
    column: u32,
    /// Logical document line.
    line: u32,
    /// Visual row within the logical line (0 when wrap off).
    row_in_line: u32,
    /// Absolute char index in document.
    pos: usize,
    /// Sticky visual column for vertical movement. Tracks user intent across short lines.
    sticky_col: u32,
    visible: bool,
}

pub struct Caret {
    pub column: u32,
    pub line: u32,
    pub pos: usize,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Editor {
    document: Document,
    tab_align: i32,
    /// Top logical line in the viewport.
    start_line: u32,
    /// Horizontal scroll in cells. Always 0 when wrap is on.
    horizontal_scroll: u32,
    cursor: Cursor,
    flags: Flags,
    margin: MarginSize,
    view: ViewPort,
    selection: Selection,
    scrollbars: ScrollBars,
}

impl Editor {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut editor = Self {
            base: ControlBase::with_status_flags(
                layout,
                StatusFlags::Visible
                    | StatusFlags::Enabled
                    | StatusFlags::AcceptInput
                    | if flags.contains(Flags::ScrollBars) {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            document: Document::new(""),
            tab_align: 4,
            start_line: 0,
            horizontal_scroll: 0,
            cursor: Cursor::default(),
            flags,
            margin: MarginSize::default(),
            view: ViewPort::new(16),
            selection: Selection::NONE,
            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
        };
        editor.set_text(text);
        editor.update_margin();
        editor
    }

    pub fn set_text(&mut self, text: &str) {
        self.document = Document::new(text);
        self.view.reset();
        self.goto_position(0, false);
        self.update_view();
    }

    pub fn set_word_wrap(&mut self, enabled: bool) {
        if self.view.is_word_wrap_enabled() != enabled {
            self.view.set_wrap_enabled(enabled);
            self.cursor.sticky_col = 0;
            if enabled {
                self.horizontal_scroll = 0;
            }
            self.update_view();
            // Re-anchor cursor to keep it visible after the layout change.
            let pos = self.cursor.pos;
            self.goto_position(pos, false);
        }
    }

    /// Returns the current caret position
    #[inline(always)]
    pub fn caret(&self) -> Caret {
        Caret {
            column: self.cursor.column,
            line: self.cursor.line,
            pos: self.cursor.pos,
        }
    }
    /// Returns the number of lines of the document that is opened in the editor
    #[inline(always)]
    pub fn lines_count(&self) -> usize {
        self.document.lines_count()
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
            self.margin.width = width + 2;
        } else {
            self.margin.line_number_x = 0;
            self.margin.width = 0;
        }
        self.margin.visible = width > 0;
    }

    fn ensure_line_is_cached(&mut self, line_index: u32) {
        if !self.view.contains_line(line_index) {
            if line_index < self.view.first_line() {
                self.start_line = line_index;
            } else if line_index > self.view.last_line() {
                let h = self.view.size().height.saturating_sub(1);
                self.start_line = line_index.saturating_sub(h);
            }
            self.view.set_start_visual_row(0);
            self.update_view();
        }
    }

    /// Scroll vertically so that (line_index, row_in_line) is visible.
    /// Only relevant when wrap is on; otherwise `ensure_line_is_cached` is enough.
    fn ensure_visual_row_visible(&mut self, line_index: u32, row_in_line: u32) {
        if !self.view.is_word_wrap_enabled() {
            return;
        }
        // First make sure the logical line is cached.
        self.ensure_line_is_cached(line_index);

        let height = self.view.size().height;
        if height == 0 {
            return;
        }

        // Compute the screen y for this (line, row_in_line) given current scroll.
        let Some(screen_y) = self.visual_screen_y(line_index, row_in_line) else {
            return;
        };

        if screen_y < 0 {
            // Target is above the top: scroll up by adjusting start_visual_row, then start_line.
            let mut deficit = (-screen_y) as u32;
            let mut svr = self.view.start_visual_row();
            // First consume start_visual_row of the top line.
            if svr >= deficit {
                self.view.set_start_visual_row(svr - deficit);
                return;
            }
            deficit -= svr;
            // Then walk upward through logical lines.
            let mut ln = self.start_line;
            while deficit > 0 && ln > 0 {
                ln -= 1;
                let rows = self.line_visual_row_count(ln);
                if rows >= deficit {
                    svr = rows - deficit;
                    deficit = 0;
                } else {
                    deficit -= rows;
                    svr = 0;
                }
            }
            self.start_line = ln;
            self.update_view();
            self.view.set_start_visual_row(svr);
        } else if screen_y >= height as i32 {
            // Target is below the bottom: scroll down by one visual row at a time
            // until target_screen_y < height. Cheap because we usually need few steps.
            let mut overflow = (screen_y as u32) - height + 1;
            while overflow > 0 {
                let top_rows = self.line_visual_row_count(self.start_line);
                let svr = self.view.start_visual_row();
                if svr + 1 < top_rows {
                    self.view.set_start_visual_row(svr + 1);
                } else {
                    self.start_line += 1;
                    self.view.set_start_visual_row(0);
                    self.update_view();
                }
                overflow -= 1;
            }
        }
    }

    fn line_visual_row_count(&self, line_number: u32) -> u32 {
        // If the line is cached, use the cached value; otherwise assume 1.
        // Lines we scroll through are cached as we go, so this stays consistent.
        self.view
            .visual_pos(line_number, super::LineCharIndex::new(0))
            .map(|_| {
                // We have the line cached; ask the viewport.
                self.view.cached_visual_row_count(line_number).unwrap_or(1)
            })
            .unwrap_or(1)
    }

    /// Screen y (relative to viewport top) for a (line, row_in_line). May be negative or >= height.
    fn visual_screen_y(&self, line_number: u32, row_in_line: u32) -> Option<i32> {
        if line_number < self.start_line {
            // Not cached; the caller (ensure_visual_row_visible) handles this.
            // Return a negative sentinel based on a rough estimate.
            return Some(-((self.start_line - line_number) as i32));
        }
        let mut y: i32 = -(self.view.start_visual_row() as i32);
        let mut ln = self.start_line;
        while ln < line_number {
            y += self.line_visual_row_count(ln) as i32;
            ln += 1;
        }
        Some(y + row_in_line as i32)
    }

    fn goto_position(&mut self, position: usize, select: bool) {
        let new_pos = position.min(self.document.chars_count());
        if select {
            self.selection.update(self.cursor.pos, new_pos);
        } else {
            self.selection.clear();
        }
        let pos_info = self.document.char_to_pos_info(new_pos);
        self.ensure_line_is_cached(pos_info.line_index);

        self.cursor.pos = new_pos;
        self.cursor.line = pos_info.line_index;

        if !self.view.contains_line(pos_info.line_index) {
            self.cursor.visible = false;
            return;
        }

        if self.view.is_word_wrap_enabled() {
            // Wrap mode: compute visual position, scroll partway into line if needed.
            let Some(vp) = self.view.visual_pos(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor.visible = false;
                return;
            };
            self.cursor.row_in_line = vp.row;
            self.ensure_visual_row_visible(pos_info.line_index, vp.row);
            // Recompute screen y after scroll adjustments.
            self.cursor.column = vp.col_in_row;
            self.cursor.visible = true;
        } else {
            // No wrap: horizontal scroll, single row per line.
            self.cursor.row_in_line = 0;
            let Some(x_offset) = self.view.unwrapped_col(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor.column = 0;
                self.cursor.visible = false;
                return;
            };
            let w = self.view.size().width;
            if w > 0 {
                if x_offset < self.horizontal_scroll {
                    self.horizontal_scroll = x_offset;
                }
                if x_offset >= self.horizontal_scroll + w {
                    self.horizontal_scroll = (x_offset + 1).saturating_sub(w);
                }
            }
            self.cursor.column = x_offset.saturating_sub(self.horizontal_scroll);
            self.cursor.visible = true;
        }

        // Update sticky column on any horizontal-aware operation.
        // Callers that want to *preserve* sticky (vertical movement) save+restore around the call.
        self.cursor.sticky_col = self.cursor.column + self.horizontal_scroll;
        // update the scrollbars
        self.scrollbars
            .update(self.view.max_columns() as u64, self.document.lines_count() as u64, self.size());
        self.scrollbars
            .set_indexes(self.cursor.column as u64 + self.horizontal_scroll as u64, self.start_line as u64);
    }

    /// Refresh on-screen cursor state after viewport-only scroll (Ctrl+Up/Down).
    /// Document position is unchanged; visibility and screen column are updated.
    fn sync_cursor_visibility(&mut self) {
        if !self.view.contains_line(self.cursor.line) {
            self.cursor.visible = false;
            return;
        }

        let height = self.view.size().height as i32;
        let screen_y = self.cursor_screen_y();
        if screen_y < 0 || screen_y >= height {
            self.cursor.visible = false;
            return;
        }

        let pos_info = self.document.char_to_pos_info(self.cursor.pos);

        if self.view.is_word_wrap_enabled() {
            let Some(vp) = self.view.visual_pos(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor.visible = false;
                return;
            };
            self.cursor.row_in_line = vp.row;
            self.cursor.column = vp.col_in_row;
            self.cursor.visible = true;
        } else {
            self.cursor.row_in_line = 0;
            let Some(x_offset) = self.view.unwrapped_col(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor.visible = false;
                return;
            };
            let w = self.view.size().width;
            if w > 0 && (x_offset < self.horizontal_scroll || x_offset >= self.horizontal_scroll + w) {
                self.cursor.visible = false;
                return;
            }
            self.cursor.column = x_offset.saturating_sub(self.horizontal_scroll);
            self.cursor.visible = true;
        }
    }

    /// Move the cursor by N visual rows. Preserves sticky column across short lines.
    fn move_visual_rows(&mut self, delta: i32, select: bool) {
        let lines_count = self.document.lines_count();
        if lines_count == 0 {
            return;
        }

        let sticky = self.cursor.sticky_col;

        if self.view.is_word_wrap_enabled() {
            self.move_visual_rows_wrapped(delta, sticky, select);
        } else {
            // No wrap: visual rows == logical lines.
            let new_line = (self.cursor.line as i32 + delta).clamp(0, lines_count as i32 - 1) as u32;
            self.ensure_line_is_cached(new_line);
            // Use sticky column (in unwrapped coordinates) to pick the target char.
            let target_col = sticky;
            let lci = self
                .view
                .unwrapped_col_to_line_char_index(new_line, target_col)
                .unwrap_or_else(|| super::LineCharIndex::new(0));
            let new_pos = self.document.line_to_char(new_line) + lci.get() as usize;
            self.goto_position(new_pos, select);
            // Restore sticky (goto_position overwrote it).
            self.cursor.sticky_col = sticky;
        }
    }

    fn move_visual_rows_wrapped(&mut self, delta: i32, sticky: u32, select: bool) {
        let mut remaining = delta;
        let mut line = self.cursor.line;
        let mut row = self.cursor.row_in_line;
        let lines_count = self.document.lines_count() as u32;

        while remaining != 0 {
            if remaining > 0 {
                self.ensure_line_is_cached(line);
                let rows = self.line_visual_row_count(line);
                if row + 1 < rows {
                    row += 1;
                    remaining -= 1;
                } else if line + 1 < lines_count {
                    line += 1;
                    row = 0;
                    remaining -= 1;
                } else {
                    // At end of document.
                    break;
                }
            } else {
                if row > 0 {
                    row -= 1;
                    remaining += 1;
                } else if line > 0 {
                    line -= 1;
                    self.ensure_line_is_cached(line);
                    row = self.line_visual_row_count(line).saturating_sub(1);
                    remaining += 1;
                } else {
                    break;
                }
            }
        }

        self.ensure_line_is_cached(line);
        let lci = self
            .view
            .visual_to_line_char_index(line, row, sticky)
            .unwrap_or_else(|| super::LineCharIndex::new(0));
        let new_pos = self.document.line_to_char(line) + lci.get() as usize;
        self.goto_position(new_pos, select);
        self.cursor.sticky_col = sticky;
    }

    fn move_to_next_word(&mut self, select: bool) {
        let mut iter = self.document.chars_iter(self.cursor.pos);
        let Some(first) = iter.next() else {
            return;
        };
        let char_class = CharClass::from(first);
        let mut pos = self.cursor.pos + 1;
        let mut new_char_class = char_class;
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
        // if scrollbars are enabled, update the max columns
        if self.flags.contains(Flags::ScrollBars) {
            self.view.update_max_columns();
        }
    }
    fn notify_document_changed(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Editor(EventData {
                evtype: EditorEventsType::OnDocumentChanged,
            }),
        });
    }
}

impl OnPaint for Editor {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags.contains(Flags::ScrollBars)) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        let x = if self.margin.visible { self.margin.width as i32 } else { 0 };
        let w = self.view.size().width;
        let h = self.view.size().height as i32;
        let wrap_enabled = self.view.is_word_wrap_enabled();
        surface.clear(Character::with_attributes(' ', theme.editor.normal));

        let mut y = 0i32;
        for row in self.view.visual_rows() {
            if y >= h {
                break;
            }
            if self.margin.visible {
                if row.is_first_row {
                    self.paint_line_number(surface, 0, y, row.line.line_number() + 1, theme.editor.inactive);
                }
            }

            let chars = if wrap_enabled {
                row.line.visual_row_chars(row.row_in_line, w)
            } else {
                row.line.unwrapped_chars(self.horizontal_scroll, self.horizontal_scroll + w)
            };
            surface.write_chars(x, y, chars);
            y += 1;
        }

        if self.has_focus() && self.cursor.visible {
            // TODO: paint selection
            let cursor_y = self.cursor_screen_y();
            surface.set_cursor(self.cursor.column as i32 + x, cursor_y);
        }
    }
}

impl Editor {
    fn cursor_screen_y(&self) -> i32 {
        if !self.view.is_word_wrap_enabled() {
            return self.cursor.line.saturating_sub(self.start_line) as i32;
        }
        // Walk cached logical lines from start_line, summing visual rows.
        let mut y: i32 = -(self.view.start_visual_row() as i32);
        let mut ln = self.start_line;
        while ln < self.cursor.line {
            y += self.view.cached_visual_row_count(ln).unwrap_or(1) as i32;
            ln += 1;
        }
        y + self.cursor.row_in_line as i32
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
                self.move_visual_rows(-1, select);
                EventProcessStatus::Processed
            }
            key!("Down") | key!("Shift+Down") => {
                self.move_visual_rows(1, select);
                EventProcessStatus::Processed
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                self.move_visual_rows(-(self.view.size().height as i32), select);
                EventProcessStatus::Processed
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.move_visual_rows(self.view.size().height as i32, select);
                EventProcessStatus::Processed
            }

            // viewport scroll (cursor stays put)
            key!("Ctrl+Up") => {
                if self.view.is_word_wrap_enabled() {
                    let svr = self.view.start_visual_row();
                    if svr > 0 {
                        self.view.set_start_visual_row(svr - 1);
                    } else if self.start_line > 0 {
                        self.start_line -= 1;
                        self.update_view();
                        let rows = self.view.cached_visual_row_count(self.start_line).unwrap_or(1);
                        self.view.set_start_visual_row(rows.saturating_sub(1));
                    }
                } else {
                    self.start_line = self.start_line.saturating_sub(1);
                    self.update_view();
                }
                self.sync_cursor_visibility();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") => {
                if self.view.is_word_wrap_enabled() {
                    let top_rows = self.view.cached_visual_row_count(self.start_line).unwrap_or(1);
                    let svr = self.view.start_visual_row();
                    if svr + 1 < top_rows {
                        self.view.set_start_visual_row(svr + 1);
                    } else if (self.start_line as usize) + 1 < self.document.lines_count() {
                        self.start_line += 1;
                        self.view.set_start_visual_row(0);
                        self.update_view();
                    }
                } else {
                    let max = self.document.lines_count() as u32;
                    self.start_line = (self.start_line + 1).min(max.saturating_sub(1));
                    self.update_view();
                }
                self.sync_cursor_visibility();
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
                // VSCode-like: Home goes to start of visual row when wrapped.
                if self.view.is_word_wrap_enabled() {
                    let line = self.cursor.line;
                    let row = self.cursor.row_in_line;
                    if let Some(lci) = self.view.visual_to_line_char_index(line, row, 0) {
                        let new_pos = self.document.line_to_char(line) + lci.get() as usize;
                        self.goto_position(new_pos, select);
                    }
                } else {
                    let line = self.cursor.line;
                    self.goto_position(self.document.line_to_char(line), select);
                }
                EventProcessStatus::Processed
            }
            key!("End") | key!("Shift+End") => {
                if self.view.is_word_wrap_enabled() {
                    // End of visual row: pass a huge col_in_row, Line clamps it.
                    let line = self.cursor.line;
                    let row = self.cursor.row_in_line;
                    if let Some(lci) = self.view.visual_to_line_char_index(line, row, u32::MAX) {
                        let new_pos = self.document.line_to_char(line) + lci.get() as usize;
                        self.goto_position(new_pos, select);
                    }
                } else {
                    let line = self.cursor.line;
                    self.goto_position(self.document.line_end_position(line), select);
                }
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

            key!("Ctrl+A") => {
                let end = self.document.chars_count();
                self.goto_position(end, false);
                self.selection.set(0, end);
                EventProcessStatus::Processed
            }

            key!("Alt+Z") => {
                self.set_word_wrap(!self.view.is_word_wrap_enabled());
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
        self.scrollbars
            .resize(self.view.max_columns() as u64, self.document.lines_count() as u64, &self.base);
        if new_size.width <= self.margin.width as u32 {
            self.margin.visible = false;
            self.view.resize(new_size);
        } else {
            self.margin.visible = self.flags.contains(Flags::ShowLineNumbers);
            self.view
                .resize(Size::new(new_size.width.saturating_sub(self.margin.width as u32), new_size.height));
        }
        self.update_view();
        // Re-anchor cursor so column/row_in_line and scroll offsets stay consistent.
        let pos = self.cursor.pos;
        self.goto_position(pos, false);
    }
}

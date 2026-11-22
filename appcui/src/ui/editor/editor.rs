use super::events::*;
use super::{CharClass, Document, Flags, Line, LineCharIndex, Selection, ViewPort};
use crate::prelude::*;
use crate::system::Clipboard;
use std::time::{Duration, Instant};

const WHEEL_SCROLL_ROWS: i32 = 3;
const TRIPLE_CLICK_TIME: Duration = Duration::from_millis(500);
const GUIDE_SEARCH_LINES: u32 = 64;

#[derive(Default)]
struct MarginSize {
    line_number_width: u8,
    line_number_x: u8,
    width: u8,
    visible: bool,
}

#[derive(Copy, Clone)]
struct Cursor {
    /// Absolute char index in document.
    pos: usize,
    /// Sticky visual column for vertical movement. Tracks user intent across short lines.
    sticky_col: u32,
    selection: Selection,
}

impl Cursor {
    const fn new(pos: usize) -> Self {
        Self {
            pos,
            sticky_col: 0,
            selection: Selection::NONE,
        }
    }
}

/// On-screen location of the primary caret (the only one shown by the terminal cursor).
#[derive(Default)]
struct CursorView {
    /// Visual column on screen (post-wrap, post-horizontal-scroll). 0-based from text area left edge.
    column: u32,
    /// Logical document line.
    line: u32,
    /// Visual row within the logical line (0 when wrap off).
    row_in_line: u32,
    visible: bool,
}

#[derive(Copy, Clone)]
enum Motion {
    Left,
    Right,
    Rows(i32),
    WordLeft,
    WordRight,
    RowStart,
    RowEnd,
    DocStart,
    DocEnd,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MouseDrag {
    None,
    Chars,
    Lines { start: usize, end: usize },
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
    cursors: Vec<Cursor>,
    primary: usize,
    cursor_view: CursorView,
    flags: Flags,
    margin: MarginSize,
    view: ViewPort,
    scrollbars: ScrollBars,
    scratch_line: Line,
    mouse_drag: MouseDrag,
    /// Last mouse position during a drag: the wheel event has no coordinates.
    drag_pos: (i32, i32),
    last_double_click: Option<(Instant, i32, i32)>,
    overwrite: bool,
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
            cursors: vec![Cursor::new(0)],
            primary: 0,
            cursor_view: CursorView::default(),
            flags,
            margin: MarginSize::default(),
            view: ViewPort::new(16),
            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            scratch_line: Line::new(64),
            mouse_drag: MouseDrag::None,
            drag_pos: (0, 0),
            last_double_click: None,
            overwrite: false,
        };
        editor.set_text(text);
        editor
    }

    pub fn set_text(&mut self, text: &str) {
        self.document = Document::new(text);
        self.keep_only_primary();
        self.update_margin();
        self.view.reset();
        self.goto_position(0, false);
        self.update_view();
    }

    pub fn set_word_wrap(&mut self, enabled: bool) {
        if self.view.is_word_wrap_enabled() != enabled {
            self.view.set_wrap_enabled(enabled);
            self.primary_cursor_mut().sticky_col = 0;
            if enabled {
                self.horizontal_scroll = 0;
            }
            self.update_view();
            // Re-anchor cursor to keep it visible after the layout change.
            let pos = self.primary_cursor().pos;
            self.goto_position(pos, false);
            self.refresh_sticky_cols();
        }
    }

    /// Returns the current caret position
    #[inline(always)]
    pub fn caret(&self) -> Caret {
        Caret {
            column: self.cursor_view.column,
            line: self.cursor_view.line,
            pos: self.primary_cursor().pos,
        }
    }
    #[inline(always)]
    pub fn carets_count(&self) -> usize {
        self.cursors.len()
    }
    /// Returns the number of lines of the document that is opened in the editor
    #[inline(always)]
    pub fn lines_count(&self) -> usize {
        self.document.lines_count()
    }

    #[inline(always)]
    fn primary_cursor(&self) -> &Cursor {
        &self.cursors[self.primary]
    }

    #[inline(always)]
    fn primary_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[self.primary]
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
            self.margin.width = if self.flags.contains(Flags::ShowFoldMarkers) { 1 } else { 0 };
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
        self.view.cached_visual_row_count(line_number).unwrap_or(1)
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
        let cursor = self.primary_cursor_mut();
        if select {
            cursor.selection.update(cursor.pos, new_pos);
        } else {
            cursor.selection.clear();
        }
        cursor.pos = new_pos;
        self.follow_primary();
    }

    fn follow_primary(&mut self) {
        let pos_info = self.document.char_to_pos_info(self.primary_cursor().pos);
        self.ensure_line_is_cached(pos_info.line_index);

        self.cursor_view.line = pos_info.line_index;

        if !self.view.contains_line(pos_info.line_index) {
            self.cursor_view.visible = false;
            return;
        }

        if self.view.is_word_wrap_enabled() {
            // Wrap mode: compute visual position, scroll partway into line if needed.
            let Some(vp) = self.view.visual_pos(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor_view.visible = false;
                return;
            };
            self.cursor_view.row_in_line = vp.row;
            self.ensure_visual_row_visible(pos_info.line_index, vp.row);
            // Recompute screen y after scroll adjustments.
            self.cursor_view.column = vp.col_in_row;
            self.cursor_view.visible = true;
        } else {
            // No wrap: horizontal scroll, single row per line.
            self.cursor_view.row_in_line = 0;
            let Some(x_offset) = self.view.unwrapped_col(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor_view.column = 0;
                self.cursor_view.visible = false;
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
            self.cursor_view.column = x_offset.saturating_sub(self.horizontal_scroll);
            self.cursor_view.visible = true;
        }

        // Update sticky column on any horizontal-aware operation.
        // Callers that want to *preserve* sticky (vertical movement) save+restore around the call.
        self.primary_cursor_mut().sticky_col = self.cursor_view.column + self.horizontal_scroll;
        // update the scrollbars
        self.scrollbars
            .update(self.view.max_columns() as u64, self.document.lines_count() as u64, self.size());
        self.scrollbars.set_indexes(self.horizontal_scroll as u64, self.start_line as u64);
    }

    /// Refresh on-screen cursor state after viewport-only scroll (Ctrl+Up/Down, mouse wheel).
    /// Document position is unchanged; visibility and screen column are updated.
    fn sync_cursor_visibility(&mut self) {
        if !self.view.contains_line(self.cursor_view.line) {
            self.cursor_view.visible = false;
            return;
        }

        let height = self.view.size().height as i32;
        let screen_y = self.cursor_screen_y();
        if screen_y < 0 || screen_y >= height {
            self.cursor_view.visible = false;
            return;
        }

        let pos_info = self.document.char_to_pos_info(self.primary_cursor().pos);

        if self.view.is_word_wrap_enabled() {
            let Some(vp) = self.view.visual_pos(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor_view.visible = false;
                return;
            };
            self.cursor_view.row_in_line = vp.row;
            self.cursor_view.column = vp.col_in_row;
            self.cursor_view.visible = true;
        } else {
            self.cursor_view.row_in_line = 0;
            let Some(x_offset) = self.view.unwrapped_col(pos_info.line_index, pos_info.line_char_index) else {
                self.cursor_view.visible = false;
                return;
            };
            let w = self.view.size().width;
            if w > 0 && (x_offset < self.horizontal_scroll || x_offset >= self.horizontal_scroll + w) {
                self.cursor_view.visible = false;
                return;
            }
            self.cursor_view.column = x_offset.saturating_sub(self.horizontal_scroll);
            self.cursor_view.visible = true;
        }
    }

    fn line_layout(&mut self, line: u32) -> &Line {
        if let Some(idx) = self.view.line_number_to_index(line) {
            return self.view.line_at(idx);
        }
        let attr = self.theme().editor.normal;
        self.scratch_line.update(line, &self.document, attr, self.tab_align as u32);
        if self.view.is_word_wrap_enabled() {
            self.scratch_line.update_wrap_points(self.view.size().width);
        }
        &self.scratch_line
    }

    fn sticky_col_of(&mut self, pos: usize) -> u32 {
        let info = self.document.char_to_pos_info(pos);
        let wrap = self.view.is_word_wrap_enabled();
        let line = self.line_layout(info.line_index);
        if wrap {
            line.line_char_index_to_visual(info.line_char_index).col_in_row
        } else {
            line.line_char_index_to_unwrapped_col(info.line_char_index)
        }
    }

    fn motion_target(&mut self, pos: usize, sticky: u32, motion: Motion) -> (usize, Option<u32>) {
        match motion {
            Motion::Left => (self.previous_char_pos(pos), None),
            Motion::Right => (self.next_char_pos(pos), None),
            Motion::Rows(delta) => (self.pos_after_rows(pos, sticky, delta), Some(sticky)),
            Motion::WordLeft => (self.previous_word_pos(pos), None),
            Motion::WordRight => (self.next_word_pos(pos), None),
            Motion::RowStart => (self.row_edge_pos(pos, false), None),
            Motion::RowEnd => (self.row_edge_pos(pos, true), None),
            Motion::DocStart => (0, None),
            Motion::DocEnd => (self.document.chars_count(), None),
        }
    }

    fn row_edge_pos(&mut self, pos: usize, end: bool) -> usize {
        let info = self.document.char_to_pos_info(pos);
        let line = info.line_index;
        if !self.view.is_word_wrap_enabled() {
            return if end {
                self.document.line_end_position(line)
            } else {
                self.document.line_to_char(line)
            };
        }
        // VSCode-like: Home/End go to the start/end of the visual row when wrapped.
        // For End pass a huge col_in_row, Line clamps it.
        let lci = {
            let line_layout = self.line_layout(line);
            let row = line_layout.line_char_index_to_visual(info.line_char_index).row;
            line_layout.visual_to_line_char_index(row, if end { u32::MAX } else { 0 })
        };
        self.document.line_to_char(line) + lci.get() as usize
    }

    fn pos_after_rows(&mut self, pos: usize, sticky: u32, delta: i32) -> usize {
        let lines_count = self.document.lines_count() as u32;
        if lines_count == 0 {
            return pos;
        }
        let info = self.document.char_to_pos_info(pos);

        if !self.view.is_word_wrap_enabled() {
            // No wrap: visual rows == visible logical lines. Use fold-aware navigation.
            let new_line = if delta >= 0 {
                let target = self.document.folds().next_visible_nth_line(info.line_index, delta as u32);
                target.min(lines_count - 1)
            } else {
                self.document.folds().previous_visible_nth_line(info.line_index, delta.unsigned_abs())
            };
            let lci = self.line_layout(new_line).unwrapped_col_to_line_char_index(sticky);
            return self.document.line_to_char(new_line) + lci.get() as usize;
        }

        let mut line = info.line_index;
        let mut row = self.line_layout(line).line_char_index_to_visual(info.line_char_index).row;
        let mut remaining = delta;
        while remaining != 0 {
            if remaining > 0 {
                if row + 1 < self.line_layout(line).visual_row_count() {
                    row += 1;
                } else {
                    let next = self.document.folds().next_visible_nth_line(line, 1);
                    if next == line || next >= lines_count {
                        break; // end of document or no more visible lines
                    }
                    line = next;
                    row = 0;
                }
                remaining -= 1;
            } else {
                if row > 0 {
                    row -= 1;
                } else if line > 0 {
                    let prev = self.document.folds().previous_visible_nth_line(line, 1);
                    if prev == line {
                        break; // already at top visible line
                    }
                    line = prev;
                    row = self.line_layout(line).visual_row_count().saturating_sub(1);
                } else {
                    break;
                }
                remaining += 1;
            }
        }
        let lci = self.line_layout(line).visual_to_line_char_index(row, sticky);
        self.document.line_to_char(line) + lci.get() as usize
    }

    fn next_word_pos(&self, from: usize) -> usize {
        let mut iter = self.document.chars_iter(from);
        let Some(first) = iter.next() else {
            return from;
        };
        let char_class = CharClass::from(first);
        let mut pos = from + 1;
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
        pos.min(self.document.chars_count())
    }

    fn previous_word_pos(&self, from: usize) -> usize {
        if from == 0 {
            return 0;
        }
        let mut iter = self.document.chars_iter(from);
        let Some(first) = iter.prev() else {
            return from;
        };
        let char_class = CharClass::from(first);
        let mut pos = from - 1;
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
        pos
    }

    fn word_bounds(&self, pos: usize) -> (usize, usize) {
        let is_break = |c: char| c == '\n' || c == '\r';
        // the char under the click, or the one before it (click at the end of a line)
        let anchor = match self.document.char(pos) {
            Some(c) if !is_break(c) => pos,
            _ => match pos.checked_sub(1).and_then(|p| self.document.char(p)) {
                Some(c) if !is_break(c) => pos - 1,
                _ => return (pos, pos),
            },
        };
        let mut iter = self.document.chars_iter(anchor);
        let Some(first) = iter.next() else {
            return (pos, pos);
        };
        let class = CharClass::from(first);
        let same = |c: char| !is_break(c) && CharClass::from(c) == class;
        let mut end = anchor + 1;
        for c in iter {
            if !same(c) {
                break;
            }
            end += 1;
        }
        let mut start = anchor;
        let mut iter = self.document.chars_iter(anchor);
        while let Some(c) = iter.prev() {
            if !same(c) {
                break;
            }
            start -= 1;
        }
        (start, end)
    }

    fn line_range(&self, pos: usize) -> (usize, usize) {
        let line = self.document.position_to_line(pos);
        let start = self.document.line_to_char(line);
        let end = if (line as usize) + 1 < self.document.lines_count() {
            self.document.line_to_char(line + 1)
        } else {
            self.document.chars_count()
        };
        (start, end)
    }

    fn select_range(&mut self, anchor: usize, active: usize) {
        self.goto_position(active, false);
        let cursor = self.primary_cursor_mut();
        if anchor != cursor.pos {
            cursor.selection.update(anchor, cursor.pos);
        }
    }

    fn move_carets(&mut self, motion: Motion, select: bool) {
        for i in 0..self.cursors.len() {
            let cursor = self.cursors[i];
            let (new_pos, keep_sticky) = self.motion_target(cursor.pos, cursor.sticky_col, motion);
            let sticky_col = match keep_sticky {
                Some(s) => s,
                None => self.sticky_col_of(new_pos),
            };
            let cursor = &mut self.cursors[i];
            if select {
                cursor.selection.update(cursor.pos, new_pos);
            } else {
                cursor.selection.clear();
            }
            cursor.pos = new_pos;
            cursor.sticky_col = sticky_col;
        }
        self.normalize_carets();
        // follow_primary recomputes the sticky column: vertical moves must keep the one set above
        let sticky_col = self.primary_cursor().sticky_col;
        self.follow_primary();
        self.primary_cursor_mut().sticky_col = sticky_col;
    }

    /// Keeps the carets sorted and unique. Where carets overlap, the primary one survives.
    fn normalize_carets(&mut self) {
        if self.cursors.len() < 2 {
            return;
        }
        let primary = *self.primary_cursor();
        self.cursors.retain(|c| c.pos != primary.pos);
        self.cursors.sort_unstable_by_key(|c| c.pos);
        self.cursors.dedup_by_key(|c| c.pos);
        self.primary = self.cursors.partition_point(|c| c.pos < primary.pos);
        self.cursors.insert(self.primary, primary);
    }

    fn keep_only_primary(&mut self) {
        let primary = *self.primary_cursor();
        self.cursors.clear();
        self.cursors.push(primary);
        self.primary = 0;
    }

    fn refresh_sticky_cols(&mut self) {
        for i in 0..self.cursors.len() {
            let pos = self.cursors[i].pos;
            self.cursors[i].sticky_col = self.sticky_col_of(pos);
        }
    }

    /// Alt+Click: removes the caret at `pos`, or adds a new one there, which becomes the primary caret.
    fn toggle_caret(&mut self, pos: usize) {
        let pos = pos.min(self.document.chars_count());
        match self.cursors.binary_search_by_key(&pos, |c| c.pos) {
            Ok(i) if i == self.primary => {}
            Ok(i) => {
                self.cursors.remove(i);
                if i < self.primary {
                    self.primary -= 1;
                }
            }
            Err(i) => {
                self.cursors.insert(i, Cursor::new(pos));
                self.primary = i;
                self.goto_position(pos, false);
            }
        }
    }

    fn screen_to_position(&self, x: i32, y: i32) -> Option<usize> {
        let text_x = x - if self.margin.visible { self.margin.width as i32 } else { 0 };
        let target = y.max(0) as usize;
        let mut found = None;
        for (i, row) in self.view.visual_rows().enumerate() {
            found = Some(row);
            if i >= target {
                break;
            }
        }
        let row = found?;
        let lci = if self.view.is_word_wrap_enabled() {
            row.line.visual_to_line_char_index(row.row_in_line, text_x.max(0) as u32)
        } else {
            row.line
                .unwrapped_col_to_line_char_index((self.horizontal_scroll as i32 + text_x).max(0) as u32)
        };
        Some(self.document.line_to_char(row.line.line_number()) + lci.get() as usize)
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

    fn paint_selection(
        &self,
        surface: &mut Surface,
        selection: &Selection,
        line: &Line,
        row_in_line: u32,
        line_start: usize,
        x: i32,
        y: i32,
        attr: CharAttribute,
    ) {
        let len = line.logical_len() as usize;
        if selection.is_empty() || selection.start() > line_start + len || selection.end() <= line_start {
            return;
        }
        let w = self.view.size().width;
        let from = line.line_char_index_to_unwrapped_col(LineCharIndex::new(selection.start().saturating_sub(line_start) as u32));
        let end_index = selection.end() - line_start;
        let mut to = line.line_char_index_to_unwrapped_col(LineCharIndex::new(end_index as u32));
        if end_index > len {
            to += 1;
        }
        let (row_start, row_end) = if self.view.is_word_wrap_enabled() {
            let start = line.visual_to_unwrapped_col(row_in_line, 0).unwrap_or(0);
            let end = line.visual_to_unwrapped_col(row_in_line, u32::MAX).unwrap_or(start);
            let is_last = row_in_line + 1 == line.visual_row_count();
            (start, if is_last { (end + 1).min(start + w) } else { end })
        } else {
            (self.horizontal_scroll, self.horizontal_scroll + w)
        };
        for col in from.max(row_start)..to.min(row_end) {
            let cell_x = x + (col - row_start) as i32;
            let code = surface.char(cell_x, y).map_or(' ', |c| c.code);
            surface.write_char(cell_x, y, Character::with_attributes(code, attr));
        }
    }

    // ShowSpaces / ShowTabs
    fn paint_whitespace(&self, surface: &mut Surface, line: &Line, row_in_line: u32, x: i32, y: i32, back: Color) {
        let show_spaces = self.flags.contains(Flags::ShowSpaces);
        let show_tabs = self.flags.contains(Flags::ShowTabs);
        let width = self.view.size().width;
        let (row_start, row_end) = if self.view.is_word_wrap_enabled() {
            let start = line.visual_to_unwrapped_col(row_in_line, 0).unwrap_or(0);
            let end = line.visual_to_unwrapped_col(row_in_line, u32::MAX).unwrap_or(start);
            let is_last = row_in_line + 1 >= line.visual_row_count();
            (start, if is_last { start + width } else { end })
        } else {
            (self.horizontal_scroll, self.horizontal_scroll + width)
        };
        let tab = (self.tab_align as u32).max(1);
        let text = self.document.line(line.line_number());
        let len = line.logical_len();
        let indent_len = text.chars().take_while(|&c| c == ' ' || c == '\t').count() as u32;

        let indent_cols = if !show_tabs {
            0
        } else if indent_len >= len {
            line.line_char_index_to_unwrapped_col(LineCharIndex::new(len))
                .max(self.blank_line_indent(line.line_number()))
        } else {
            line.line_char_index_to_unwrapped_col(LineCharIndex::new(indent_len))
        };
        let mark = |surface: &mut Surface, col: u32, code: char| {
            if col >= row_start && col < row_end {
                surface.write_char(x + (col - row_start) as i32, y, Character::new(code, Color::Gray, back, CharFlags::None));
            }
        };

        let start = line.unwrapped_col_to_line_char_index(row_start).get();
        for (i, c) in (start..len).zip(text.chars_at(start as usize)) {
            let col = line.line_char_index_to_unwrapped_col(LineCharIndex::new(i));
            if col >= row_end {
                break;
            }
            match c {
                ' ' if show_spaces => mark(surface, col, '·'),
                '\t' if show_tabs && i >= indent_len => mark(surface, col, '→'),
                _ => {}
            }
        }

        for col in (0..indent_cols).step_by(tab as usize) {
            mark(surface, col, '│');
        }
    }

    /// Indentation width (in columns, tabs expanded) of a line; `None` when the line is blank.
    fn indent_columns(&self, line: u32) -> Option<u32> {
        let tab = (self.tab_align as u32).max(1);
        let mut cols = 0;
        for c in self.document.line(line).chars() {
            match c {
                '\t' => cols += tab - cols % tab,
                ' ' => cols += 1,
                '\n' | '\r' => return None,
                _ => return Some(cols),
            }
        }
        None
    }

    /// Bands of a blank line: the smaller indentation of the closest non-blank lines above and below.
    fn blank_line_indent(&self, line: u32) -> u32 {
        let lines_count = self.document.lines_count() as u32;
        let up = (line.saturating_sub(GUIDE_SEARCH_LINES)..line).rev().find_map(|l| self.indent_columns(l));
        let down = (line + 1..(line + 1 + GUIDE_SEARCH_LINES).min(lines_count)).find_map(|l| self.indent_columns(l));
        up.unwrap_or(0).min(down.unwrap_or(0))
    }

    /// A non-primary caret is painted as an inverted cell (the terminal has a single cursor).
    fn paint_secondary_caret(surface: &mut Surface, x: i32, y: i32) {
        if let Some(ch) = surface.char(x, y).copied() {
            surface.write_char(x, y, Character::new(ch.code, ch.background, ch.foreground, ch.flags));
        }
    }

    fn caret_location(&self, index: usize) -> Option<(u32, LineCharIndex)> {
        self.cursors.get(index).map(|c| {
            let info = self.document.char_to_pos_info(c.pos);
            (info.line_index, info.line_char_index)
        })
    }

    fn update_view(&mut self) {
        let col_normal = self.theme().editor.normal;
        self.view.update(self.start_line, &self.document, col_normal, self.tab_align as u32);
        // if scrollbars are enabled, update the max columns
        if self.flags.contains(Flags::ScrollBars) {
            self.view.update_max_columns();
        }
    }

    fn scroll_viewport(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }
        if self.view.is_word_wrap_enabled() {
            for _ in 0..delta.unsigned_abs() {
                let moved = if delta > 0 { self.viewport_row_down() } else { self.viewport_row_up() };
                if !moved {
                    break;
                }
            }
        } else {
            let new_start = if delta > 0 {
                let last = self.document.lines_count().saturating_sub(1) as u32;
                self.document.folds().next_visible_nth_line(self.start_line, delta as u32).min(last)
            } else {
                self.document.folds().previous_visible_nth_line(self.start_line, delta.unsigned_abs())
            };
            if new_start != self.start_line {
                self.start_line = new_start;
                self.update_view();
            }
        }
        self.scrollbars.set_indexes(self.horizontal_scroll as u64, self.start_line as u64);
        self.sync_cursor_visibility();
    }

    fn viewport_row_down(&mut self) -> bool {
        let top_rows = self.view.cached_visual_row_count(self.start_line).unwrap_or(1);
        let svr = self.view.start_visual_row();
        if svr + 1 < top_rows {
            self.view.set_start_visual_row(svr + 1);
            return true;
        }
        let max = self.document.lines_count() as u32;
        let next = self.document.folds().next_visible_nth_line(self.start_line, 1);
        if next == self.start_line || next >= max {
            return false;
        }
        self.start_line = next;
        self.view.set_start_visual_row(0);
        self.update_view();
        true
    }

    fn viewport_row_up(&mut self) -> bool {
        let svr = self.view.start_visual_row();
        if svr > 0 {
            self.view.set_start_visual_row(svr - 1);
            return true;
        }
        if self.start_line == 0 {
            return false;
        }
        let prev = self.document.folds().previous_visible_nth_line(self.start_line, 1);
        if prev == self.start_line {
            return false;
        }
        self.start_line = prev;
        self.update_view();
        let rows = self.view.cached_visual_row_count(self.start_line).unwrap_or(1);
        self.view.set_start_visual_row(rows.saturating_sub(1));
        true
    }

    fn scroll_from_scrollbars(&mut self) {
        let last = self.document.lines_count().saturating_sub(1) as u64;
        let new_start = self.scrollbars.vertical_index().min(last) as u32;
        if !self.view.is_word_wrap_enabled() {
            self.horizontal_scroll = self.scrollbars.horizontal_index() as u32;
        }
        if new_start != self.start_line {
            self.start_line = new_start;
            self.view.set_start_visual_row(0);
            self.update_view();
        }
        self.sync_cursor_visibility();
    }

    fn on_mouse_pressed(&mut self, data: &MouseEventData) {
        // a click right after a double-click, in the same place => triple-click
        let triple = self
            .last_double_click
            .take()
            .is_some_and(|(time, x, y)| x == data.x && y == data.y && time.elapsed() <= TRIPLE_CLICK_TIME);
        self.mouse_drag = MouseDrag::None;
        self.drag_pos = (data.x, data.y);
        let Some(pos) = self.screen_to_position(data.x, data.y) else {
            return;
        };
        if data.modifier.contains(KeyModifier::Alt) {
            self.toggle_caret(pos);
            return;
        }
        self.keep_only_primary();
        if triple {
            let (start, end) = self.line_range(pos);
            self.select_range(start, end);
            self.mouse_drag = MouseDrag::Lines { start, end };
        } else {
            self.goto_position(pos, data.modifier.contains(KeyModifier::Shift));
            self.mouse_drag = MouseDrag::Chars;
        }
    }

    fn on_mouse_double_click(&mut self, data: &MouseEventData) {
        self.mouse_drag = MouseDrag::None;
        let Some(pos) = self.screen_to_position(data.x, data.y) else {
            return;
        };
        if data.modifier.contains(KeyModifier::Alt) {
            // second Alt+Click in the same place (the terminal reports it as a double-click)
            self.toggle_caret(pos);
            return;
        }
        self.keep_only_primary();
        let (start, end) = self.word_bounds(pos);
        self.select_range(start, end);
        self.last_double_click = Some((Instant::now(), data.x, data.y));
    }

    fn on_mouse_drag(&mut self, data: &MouseEventData) {
        let h = self.view.size().height as i32;
        let mut y = data.y;
        if y < 0 {
            self.scroll_viewport(-1);
            y = 0;
        } else if h > 0 && y >= h {
            self.scroll_viewport(1);
            y = h - 1;
        }
        self.drag_pos = (data.x, y);
        self.extend_drag_selection();
    }

    fn on_mouse_wheel(&mut self, delta: i32) {
        self.scroll_viewport(delta);
        if self.mouse_drag != MouseDrag::None {
            self.extend_drag_selection();
        }
    }

    fn extend_drag_selection(&mut self) {
        let (x, y) = self.drag_pos;
        let Some(pos) = self.screen_to_position(x, y) else {
            return;
        };
        match self.mouse_drag {
            MouseDrag::None => {}
            MouseDrag::Chars => self.goto_position(pos, true),
            MouseDrag::Lines { start, end } => {
                let (line_start, line_end) = self.line_range(pos);
                if line_start >= start {
                    self.select_range(start, line_end);
                } else {
                    self.select_range(end, line_start);
                }
            }
        }
    }

    fn previous_char_pos(&self, pos: usize) -> usize {
        if pos >= 2 && self.document.char(pos - 1) == Some('\n') && self.document.char(pos - 2) == Some('\r') {
            pos - 2
        } else {
            pos.saturating_sub(1)
        }
    }

    fn next_char_pos(&self, pos: usize) -> usize {
        if self.document.char(pos) == Some('\r') && self.document.char(pos + 1) == Some('\n') {
            pos + 2
        } else {
            (pos + 1).min(self.document.chars_count())
        }
    }

    fn edit_carets(&mut self, texts: &[&str], range: impl Fn(&Self, usize) -> (usize, usize)) {
        if texts.is_empty() {
            return;
        }
        self.edit_carets_with(|_, i, _| texts[i % texts.len()].to_string(), range);
    }

    fn edit_carets_with(&mut self, text: impl Fn(&Self, usize, usize) -> String, range: impl Fn(&Self, usize) -> (usize, usize)) {
        if self.flags.contains(Flags::ReadOnly) {
            return;
        }
        let mut offset = 0isize;
        let mut min_start = 0usize;
        let mut first_line = u32::MAX;
        let mut last_line = 0u32;
        let mut lines_changed = false;
        for i in 0..self.cursors.len() {
            let shift = |p: usize| p.saturating_add_signed(offset);
            let cursor = &mut self.cursors[i];
            cursor.pos = shift(cursor.pos);
            cursor.selection.map(shift);
            let cursor = self.cursors[i];
            let (start, end) = if cursor.selection.is_empty() {
                range(self, cursor.pos)
            } else {
                (cursor.selection.start(), cursor.selection.end())
            };
            // overlapping selections: never touch the text already edited for a previous caret
            let start = start.max(min_start);
            let end = end.max(start);
            let text = text(self, i, start);
            let text = text.as_str();
            if start == end && text.is_empty() {
                continue;
            }
            let start_line = self.document.position_to_line(start);
            lines_changed |= start_line != self.document.position_to_line(end) || text.contains(['\n', '\r']);
            first_line = first_line.min(start_line);
            last_line = last_line.max(start_line);
            self.document.remove(start, end);
            self.document.insert(start, text);
            let inserted = text.chars().count();
            offset += inserted as isize - (end - start) as isize;
            let cursor = &mut self.cursors[i];
            cursor.pos = start + inserted;
            cursor.selection.clear();
            min_start = cursor.pos;
        }
        if first_line != u32::MAX {
            self.after_edit(first_line, if lines_changed { u32::MAX } else { last_line });
        }
    }

    /// After the text changed on lines `first_line..=last_line` (`u32::MAX` = to the end).
    fn after_edit(&mut self, first_line: u32, last_line: u32) {
        self.view.invalidate_lines(first_line, last_line);
        self.normalize_carets();
        // the line count can gain / lose a digit => the margin and the text area change width
        let (width, visible) = (self.margin.width, self.margin.visible);
        self.update_margin();
        if self.margin.width != width {
            self.resize_view(self.size());
        } else {
            self.margin.visible = visible;
        }
        self.update_view();
        self.follow_primary();
        self.refresh_sticky_cols();
        self.notify_document_changed();
    }

    fn type_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let text: &str = ch.encode_utf8(&mut buf);
        let overwrite = self.overwrite;
        self.edit_carets(&[text], |ed, pos| match ed.document.char(pos) {
            Some(c) if overwrite && c != '\n' && c != '\r' => (pos, pos + 1),
            _ => (pos, pos),
        });
    }

    /// Tab: spaces up to the next tab stop (every `tab_align` columns), like Zed with `hard_tabs: false`.
    fn insert_soft_tab(&mut self) {
        self.edit_carets_with(
            |ed, _, start| {
                let tab = (ed.tab_align as usize).max(1);
                " ".repeat(tab - ed.column_of(start) % tab)
            },
            |_, pos| (pos, pos),
        );
    }

    /// Column of a position in its line, with the tabs expanded.
    fn column_of(&self, pos: usize) -> usize {
        let tab = (self.tab_align as usize).max(1);
        let line_start = self.document.line_to_char(self.document.position_to_line(pos));
        self.document
            .chars_iter(line_start)
            .take(pos - line_start)
            .fold(0, |col, c| if c == '\t' { col + tab - col % tab } else { col + 1 })
    }

    fn selection_spans_lines(&self) -> bool {
        self.cursors.iter().any(|c| {
            !c.selection.is_empty() && self.document.position_to_line(c.selection.start()) != self.document.position_to_line(c.selection.end())
        })
    }

    /// Lines touched by the carets and their selections, sorted and unique.
    /// A selection ending at the start of a line does not include that line.
    fn caret_lines(&self) -> Vec<u32> {
        let mut lines = Vec::new();
        for c in &self.cursors {
            let (first, last) = if c.selection.is_empty() {
                let l = self.document.position_to_line(c.pos);
                (l, l)
            } else {
                let first = self.document.position_to_line(c.selection.start());
                let mut last = self.document.position_to_line(c.selection.end());
                if last > first && self.document.line_to_char(last) == c.selection.end() {
                    last -= 1;
                }
                (first, last)
            };
            lines.extend(first..=last);
        }
        lines.sort_unstable();
        lines.dedup();
        lines
    }

    fn indent_lines(&mut self, outdent: bool) {
        if self.flags.contains(Flags::ReadOnly) {
            return;
        }
        let lines = self.caret_lines();
        let mut changed = false;
        // bottom-up: an edit never moves the lines that are still to be processed
        for &line in lines.iter().rev() {
            let start = self.document.line_to_char(line);
            if outdent {
                let removed = self.indent_width(start);
                if removed == 0 {
                    continue;
                }
                self.document.remove(start, start + removed);
                let map = |p: usize| if p >= start + removed { p - removed } else { p.min(start) };
                for c in self.cursors.iter_mut() {
                    c.pos = map(c.pos);
                    c.selection.map(map);
                }
            } else {
                // spaces, not a tab (like Tab in the text)
                let added = (self.tab_align as usize).max(1);
                self.document.insert(start, &" ".repeat(added));
                for c in self.cursors.iter_mut() {
                    // a caret at the line start moves after the spaces, a selection starting there grows
                    if c.pos > start || (c.pos == start && c.selection.is_empty()) {
                        c.pos += added;
                    }
                    c.selection.map(|p| if p > start { p + added } else { p });
                }
            }
            changed = true;
        }
        if changed {
            self.after_edit(lines[0], lines[lines.len() - 1]);
        }
    }

    /// Indentation removed by Shift+Tab from the line starting at `start`.
    fn indent_width(&self, start: usize) -> usize {
        let mut it = self.document.chars_iter(start);
        if it.next() == Some('\t') {
            return 1;
        }
        let mut it = self.document.chars_iter(start);
        let mut n = 0;
        while n < self.tab_align as usize && it.next() == Some(' ') {
            n += 1;
        }
        n
    }

    fn copy_to_clipboard(&self) -> bool {
        let has_selection = self.cursors.iter().any(|c| !c.selection.is_empty());
        let line_break = self.document.line_break();
        let mut text = String::new();
        let mut last_line = u32::MAX;
        for c in &self.cursors {
            let (start, end) = if has_selection {
                if c.selection.is_empty() {
                    continue;
                }
                if !text.is_empty() {
                    text.push_str(line_break);
                }
                (c.selection.start(), c.selection.end())
            } else {
                let line = self.document.position_to_line(c.pos);
                if line == last_line {
                    continue;
                }
                last_line = line;
                self.line_range(c.pos)
            };
            for chunk in self.document.slice(start, end).chunks() {
                text.push_str(chunk);
            }
            if !has_selection && !text.ends_with(['\n', '\r']) {
                text.push_str(line_break);
            }
        }
        Clipboard::set_text(&text);
        has_selection
    }

    fn cut(&mut self) {
        if self.copy_to_clipboard() {
            self.edit_carets(&[""], |_, pos| (pos, pos));
            return;
        }
        self.keep_one_caret_per_line();
        self.edit_carets(&[""], |ed, pos| {
            let (start, end) = ed.line_range(pos);
            let line = ed.document.position_to_line(pos);
            if line > 0 && line as usize + 1 == ed.document.lines_count() {
                (ed.document.line_end_position(line - 1), end)
            } else {
                (start, end)
            }
        });
    }

    fn keep_one_caret_per_line(&mut self) {
        let primary = *self.primary_cursor();
        let doc = &self.document;
        let primary_line = doc.position_to_line(primary.pos);
        self.cursors.retain(|c| doc.position_to_line(c.pos) != primary_line);
        self.cursors.dedup_by_key(|c| doc.position_to_line(c.pos));
        self.primary = self.cursors.partition_point(|c| c.pos < primary.pos);
        self.cursors.insert(self.primary, primary);
    }

    fn paste(&mut self) {
        let Some(text) = Clipboard::text() else {
            return;
        };
        if text.is_empty() {
            return;
        }
        let line_break = self.document.line_break();
        let text = text.replace("\r\n", "\n");
        let text = if line_break == "\n" { text } else { text.replace('\n', line_break) };
        if self.cursors.len() > 1 {
            let lines: Vec<&str> = text.strip_suffix(line_break).unwrap_or(&text).split(line_break).collect();
            if lines.len() == self.cursors.len() {
                self.edit_carets(&lines, |_, pos| (pos, pos));
                return;
            }
        }
        self.edit_carets(&[&text], |_, pos| (pos, pos));
    }

    fn resize_view(&mut self, size: Size) {
        if size.width <= self.margin.width as u32 {
            self.margin.visible = false;
            self.view.resize(size);
        } else {
            self.margin.visible = self.flags.contains(Flags::ShowLineNumbers);
            self.view
                .resize(Size::new(size.width.saturating_sub(self.margin.width as u32), size.height));
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

        // Carets are sorted by position and rows are painted top-down,
        // so they are matched to rows in a single forward pass.
        let paint_carets = self.has_focus() && self.cursors.len() > 1;
        let has_selection = self.cursors.iter().any(|c| !c.selection.is_empty());
        let attr_selection = theme.editor.pressed_or_selected;
        let show_whitespace = self.flags.contains(Flags::ShowSpaces) || self.flags.contains(Flags::ShowTabs);
        let mut next_caret = 0usize;
        let mut pending: Option<(u32, LineCharIndex)> = None;

        let mut y = 0i32;
        for row in self.view.visual_rows() {
            if y >= h {
                break;
            }
            let line_number = row.line.line_number();
            if self.margin.visible {
                if row.is_first_row {
                    self.paint_line_number(surface, 0, y, line_number + 1, theme.editor.inactive);
                }
            }

            let chars = if wrap_enabled {
                row.line.visual_row_chars(row.row_in_line, w)
            } else {
                row.line.unwrapped_chars(self.horizontal_scroll, self.horizontal_scroll + w)
            };
            // Surface::write_chars paints nothing for a single cell
            match chars {
                [c] => surface.write_char(x, y, *c),
                _ => surface.write_chars(x, y, chars),
            }
            if show_whitespace {
                self.paint_whitespace(surface, row.line, row.row_in_line, x, y, theme.editor.normal.background);
            }

            if has_selection {
                let line_start = self.document.line_to_char(line_number);
                for cursor in &self.cursors {
                    self.paint_selection(surface, &cursor.selection, row.line, row.row_in_line, line_start, x, y, attr_selection);
                }
            }

            if paint_carets {
                if y == 0 {
                    let first_pos = self.document.line_to_char(line_number);
                    next_caret = self.cursors.partition_point(|c| c.pos < first_pos);
                    pending = self.caret_location(next_caret);
                }
                while let Some((caret_line, lci)) = pending {
                    if caret_line > line_number {
                        break;
                    }
                    if caret_line == line_number {
                        let vp = row.line.line_char_index_to_visual(lci);
                        if vp.row > row.row_in_line {
                            break;
                        }
                        // the primary caret is shown by the terminal cursor
                        if vp.row == row.row_in_line && next_caret != self.primary {
                            let col = if wrap_enabled {
                                Some(vp.col_in_row)
                            } else {
                                let c = row.line.line_char_index_to_unwrapped_col(lci);
                                (c >= self.horizontal_scroll && c < self.horizontal_scroll + w).then(|| c - self.horizontal_scroll)
                            };
                            if let Some(col) = col {
                                Self::paint_secondary_caret(surface, x + col as i32, y);
                            }
                        }
                    }
                    next_caret += 1;
                    pending = self.caret_location(next_caret);
                }
            }
            y += 1;
        }

        if self.has_focus() && self.cursor_view.visible {
            // TODO: paint selection
            let cursor_y = self.cursor_screen_y();
            surface.set_cursor(self.cursor_view.column as i32 + x, cursor_y);
        }
    }
}

impl Editor {
    fn cursor_screen_y(&self) -> i32 {
        if !self.view.is_word_wrap_enabled() {
            return self.cursor_view.line.saturating_sub(self.start_line) as i32;
        }
        // Walk cached logical lines from start_line, summing visual rows.
        let mut y: i32 = -(self.view.start_visual_row() as i32);
        let mut ln = self.start_line;
        while ln < self.cursor_view.line {
            y += self.view.cached_visual_row_count(ln).unwrap_or(1) as i32;
            ln += 1;
        }
        y + self.cursor_view.row_in_line as i32
    }
}

impl OnKeyPressed for Editor {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        let page = self.view.size().height as i32;
        let motion = match key.value() {
            key!("Left") | key!("Shift+Left") => Motion::Left,
            key!("Right") | key!("Shift+Right") => Motion::Right,
            key!("Up") | key!("Shift+Up") => Motion::Rows(-1),
            key!("Down") | key!("Shift+Down") => Motion::Rows(1),
            key!("PageUp") | key!("Shift+PageUp") => Motion::Rows(-page),
            key!("PageDown") | key!("Shift+PageDown") => Motion::Rows(page),
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => Motion::WordLeft,
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => Motion::WordRight,
            key!("Home") | key!("Shift+Home") => Motion::RowStart,
            key!("End") | key!("Shift+End") => Motion::RowEnd,
            key!("Ctrl+Home") | key!("Ctrl+Shift+Home") => Motion::DocStart,
            key!("Ctrl+End") | key!("Ctrl+Shift+End") => Motion::DocEnd,

            // viewport scroll (cursor stays put)
            key!("Ctrl+Up") => {
                self.scroll_viewport(-1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") => {
                self.scroll_viewport(1);
                return EventProcessStatus::Processed;
            }

            key!("Ctrl+A") => {
                self.keep_only_primary();
                let end = self.document.chars_count();
                self.goto_position(end, false);
                self.primary_cursor_mut().selection.set(0, end);
                return EventProcessStatus::Processed;
            }

            key!("Alt+Z") => {
                self.set_word_wrap(!self.view.is_word_wrap_enabled());
                return EventProcessStatus::Processed;
            }

            key!("Escape") => {
                if self.cursors.len() == 1 {
                    return EventProcessStatus::Ignored;
                }
                self.keep_only_primary();
                return EventProcessStatus::Processed;
            }

            key!("Enter") => {
                let line_break = self.document.line_break();
                self.edit_carets(&[line_break], |_, pos| (pos, pos));
                return EventProcessStatus::Processed;
            }
            key!("Tab") => {
                if self.selection_spans_lines() {
                    self.indent_lines(false);
                } else {
                    self.insert_soft_tab();
                }
                return EventProcessStatus::Processed;
            }
            key!("Shift+Tab") => {
                self.indent_lines(true);
                return EventProcessStatus::Processed;
            }
            key!("Backspace") => {
                self.edit_carets(&[""], |ed, pos| (ed.previous_char_pos(pos), pos));
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.edit_carets(&[""], |ed, pos| (pos, ed.next_char_pos(pos)));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Backspace") => {
                self.edit_carets(&[""], |ed, pos| (ed.previous_word_pos(pos), pos));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Delete") => {
                self.edit_carets(&[""], |ed, pos| (pos, ed.next_word_pos(pos)));
                return EventProcessStatus::Processed;
            }
            key!("Insert") => {
                self.overwrite = !self.overwrite;
                return EventProcessStatus::Processed;
            }

            // Ctrl+Insert / Shift+Delete / Shift+Insert: the same keys as the other AppCUI text controls
            key!("Ctrl+C") | key!("Ctrl+Insert") => {
                self.copy_to_clipboard();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+X") | key!("Shift+Delete") => {
                self.cut();
                return EventProcessStatus::Processed;
            }

            key!("Ctrl+V") | key!("Ctrl+Shift+V") | key!("Shift+Insert") => {
                self.paste();
                return EventProcessStatus::Processed;
            }

            _ => {
                let ctrl_only = key.modifier.contains(KeyModifier::Ctrl) && !key.modifier.contains(KeyModifier::Alt);
                if character as u32 >= 32 && !ctrl_only {
                    self.type_char(character);
                    return EventProcessStatus::Processed;
                }

                if key.code == KeyCode::Tab && key.modifier.contains(KeyModifier::Ctrl) {
                    return EventProcessStatus::Ignored;
                }

                for m in [KeyModifier::Alt, KeyModifier::Shift, KeyModifier::Ctrl] {
                    if key.modifier.contains(m) {
                        let mut modifier = key.modifier;
                        modifier.remove(m);
                        return self.on_key_pressed(Key::new(key.code, modifier), '\0');
                    }
                }
                return EventProcessStatus::Ignored;
            }
        };
        self.move_carets(motion, select);
        EventProcessStatus::Processed
    }
}

impl OnMouseEvent for Editor {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.scroll_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Pressed(data) if data.button == MouseButton::Left => {
                self.on_mouse_pressed(data);
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) if data.button == MouseButton::Left => {
                self.on_mouse_double_click(data);
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.mouse_drag == MouseDrag::None {
                    return EventProcessStatus::Ignored;
                }
                self.on_mouse_drag(data);
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.mouse_drag = MouseDrag::None;
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(MouseWheelDirection::Up) => {
                self.on_mouse_wheel(-WHEEL_SCROLL_ROWS);
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(MouseWheelDirection::Down) => {
                self.on_mouse_wheel(WHEEL_SCROLL_ROWS);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnResize for Editor {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.scrollbars
            .resize(self.view.max_columns() as u64, self.document.lines_count() as u64, &self.base);
        self.resize_view(new_size);
        self.update_view();
        // Re-anchor cursor so column/row_in_line and scroll offsets stay consistent.
        let pos = self.primary_cursor().pos;
        self.goto_position(pos, false);
        self.refresh_sticky_cols();
    }
}

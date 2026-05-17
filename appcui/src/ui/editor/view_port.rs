use super::{Document, Line, LineCharIndex, VisualPos};
use crate::prelude::{CharAttribute, Size};

pub(super) struct ViewPort {
    lines: Vec<Line>,
    size: Size,
    count: u32,
    word_wrap: bool,
    /// First visual row of `lines[0]` shown on screen. Always 0 when wrap is off.
    /// When wrap is on, lets us scroll partway into a tall logical line.
    start_visual_row: u32,
}

/// A row to paint: which logical line, which visual row within it.
pub(super) struct VisualRowRef<'a> {
    pub(super) line: &'a Line,
    pub(super) row_in_line: u32,
    pub(super) is_first_row: bool,
}

impl ViewPort {
    pub(super) fn new(initial_capacity: usize) -> Self {
        Self {
            lines: Vec::with_capacity(initial_capacity),
            size: Size::new(0, 0),
            count: 0,
            word_wrap: false,
            start_visual_row: 0,
        }
    }

    pub(super) fn resize(&mut self, sz: Size) {
        if self.size != sz {
            self.size = sz;
            self.lines.reserve(sz.height.max(8) as usize);
            while self.lines.len() < sz.height.max(8) as usize {
                self.lines.push(Line::new(sz.width.max(8) as usize));
            }
            // Re-wrap any cached lines whose wrap width depends on viewport width.
            if self.word_wrap {
                for i in 0..self.count as usize {
                    self.lines[i].update_wrap_points(self.size.width);
                }
            }
        }
    }

    #[inline(always)]
    pub(super) fn size(&self) -> Size {
        self.size
    }

    /// Enable/disable wrap. `None` disables; `Some(_)` enables and re-wraps cached lines.
    /// The actual wrap column is the viewport width (the argument is currently just a flag);
    /// kept as Option<u32> so a future "fixed wrap column" mode is a small change.
    pub(super) fn set_wrap_enabled(&mut self, enabled: bool) {
        let w = if enabled { self.size.width } else { 0 };
        for i in 0..self.count as usize {
            self.lines[i].update_wrap_points(w);
        }
        self.start_visual_row = 0;
        self.word_wrap = enabled;
    }

    #[inline(always)]
    pub(super) fn is_word_wrap_enabled(&self) -> bool {
        self.word_wrap
    }

    #[inline(always)]
    pub(super) fn start_visual_row(&self) -> u32 {
        self.start_visual_row
    }

    pub(super) fn set_start_visual_row(&mut self, row: u32) {
        if self.count == 0 {
            self.start_visual_row = 0;
            return;
        }
        let max = self.lines[0].visual_row_count().saturating_sub(1);
        self.start_visual_row = row.min(max);
    }

    pub(super) fn contains_line(&self, line_number: u32) -> bool {
        if self.count == 0 {
            return false;
        }
        line_number >= self.lines[0].line_number() && line_number <= self.lines[self.count as usize - 1].line_number()
    }

    pub(super) fn first_line(&self) -> u32 {
        if self.count == 0 {
            u32::MAX
        } else {
            self.lines[0].line_number()
        }
    }

    pub(super) fn last_line(&self) -> u32 {
        if self.count == 0 {
            u32::MAX
        } else {
            self.lines[self.count as usize - 1].line_number()
        }
    }

    pub(super) fn reset(&mut self) {
        self.count = 0;
        self.start_visual_row = 0;
    }

    #[inline(always)]
    pub(super) fn update_line(&mut self, index: u32, line_number: u32, doc: &Document, attr: CharAttribute, tab_align: u32) {
        if index >= self.lines.len() as u32 {
            return;
        }
        // Fill any gap with reset lines so `count` matches `index + 1` afterwards.
        while self.count <= index {
            self.lines[self.count as usize].reset();
            self.count += 1;
        }
        self.lines[index as usize].update(line_number, doc, attr, tab_align);
        if self.word_wrap {
            self.lines[index as usize].update_wrap_points(self.size.width);
        }
    }

    /// O(1) — lines in the viewport are contiguous starting at `first_line()`.
    pub(super) fn line_number_to_index(&self, line_number: u32) -> Option<u32> {
        if self.count == 0 {
            return None;
        }
        let first = self.lines[0].line_number();
        if line_number < first {
            return None;
        }
        let offset = line_number - first;
        if offset >= self.count {
            return None;
        }
        // Defensive check: confirm contiguity (cheap; one comparison).
        debug_assert_eq!(self.lines[offset as usize].line_number(), line_number);
        Some(offset)
    }

    // --- Unwrapped column queries (kept for selection painting, internal use) ---

    pub(super) fn unwrapped_col(&self, line_number: u32, char_index: LineCharIndex) -> Option<u32> {
        let idx = self.line_number_to_index(line_number)?;
        self.lines[idx as usize].line_char_index_to_unwrapped_col(char_index)
    }

    pub(super) fn unwrapped_col_to_line_char_index(&self, line_number: u32, column: u32) -> Option<LineCharIndex> {
        let idx = self.line_number_to_index(line_number)?;
        self.lines[idx as usize].unwrapped_col_to_line_char_index(column)
    }

    // --- Visual queries (cursor paint, mouse, vertical movement) ---

    /// Visual position of a logical char within its line (row + col_in_row).
    pub(super) fn visual_pos(&self, line_number: u32, char_index: LineCharIndex) -> Option<VisualPos> {
        let idx = self.line_number_to_index(line_number)?;
        self.lines[idx as usize].line_char_index_to_visual(char_index)
    }

    /// Map a visual position within a logical line to a logical char index.
    pub(super) fn visual_to_line_char_index(&self, line_number: u32, row_in_line: u32, col_in_row: u32) -> Option<LineCharIndex> {
        let idx = self.line_number_to_index(line_number)?;
        self.lines[idx as usize].visual_to_line_char_index(row_in_line, col_in_row)
    }

    /// Iterate paintable visual rows, top-to-bottom, clipped to viewport height.
    pub(super) fn visual_rows(&self) -> impl Iterator<Item = VisualRowRef<'_>> {
        let height = self.size.height;
        let start_row = self.start_visual_row;
        let count = self.count as usize;
        let lines = &self.lines[..count];

        lines
            .iter()
            .enumerate()
            .flat_map(move |(li, line)| {
                let total = line.visual_row_count();
                let skip = if li == 0 { start_row } else { 0 };
                (skip..total).map(move |r| VisualRowRef {
                    line,
                    row_in_line: r,
                    is_first_row: r == 0,
                })
            })
            .take(height as usize)
    }

    pub(super) fn cached_visual_row_count(&self, line_number: u32) -> Option<u32> {
        let idx = self.line_number_to_index(line_number)?;
        Some(self.lines[idx as usize].visual_row_count())
    }
}

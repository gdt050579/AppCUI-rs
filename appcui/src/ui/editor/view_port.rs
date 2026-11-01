use super::{Document, Folds, Line, LineCharIndex, VisualPos};
use crate::prelude::{CharAttribute, Size};

pub(super) struct ViewPort {
    lines: Vec<Line>,
    size: Size,
    count: u32,
    word_wrap: bool,
    start_visual_row: u32,
    max_columns: u32,
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
            max_columns: 0,
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
        self.max_columns = 0;
    }

    pub(super) fn update(&mut self, start_line: u32, doc: &Document, attr: CharAttribute, tab_align: u32) {
        let total_lines = doc.lines_count() as u32;
        let capacity = self.lines.len() as u32;
        let folds = doc.folds();
        let cur_count = self.count as usize;
    
        // Pass 1: find target[0] (the first visible line) and locate it in the current cache.
        let first_target = {
            let mut it = folds.visible_lines_from(start_line);
            loop {
                match it.next() {
                    Some(ln) if ln < total_lines => break Some(ln),
                    Some(_) => break None,
                    None => break None,
                }
            }
        };
        let Some(first_target) = first_target else {
            self.count = 0;
            return;
        };
    
        let reuse = if let Some(k) = (0..cur_count).find(|&i| self.lines[i].line_number() == first_target) {
            if k > 0 {
                self.lines[..cur_count].rotate_left(k);
            }
            0..cur_count - k
        } else if let Some(k) = self.scrolled_up_by(first_target, folds) {
            self.lines.rotate_right(k);
            k..(cur_count + k).min(capacity as usize)
        } else {
            0..0
        };

        let wrap_width = if self.word_wrap { self.size.width } else { 0 };
        let mut it = folds.visible_lines_from(start_line);
        let mut i = 0u32;
        while i < capacity {
            let Some(ln) = it.next() else { break; };
            if ln >= total_lines { break; }
            let slot = i as usize;
            let line = &mut self.lines[slot];
            if !reuse.contains(&slot) || line.line_number() != ln || line.is_modified() {
                line.update(ln, doc, attr, tab_align);
                if wrap_width > 0 {
                    line.update_wrap_points(wrap_width);
                }
            }
            i += 1;
        }
        self.count = i;
    }

    fn scrolled_up_by(&self, new_first: u32, folds: &Folds) -> Option<usize> {
        if self.count == 0 {
            return None;
        }
        let old_first = self.lines[0].line_number();
        if old_first <= new_first {
            return None;
        }
        folds
            .visible_lines_from(new_first)
            .take(self.lines.len())
            .take_while(|&ln| ln <= old_first)
            .position(|ln| ln == old_first)
    }

    #[inline(always)]
    pub(super) fn line_at(&self, index: u32) -> &Line {
        &self.lines[index as usize]
    }

    #[inline(always)]
    pub(super) fn update_max_columns(&mut self) {
        self.max_columns = self.lines[..self.count as usize].iter().map(|line| line.columns_count()).max().unwrap_or(0);
    }

    #[inline(always)]
    pub(super) fn max_columns(&self) -> u32 {
        if self.word_wrap { 0 } else { self.max_columns }
    }

    pub(super) fn line_number_to_index(&self, line_number: u32) -> Option<u32> {
        let lines = &self.lines[..self.count as usize];
        let first = lines.first()?.line_number();
        if line_number < first {
            return None;
        }
        let offset = (line_number - first) as usize;
        if offset < lines.len() && lines[offset].line_number() == line_number {
            return Some(offset as u32);
        }
        lines
            .binary_search_by_key(&line_number, |l| l.line_number())
            .ok()
            .map(|i| i as u32)
    }

    // --- Unwrapped column queries (kept for selection painting, internal use) ---

    pub(super) fn unwrapped_col(&self, line_number: u32, char_index: LineCharIndex) -> Option<u32> {
        let idx = self.line_number_to_index(line_number)?;
        Some(self.lines[idx as usize].line_char_index_to_unwrapped_col(char_index))
    }

    pub(super) fn unwrapped_col_to_line_char_index(&self, line_number: u32, column: u32) -> Option<LineCharIndex> {
        let idx = self.line_number_to_index(line_number)?;
        Some(self.lines[idx as usize].unwrapped_col_to_line_char_index(column))
    }

    // --- Visual queries (cursor paint, mouse, vertical movement) ---

    /// Visual position of a logical char within its line (row + col_in_row).
    pub(super) fn visual_pos(&self, line_number: u32, char_index: LineCharIndex) -> Option<VisualPos> {
        let idx = self.line_number_to_index(line_number)?;
        Some(self.lines[idx as usize].line_char_index_to_visual(char_index))    
    }

    /// Map a visual position within a logical line to a logical char index.
    pub(super) fn visual_to_line_char_index(&self, line_number: u32, row_in_line: u32, col_in_row: u32) -> Option<LineCharIndex> {
        let idx = self.line_number_to_index(line_number)?;
        Some(self.lines[idx as usize].visual_to_line_char_index(row_in_line, col_in_row))
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

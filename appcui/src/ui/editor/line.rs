use super::Document;
use crate::prelude::{CharAttribute, Character};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(super) struct LineCharIndex(u32);
impl LineCharIndex {
    #[inline(always)]
    pub(super) fn new(index: u32) -> Self {
        Self(index)
    }
    #[inline(always)]
    pub(super) fn get(&self) -> u32 {
        self.0
    }
}

/// A position on the wrapped screen: which visual row of the logical line,
/// and the column offset within that row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VisualPos {
    pub(super) row: u32,
    pub(super) col_in_row: u32,
}

pub struct Line {
    line_number: u32,
    chars: Vec<Character>,
    col_map: Vec<u32>,
    wrap_points: Vec<u32>,
    is_modified: bool,
    last_col: u32,
}

impl Line {
    pub(super) fn new(initial_capacity: usize) -> Self {
        let mut wrap_points = Vec::with_capacity(4);
        wrap_points.push(0);
        Self {
            line_number: u32::MAX,
            chars: Vec::with_capacity(initial_capacity),
            col_map: Vec::with_capacity(initial_capacity),
            wrap_points,
            is_modified: false,
            last_col: 0,
        }
    }

    pub(super) fn update(&mut self, line_number: u32, doc: &Document, attr: CharAttribute, tab_align: u32) {
        self.line_number = line_number;
        self.chars.clear();
        self.col_map.clear();
        let mut col = 0u32;

        for c in doc.line(line_number).chars() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.chars.push(Character::with_attributes(if c as u32 >= 32 { c } else { ' ' }, attr));
            self.col_map.push(col);
            if c == '\t' {
                col += tab_align - (col % tab_align);
                while (col as usize) > self.chars.len() {
                    self.chars.push(Character::with_attributes(' ', attr));
                }
            } else {
                col += 1;
            }

        }
        self.last_col = col;
        self.wrap_points.clear();
        self.wrap_points.push(0);
        self.is_modified = false;
    }

    /// Recompute wrap break columns for the current `chars` buffer.
    /// `width = None` (or 0) disables wrap: a single row starting at column 0.
    /// Call after `update` whenever wrap width changes.
    pub(super) fn update_wrap_points(&mut self, width: u32) {
        self.wrap_points.clear();
        self.wrap_points.push(0);

        if width == 0 {
            return;
        }
        let total = self.chars.len() as u32;
        if total <= width {
            return;
        }

        // Word-wrap: prefer breaking at the last whitespace inside the window.
        // Fall back to hard break at `w` if no whitespace is found (long token).
        let mut row_start = 0u32;
        while row_start + width < total {
            let window_end = row_start + width;
            let mut break_at = None;
            let mut i = window_end;
            while i > row_start + 1 {
                i -= 1;
                if self.chars[i as usize].code == ' ' {
                    break_at = Some(i + 1);
                    break;
                }
            }
            let next_start = break_at.unwrap_or(window_end);
            if next_start <= row_start {
                break;
            }
            self.wrap_points.push(next_start);
            row_start = next_start;
        }
    }

    pub(super) fn mark_modified(&mut self) {
        self.is_modified = true;
    }

    #[inline(always)]
    pub(super) fn columns_count(&self) -> u32 {
        self.chars.len() as u32
    }

    #[inline(always)]
    pub(super) fn line_number(&self) -> u32 {
        self.line_number
    }

    #[inline(always)]
    pub(super) fn visual_row_count(&self) -> u32 {
        self.wrap_points.len() as u32
    }

    /// Cells for a given visual row, clipped to `max_width`.
    /// `row` must be < `visual_row_count()`.
    pub(super) fn visual_row_chars(&self, row: u32, max_width: u32) -> &[Character] {
        let start = self.wrap_points[row as usize];
        let end = self.wrap_points.get(row as usize + 1).copied().unwrap_or(self.chars.len() as u32);
        let end = end.min(start + max_width);
        if start >= end {
            return &[];
        }
        &self.chars[start as usize..end as usize]
    }

    /// Cells in a horizontal slice of the unwrapped line. Use when wrap is off
    /// and the viewport scrolls horizontally.
    #[inline(always)]
    pub(super) fn unwrapped_chars(&self, start: u32, end: u32) -> &[Character] {
        let len = self.chars.len() as u32;
        let end = end.min(len);
        if start >= end {
            return &[];
        }
        &self.chars[start as usize..end as usize]
    }

    #[inline(always)]
    pub(super) fn reset(&mut self) {
        self.last_col = 0;
        self.chars.clear();
        self.col_map.clear();
        self.wrap_points.clear();
        self.wrap_points.push(0);
        self.line_number = u32::MAX;
        self.is_modified = false;
    }

    // --- Unwrapped-column conversions (highlighter, selection, internal use) ---

    pub(super) fn line_char_index_to_unwrapped_col(&self, lci: LineCharIndex) -> u32 {
        if (lci.0 as usize) < self.col_map.len() {
            self.col_map[lci.0 as usize]
        } else {
            self.last_col
        }
    }
    
    pub(super) fn unwrapped_col_to_line_char_index(&self, column: u32) -> LineCharIndex {
        if self.chars.is_empty() || column >= self.last_col {
            return LineCharIndex(self.chars.len() as u32);
        }
        let idx = self.col_map.partition_point(|&pos| pos <= column).saturating_sub(1);
        LineCharIndex(idx as u32)
    }

    // --- Visual conversions (cursor paint, mouse, vertical movement) ---

    /// Map an unwrapped column to a visual `(row, col_in_row)`.
    pub(super) fn unwrapped_col_to_visual(&self, column: u32) -> VisualPos {
        // wrap_breaks is non-empty and starts with 0, so partition_point >= 1.
        let row = self.wrap_points.partition_point(|&c| c <= column).saturating_sub(1);
        let col_in_row = column - self.wrap_points[row];
        VisualPos { row: row as u32, col_in_row }
    }

    /// Map a visual `(row, col_in_row)` to an unwrapped column.
    /// Clamps `col_in_row` to the row's actual width. Returns None if `row` is OOB.
    pub(super) fn visual_to_unwrapped_col(&self, row: u32, col_in_row: u32) -> Option<u32> {
        let start = *self.wrap_points.get(row as usize)?;
        let row_end = self.wrap_points.get(row as usize + 1).copied().unwrap_or(self.chars.len() as u32);
        // Clamp inside this row so a click past row-end lands at row-end, not the next row.
        let row_width = row_end.saturating_sub(start);
        Some(start + col_in_row.min(row_width))
    }

    /// Convenience: logical char → visual position.
    #[inline]
    pub(super) fn line_char_index_to_visual(&self, lci: LineCharIndex) -> VisualPos {
        let col = self.line_char_index_to_unwrapped_col(lci);
        self.unwrapped_col_to_visual(col)
    }

    /// Convenience: visual position → logical char.
    #[inline]
    pub(super) fn visual_to_line_char_index(&self, row: u32, col_in_row: u32) -> LineCharIndex {
        let col = self.visual_to_unwrapped_col(row, col_in_row).unwrap_or(self.last_col);
        self.unwrapped_col_to_line_char_index(col)
    }
}

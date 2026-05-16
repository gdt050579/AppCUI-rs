use ropey::{Rope, RopeSlice};
use ropey::iter::{Lines, Chars};
use super::LineCharIndex;

pub(super) struct PosInfo {
    pub(super) line_index: u32,
    pub(super) line_char_index: LineCharIndex,
}
pub(super) struct Document {
    rope: ropey::Rope,
}

impl Document {
    pub(super) fn new(text: &str) -> Self {
        Self { rope: Rope::from(text) }
    }
    pub(super) fn lines_count(&self) -> usize {
        self.rope.len_lines()
    }
    pub(super) fn line(&self, line_index: u32) -> RopeSlice<'_> {
        self.rope.line(line_index as usize)
    }
    // pub(super) fn lines_starting_from(&self, line_index: u32) -> Lines<'_> {
    //     self.rope.lines_at(line_index as usize)
    // }
    pub(super) fn line_to_char(&self, line_index: u32) -> usize {
        self.rope.line_to_char(line_index as usize)
    }
    pub(super) fn chars_count(&self) -> usize {
        self.rope.len_chars()
    }
    pub(super) fn char(&self, char_index: usize) -> Option<char> {
        self.rope.get_char(char_index)
    }
    pub(super) fn chars_iter(&self, char_index: usize) -> Chars<'_> {
        self.rope.chars_at(char_index)
    }
    pub(super) fn position_to_line(&self, char_index: usize) -> u32 {
        self.rope.char_to_line(char_index) as u32
    }
    pub(super) fn line_end_position(&self, line: u32) -> usize {
        let ln = self.line(line);
        let start = self.line_to_char(line);
        start + ln.len_chars().saturating_sub(1)
    } 
    pub(super) fn char_to_pos_info(&self, char_index: usize) -> PosInfo {
        let line_index = self.position_to_line(char_index);
        let start = self.rope.line_to_char(line_index as usize);
        debug_assert!(char_index >= start);
        let line_char_index = LineCharIndex::new((char_index - start) as u32);
        PosInfo { line_index, line_char_index }
    }
}
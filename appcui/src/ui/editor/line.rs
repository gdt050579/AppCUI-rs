use super::Document;
use crate::prelude::{CharAttribute, Character};

pub struct Line {
    line_number: u32,
    chars: Vec<Character>,
    char_to_position: Vec<u32>,
}
impl Line {
    pub(super) fn new(initial_capacity: usize) -> Self {
        Self {
            line_number: u32::MAX,
            chars: Vec::with_capacity(initial_capacity),
            char_to_position: Vec::with_capacity(initial_capacity),
        }
    }
    pub(super) fn update(&mut self, line_number: u32, doc: &Document, attr: CharAttribute, tab_align: u32) {
        self.line_number = line_number;
        self.chars.clear();
        self.char_to_position.clear();
        let mut col = 0;

        for c in doc.line(line_number).chars() {
            self.chars.push(Character::with_attributes(if c as u32 >= 32 { c } else { ' ' }, attr));
            self.char_to_position.push(col);
            if c == '\t' {
                col += tab_align - (col % tab_align);
                // add missing spaces
                while col as usize >= self.chars.len() {
                    self.chars.push(Character::with_attributes(' ', attr));
                }
            } else {
                col += 1;
            }
        }
    }
    #[inline(always)]
    pub(super) fn line_number(&self) -> u32 {
        self.line_number
    }
    #[inline(always)]
    pub(super) fn visible_chars(&self, start: u32, end: u32) -> &[Character] {
        let len = self.chars.len() as u32;
        let end = end.min(len);
        if start >= end {
            return &[];
        }
        &self.chars[start as usize..end as usize]
    }
    #[inline(always)]
    pub(super) fn reset(&mut self) {
        self.chars.clear();
        self.char_to_position.clear();
        self.line_number = u32::MAX;
    }
    #[inline(always)]
    pub(super) fn x_offset(&self, char_index: usize) -> Option<u32> {
        self.char_to_position.get(char_index).copied()
    }
}

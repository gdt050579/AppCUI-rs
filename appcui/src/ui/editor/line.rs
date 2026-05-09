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
}

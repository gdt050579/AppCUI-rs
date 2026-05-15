use std::cmp::Ordering;

use super::{Document, Line};
use crate::prelude::{CharAttribute, Size};

pub(super) struct ViewPort {
    lines: Vec<Line>,
    size: Size,
    count: u32,
}
impl ViewPort {
    pub(super) fn new(initial_capacity: usize) -> Self {
        Self {
            lines: Vec::with_capacity(initial_capacity),
            size: Size::new(0, 0),
            count: 0,
        }
    }
    pub(super) fn resize(&mut self, sz: Size) {
        if self.size != sz {
            self.size = sz;
            self.lines.reserve(sz.height.max(8) as usize);
            // allocate space for new lines
            while self.lines.len() < sz.height.max(8) as usize {
                self.lines.push(Line::new(sz.width.max(8) as usize));
            }
        }
    }
    #[inline(always)]
    pub(super) fn size(&self) -> Size {
        self.size
    }
    #[inline(always)]
    pub(super) fn lines(&self) -> &[Line] {
        &self.lines[..self.count as usize]
    }
    pub(super) fn contains_line(&self, line_number: u32) -> bool {
        if self.count == 0 {
            return false;
        }
        line_number >= self.lines[0].line_number() && line_number <= self.lines[self.count as usize - 1].line_number()
    }
    pub(super) fn first_line(&self) -> u32 {
        if self.count == 0 {
            return u32::MAX;
        }
        self.lines[0].line_number()
    }
    pub(super) fn last_line(&self) -> u32 {
        if self.count == 0 {
            return u32::MAX;
        }
        self.lines[self.count as usize - 1].line_number()
    }
    pub(super) fn reset(&mut self) {
        self.count = 0;
    }
    #[inline(always)]
    pub(super) fn update_line(&mut self, index: u32, line_number: u32, doc: &Document, attr: CharAttribute, tab_align: u32) {
        if index >= self.count {
            if index > self.lines.len() as u32 {
                return;
            }
            while self.count <= index {
                self.lines[self.count as usize].reset();
                self.count += 1;
            }
        }
        self.lines[index as usize].update(line_number, doc, attr, tab_align);
    }
    pub(super) fn line_number_to_index(&self, line_number: u32) -> Option<u32> {
        if self.count == 0 {
            return None;
        }
        if line_number < self.lines[0].line_number() {
            return None;
        }
        if line_number > self.lines[self.count as usize - 1].line_number() {
            return None;
        }
        // binary search
        let mut left = 0;
        let mut right = self.count as usize - 1;
        while left <= right {
            let mid = (left + right) / 2;
            let mid_line = self.lines[mid].line_number();
            match mid_line.cmp(&line_number) {
                Ordering::Equal => return Some(mid as u32),
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
            }
        }
        None
    }
    pub(super) fn x_offset(&self, line_number: u32, relative_char_index: usize) -> Option<u32> {
        if let Some(line_index) = self.line_number_to_index(line_number) {
            self.lines[line_index as usize].x_offset(relative_char_index)
        } else {
            None
        }
    }
}

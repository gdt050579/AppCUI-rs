use super::{Line, Document};
use crate::prelude::{Size, CharAttribute};

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
}

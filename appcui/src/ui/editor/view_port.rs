use crate::prelude::Size;
use super::Line;

pub(super) struct ViewPort {
    lines: Vec<Line>,
    size: Size,
}
impl ViewPort {
    pub(super) fn new(initial_capacity: usize) -> Self {
        Self {
            lines: Vec::with_capacity(initial_capacity),
            size: Size::new(0, 0),
        }
    }
    pub(super) fn resize(&mut self,sz: Size) {
        if self.size != sz {
            self.lines.reserve(sz.height.max(8) as usize);
            // allocate space for new lines
            while self.lines.len() < sz.height.max(8) as usize {
                self.lines.push(Line::new(sz.width.max(8) as usize));
            }
        }
    }
}

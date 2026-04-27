use ropey::{Rope, RopeSlice};

pub(super) struct Document {
    repo: ropey::Rope,
}

impl Document {
    pub(super) fn new(text: &str) -> Self {
        Self { repo: Rope::from(text) }
    }
    pub(super) fn lines_count(&self) -> usize {
        self.repo.len_lines()
    }
    pub(super) fn line(&self, index: usize) -> RopeSlice {
        self.repo.line(index)
    }
}
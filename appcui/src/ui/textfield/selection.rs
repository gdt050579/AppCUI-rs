pub(super) struct Selection {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) origin: usize,
}

impl Selection {
    pub(super) const NONE: Selection = Selection {
        start: usize::MAX,
        end: usize::MAX,
        origin: usize::MAX,
    };
    #[inline(always)]
    pub(super) fn is_empty(&self) -> bool {
        self.origin == usize::MAX
    }
    #[inline(always)]
    pub(super) fn update(&mut self, start: usize, end: usize) {
        if self.is_empty() {
            self.origin = start;
            self.end = start.max(end);
            self.start = start.min(end);
        } else {
            self.start = self.origin.min(end);
            self.end = self.origin.max(end);
        }
    }
    #[inline(always)]
    pub(super) fn contains(&self, pos: usize) -> bool {
        (pos >= self.start) && (pos < self.end)
    }
}
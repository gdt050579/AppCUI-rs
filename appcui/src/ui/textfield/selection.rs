pub(crate) struct Selection {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) origin: usize,
}

impl Selection {
    pub(crate) const NONE: Selection = Selection {
        start: usize::MAX,
        end: usize::MAX,
        origin: usize::MAX,
    };
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.origin == usize::MAX
    }
    #[inline(always)]
    pub(crate) fn update(&mut self, start: usize, end: usize) {
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
    pub(crate) fn contains(&self, pos: usize) -> bool {
        (pos >= self.start) && (pos < self.end)
    }
}
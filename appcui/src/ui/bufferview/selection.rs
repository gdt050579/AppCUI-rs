#[derive(Copy, Clone)]
pub(super) struct Selection {
    start: u64,
    end: u64,
    origin: u64,
}

impl Selection {
    pub(super) const NONE: Selection = Selection {
        start: u64::MAX,
        end: u64::MAX,
        origin: u64::MAX,
    };

    #[inline(always)]
    pub(super) fn is_empty(&self) -> bool {
        self.origin == u64::MAX
    }
    #[inline(always)]
    pub(super) fn update(&mut self, anchor: u64, pos: u64) {
        if self.is_empty() {
            self.origin = anchor;
        }
        let (start, end) = if pos >= self.origin {
            (self.origin, pos.saturating_add(1))
        } else {
            (pos, self.origin.saturating_add(1))
        };
        self.start = start;
        self.end = end;
    }

    #[inline(always)]
    pub(super) fn clear(&mut self) {
        *self = Selection::NONE;
    }

    #[inline(always)]
    pub(super) fn range(&self) -> Option<(u64, u64)> {
        if self.is_empty() {
            None
        } else {
            Some((self.start, self.end))
        }
    }
}

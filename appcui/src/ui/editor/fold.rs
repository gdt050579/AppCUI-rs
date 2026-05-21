use std::cmp::Ordering;

const FOLDED_BIT: u32 = 0x8000_0000;
const COUNT_MASK: u32 = 0x7FFF_FFFF;

#[derive(Clone, Copy)]
pub(super) struct Fold {
    start_line: u32,
    packed_count: u32,
}

impl Fold {
    #[inline]
    pub(super) fn new(start_line: u32, count: u32, folded: bool) -> Option<Self> {
        if count == 0 || count >= FOLDED_BIT {
            return None;
        }
        start_line.checked_add(count)?;
        let mut packed = count & COUNT_MASK;
        if folded {
            packed |= FOLDED_BIT;
        }
        Some(Self {
            start_line,
            packed_count: packed,
        })
    }

    #[inline(always)]
    pub(super) fn start_line(&self) -> u32 {
        self.start_line
    }

    #[inline(always)]
    pub(super) fn count(&self) -> u32 {
        self.packed_count & COUNT_MASK
    }

    #[inline(always)]
    pub(super) fn end_line(&self) -> u32 {
        self.start_line + self.count() - 1
    }

    /// Fold header (`start_line`) stays visible; hidden body is `start_line + 1 ..= end_line`.
    #[inline]
    fn hides_line(&self, line: u32) -> bool {
        self.is_folded() && line > self.start_line && line <= self.end_line()
    }

    #[inline(always)]
    pub(super) fn is_folded(&self) -> bool {
        self.packed_count & FOLDED_BIT != 0
    }

    #[inline(always)]
    pub(super) fn set_folded(&mut self, folded: bool) {
        if folded {
            self.packed_count |= FOLDED_BIT;
        } else {
            self.packed_count &= COUNT_MASK;
        }
    }
}

pub(super) struct Folds {
    /// Keyed by `start_line`; iteration order is sorted ascending.
    folds: Vec<Fold>,
}

impl Folds {
    pub(super) fn new() -> Self {
        Self {
            folds: Vec::with_capacity(8),
        }
    }
    pub(super) fn add(&mut self, start_line: u32, count: u32) -> bool {
        let Some(fold) = Fold::new(start_line, count, true) else {
            return false;
        };
        let end_line = fold.end_line();
        let pos = self.folds.partition_point(|f| f.start_line() < start_line);
        if pos < self.folds.len() && self.folds[pos].start_line() == start_line {
            return false;
        }
        for f in self.folds[..pos].iter().rev() {
            match f.end_line().cmp(&end_line) {
                Ordering::Less => {
                    if f.end_line() >= start_line {
                        return false;
                    }
                }
                Ordering::Equal => return false,
                Ordering::Greater => break,
            }
        }
        for f in self.folds[pos..].iter() {
            if f.start_line() > end_line {
                break;
            }
            if f.end_line() >= end_line {
                return false;
            }
        }
        self.folds.insert(pos, fold);
        true
    }

    pub(super) fn folds(&self) -> &[Fold] {
        &self.folds
    }
    pub(super) fn clear(&mut self) {
        self.folds.clear();
    }
    pub(super) fn line_to_fold(&self, line_number: u32) -> Option<Fold> {
        if self.folds.is_empty() {
            return None;
        }
        let pos = self.folds.partition_point(|f| f.start_line() <= line_number);
        for f in self.folds[..pos].iter().rev() {
            if f.end_line() >= line_number {
                return Some(*f);
            }
        }
        None
    }
    /// Returns the line `delta` visible lines after `line` (delta=0 returns the first
    /// visible line >= `line`). Walks forward, skipping folded bodies.
    pub(super) fn next_visible_line(&self, line: u32, delta: u32) -> u32 {
        // Hot path: no folds at all.
        if self.folds.is_empty() {
            return line.saturating_add(delta);
        }

        // Step 1: skip past any folded fold hiding `line`.
        let mut current = line;
        loop {
            let Some(f) = self.line_to_fold(current) else {
                break;
            };
            if f.is_folded() && current > f.start_line() {
                current = f.end_line() + 1;
                continue;
            }
            break;
        }

        // Hot path: no folds at or after `current`.
        let pos = self.folds.partition_point(|f| f.end_line() < current);
        if pos == self.folds.len() {
            return current.saturating_add(delta);
        }

        // Walk forward `delta` visible lines, jumping over folded bodies.
        let mut remaining = delta;
        let mut fold_idx = pos;
        while remaining > 0 {
            // Find the next folded fold whose header is at or after `current`.
            while fold_idx < self.folds.len() && self.folds[fold_idx].end_line() < current {
                fold_idx += 1;
            }
            if fold_idx >= self.folds.len() {
                return current.saturating_add(remaining);
            }
            let f = &self.folds[fold_idx];
            if f.start_line() > current {
                // Gap of (f.start_line - current) visible lines before the next fold.
                let gap = f.start_line() - current;
                if remaining <= gap {
                    return current + remaining;
                }
                remaining -= gap;
                current = f.start_line();
                // Fall through: now standing on the fold header.
            }
            // current == f.start_line(): header counts as visible.
            if remaining == 0 {
                return current;
            }
            if f.is_folded() {
                current = f.end_line() + 1;
            } else {
                current += 1;
            }
            remaining -= 1;
            fold_idx += 1;
        }
        current
    }

    /// Returns the line `delta` visible lines before `line`.
    pub(super) fn previous_visible_line(&self, line: u32, delta: u32) -> u32 {
        if self.folds.is_empty() {
            return line.saturating_sub(delta);
        }

        // If `line` is hidden, snap up to its enclosing fold's header.
        let mut current = line;
        loop {
            let Some(f) = self.line_to_fold(current) else {
                break;
            };
            if f.is_folded() && current > f.start_line() {
                current = f.start_line();
                continue;
            }
            break;
        }

        let pos = self.folds.partition_point(|f| f.start_line() < current);
        if pos == 0 {
            return current.saturating_sub(delta);
        }

        let mut remaining = delta;
        let mut fold_idx = pos;
        while remaining > 0 && current > 0 {
            // Find previous fold ending before `current`.
            while fold_idx > 0 && self.folds[fold_idx - 1].start_line() >= current {
                fold_idx -= 1;
            }
            if fold_idx == 0 {
                return current.saturating_sub(remaining);
            }
            let f = &self.folds[fold_idx - 1];
            // Gap of (current - f.end_line() - 1) visible lines after the fold ends, before current.
            // But if f is folded, only f.start_line() is visible; the body is hidden.
            let visible_end = if f.is_folded() { f.start_line() } else { f.end_line() };
            let gap = current - visible_end - 1;
            if remaining <= gap {
                return current - remaining;
            }
            remaining -= gap + 1; // +1 to step onto visible_end
            current = visible_end;
            fold_idx -= 1;
        }
        current
    }
    pub(super) fn visible_lines_from(&self, from: u32) -> VisibleLineIter<'_> {
        if self.folds.is_empty() {
            return VisibleLineIter {
                iter: FoldsIterator::None,
                next_line: from,
            };
        }
        let mut iter = self.folds.iter().peekable();
        while iter.peek().is_some_and(|f| f.end_line() < from) {
            iter.next();
        }
        // Skip past `from` if it's hidden by a folded enclosing fold
        let mut next_line = from;
        if let Some(f) = self.line_to_fold(from) {
            if f.is_folded() && from > f.start_line() {
                next_line = f.end_line() + 1;
            }
        }
        VisibleLineIter { iter: FoldsIterator::Iterator(&self), next_line }
    }
}

enum FoldsIterator<'a> {
    None,
    Iterator(&'a Folds),
}

pub(super) struct VisibleLineIter<'a> {
    iter: FoldsIterator<'a>,
    next_line: u32,
}

impl<'a> Iterator for VisibleLineIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match &mut self.iter {
            FoldsIterator::None => {
                let line = self.next_line;
                self.next_line = line.saturating_add(1);
                Some(line)
            },
            FoldsIterator::Iterator(folds) => {
                let line = self.next_line;
                self.next_line = folds.next_visible_line(line, 1);
                Some(line)
            }
        }
    }
}

use std::cmp::Ordering;

const FOLDED_BIT: u32 = 0x8000_0000;
const COUNT_MASK: u32 = 0x7FFF_FFFF;

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
                Ordering::Less => if f.end_line() >= start_line { return false; },
                Ordering::Equal => return false,
                Ordering::Greater => break,
            }
        }
        for f in self.folds[pos..].iter() {
            if f.start_line() > end_line { break; }
            if f.end_line() >= end_line { return false; }
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
}

pub(super) struct VisibleLineIter<'a> {
    iter: std::iter::Peekable<std::slice::Iter<'a, Fold>>,
    next_line: u32,
}

impl<'a> Iterator for VisibleLineIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let line = self.next_line;

        while let Some(f) = self.iter.peek() {
            if f.end_line() >= line {
                break;
            }
            self.iter.next();
        }

        if let Some(f) = self.iter.peek() {
            if f.start_line() == line && f.is_folded() {
                self.next_line = f.end_line() + 1;
                self.iter.next();
                return Some(line);
            }
        }

        self.next_line = line + 1;
        Some(line)
    }
}

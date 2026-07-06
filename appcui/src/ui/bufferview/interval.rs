use crate::graphics::CharAttribute;
use flat_string::FlatString;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// A labeled, styled byte range displayed in the interval-name column of a [`super::BufferView`].
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub(super) start: u64,
    pub(super) end: u64,
    pub(super) attr: CharAttribute,
    pub(super) name: FlatString<22>,
}

impl Interval {
    /// Creates an interval covering `size` bytes beginning at `start`.
    ///
    /// `attr` controls how the interval name is rendered, and `name` is shown in the
    /// interval-name column for every byte inside the range.
    pub fn new(start: u64, size: u64, attr: CharAttribute, name: &str) -> Self {
        let end = start.saturating_add(size.saturating_sub(1));
        Self {
            start,
            end,
            attr,
            name: FlatString::from_str(name),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Segment {
    pub(super) start: u64,
    pub(super) end: u64,
    pub(super) index: usize,
}
impl Segment {
    pub(super) fn exists(&self) -> bool {
        self.index != NONE
    }
    #[inline(always)]
    pub(super) fn contains(&self, pos: u64) -> bool {
        self.start <= pos && pos <= self.end
    }
}
impl Default for Segment {
    fn default() -> Self {
        Self { start: 0, end: 0, index: NONE }
    }
}

const NONE: usize = usize::MAX;

pub(super) struct IntervalSet {
    intervals: Vec<Interval>,
    map: Vec<Segment>, // always valid, rebuilt on every add
}

impl IntervalSet {
    pub(super) fn new() -> Self {
        IntervalSet {
            intervals: Vec::new(),
            map: Vec::new(),
        }
    }
    pub(super) fn set(&mut self, intervals: &[Interval]) {
        self.intervals.clear();
        self.map.clear();
        self.intervals.extend_from_slice(intervals);
        self.update_map();
    }

    fn update_map(&mut self) {
        let mut coords: Vec<u64> = Vec::with_capacity(self.intervals.len() * 2);
        for iv in &self.intervals {
            coords.push(iv.start);
            coords.push(iv.end.saturating_add(1));
        }
        coords.sort_unstable();
        coords.dedup();

        let mut by_start: Vec<usize> = (0..self.intervals.len()).collect();
        by_start.sort_unstable_by_key(|&i| self.intervals[i].start);
        let mut next = 0;

        // top = largest start, then smallest end = innermost active interval
        let mut heap: BinaryHeap<(u64, Reverse<u64>, usize)> = BinaryHeap::new();

        let mut map: Vec<Segment> = Vec::new();
        for w in coords.windows(2) {
            let left = w[0];
            let right = w[1];

            while next < by_start.len() && self.intervals[by_start[next]].start <= left {
                let idx = by_start[next];
                let iv = &self.intervals[idx];
                heap.push((iv.start, Reverse(iv.end), idx));
                next += 1;
            }
            while let Some(&(_, Reverse(end), _)) = heap.peek() {
                if end < left {
                    heap.pop();
                } else {
                    break;
                }
            }

            let index = heap.peek().map(|&(_, _, idx)| idx).unwrap_or(NONE);
            if let Some(last) = map.last_mut() {
                if last.index == index && last.end + 1 == left {
                    last.end = right - 1;
                    continue;
                }
            }
            map.push(Segment {
                start: left,
                end: right - 1,
                index,
            });
        }

        self.map = map;
    }

    #[inline(always)]
    pub(super) fn get(&self, index: usize) -> Option<&Interval> {
        self.intervals.get(index)
    }
    pub(super) fn pos_to_segment(&self, pos: u64, length: u64) -> Segment {
        if self.map.is_empty() {
            return Segment { start: pos, end: length.saturating_sub(1), index: NONE };
        }
        let p = self.map.partition_point(|s| s.start <= pos);
        if p > 0 {
            let seg = self.map[p - 1];
            if pos <= seg.end && seg.index != NONE {
                return seg;
            }
        }
        let next_start = self.map.get(p).map(|s| s.start).unwrap_or(length);
        let end = next_start.saturating_sub(1).min(length.saturating_sub(1));
        Segment { start: pos, end, index: NONE }
    }
}

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

const MAX_CHILDREN: usize = 7;
type ArenaIndex = u32;
enum NodeVec {
    Stack { ids: [ArenaIndex; MAX_CHILDREN], len: u8 },
    Heap(Vec<ArenaIndex>),
}

impl NodeVec {
    fn new() -> Self {
        Self::Stack {
            ids: [0; MAX_CHILDREN],
            len: 0,
        }
    }
    fn clear(&mut self) {
        match self {
            NodeVec::Stack { len, .. } => *len = 0u8,
            NodeVec::Heap(items) => items.clear(),
        }
    }
    fn insert(&mut self, index: usize, id: ArenaIndex) {
        match self {
            NodeVec::Stack { ids, len } => {
                if *len < MAX_CHILDREN as u8 {
                    // move the elements to the right
                    if index >= *len as usize {
                        ids[*len as usize] = id;
                    } else {
                        ids.copy_within(index..*len as usize, index + 1);
                        ids[index] = id;
                    }
                    *len += 1;
                } else {
                    // grow the heap
                    let mut new_items = Vec::with_capacity(MAX_CHILDREN * 2);
                    new_items.extend_from_slice(ids);
                    new_items.insert(index, id);
                    *self = NodeVec::Heap(new_items);
                }
            }
            NodeVec::Heap(items) => items.insert(index, id),
        }
    }
    fn remove(&mut self, index: usize) -> ArenaIndex {
        match self {
            NodeVec::Stack { ids, len } => {
                let id = ids[index];
                ids.copy_within(index + 1..*len as usize, index);
                *len -= 1;
                id
            }
            NodeVec::Heap(v) => v.remove(index),
        }
    }
    fn as_slice(&self) -> &[ArenaIndex] {
        match self {
            NodeVec::Stack { ids, len } => &ids[..*len as usize],
            NodeVec::Heap(items) => items.as_slice(),
        }
    }
}

struct Node {
    fold: Fold,
    children: NodeVec,
}

pub(super) struct Folds {
    arena: Vec<Node>,
    root: NodeVec,
}

impl Folds {
    pub(super) fn new() -> Self {
        Self {
            arena: Vec::new(),
            root: NodeVec::new(),
        }
    }

    /// Add a fold. Returns true on success, false on duplicate or partial overlap.
    pub(super) fn add(&mut self, f: Fold) -> bool {
        // Descend to find (parent_children_list, insert_index).
        // parent_children is either self.root or some node's children.
        // We track it via Option<ArenaIndex>: None = root, Some(id) = arena[id].children.
        let mut parent: Option<ArenaIndex> = None;

        let (parent, idx) = loop {
            let kids = match parent {
                None => self.root.as_slice(),
                Some(id) => self.arena[id as usize].children.as_slice(),
            };

            let idx = kids.partition_point(|&c| self.arena[c as usize].fold.start_line() <= f.start_line());

            if idx > 0 {
                let cand_id = kids[idx - 1];
                let cand = self.arena[cand_id as usize].fold;

                if cand.start_line() == f.start_line() && cand.end_line() == f.end_line() {
                    return false; // duplicate
                }
                if cand.start_line() < f.start_line() && f.end_line() < cand.end_line() {
                    parent = Some(cand_id); // descend
                    continue;
                }
                if cand.end_line() >= f.start_line() {
                    return false; // partial overlap with previous sibling
                }
            }

            // Validate sibling at `idx`: must start after f ends, or be fully nested in f
            if let Some(&next_id) = kids.get(idx) {
                let next = self.arena[next_id as usize].fold;
                let contained = f.start_line() < next.start_line() && next.end_line() < f.end_line();
                let after = next.start_line() > f.end_line();
                if !contained && !after {
                    return false;
                }
            }

            break (parent, idx);
        };

        // Find adoption range: contiguous siblings from `idx` that are nested in f
        let kids_slice = match parent {
            None => self.root.as_slice(),
            Some(id) => self.arena[id as usize].children.as_slice(),
        };
        let mut adopt_end = idx;
        while adopt_end < kids_slice.len() {
            let c = self.arena[kids_slice[adopt_end] as usize].fold;
            if f.start_line() < c.start_line() && c.end_line() < f.end_line() {
                adopt_end += 1;
            } else if c.start_line() > f.end_line() {
                break;
            } else {
                return false; // partial overlap
            }
        }

        // Build new node, adopting the contiguous range
        let new_id = self.arena.len() as ArenaIndex;
        let mut new_children = NodeVec::new();
        let adopted: Vec<ArenaIndex> = kids_slice[idx..adopt_end].to_vec();
        for (i, &c) in adopted.iter().enumerate() {
            new_children.insert(i, c);
        }
        self.arena.push(Node {
            fold: f,
            children: new_children,
        });

        // Splice in: remove adopted from parent, insert new_id at idx
        let parent_list = match parent {
            None => &mut self.root,
            Some(id) => &mut self.arena[id as usize].children,
        };
        for _ in idx..adopt_end {
            parent_list.remove(idx);
        }
        parent_list.insert(idx, new_id);

        true
    }

    pub(super) fn line_to_fold(&self, line: u32) -> Option<Fold> {
        let mut kids = self.root.as_slice();
        let mut best: Option<Fold> = None;
        loop {
            let idx = kids.partition_point(|&c| self.arena[c as usize].fold.start_line() <= line);
            if idx == 0 {
                return best;
            }
            let cand = &self.arena[kids[idx - 1] as usize];
            let f = &cand.fold;
            if line > f.end_line() {
                return best;
            }
            best = Some(*f);
            kids = cand.children.as_slice();
        }
    }


    pub(super) fn clear(&mut self) {
        self.arena.clear();
        self.root.clear();
    }

    pub(super) fn folds(&self) -> Vec<Fold> {
        let mut out = Vec::with_capacity(self.arena.len());
        for &id in self.root.as_slice() {
            self.collect_subtree(id, &mut out);
        }
        out
    }

    fn collect_subtree(&self, id: ArenaIndex, out: &mut Vec<Fold>) {
        let node = &self.arena[id as usize];
        out.push(node.fold);
        for &c in node.children.as_slice() {
            self.collect_subtree(c, out);
        }
    }

    /// Iterate visible line numbers >= start_line.
    pub(super) fn visible_lines_from(&self, start_line: u32) -> VisibleLines<'_> {
        VisibleLines {
            folds: self,
            line: start_line,
        }
    }

    /// If `line` is hidden by a folded ancestor, return the line just after that fold's end.
    /// Otherwise return `line`. Descends through the tree following containment.
    fn skip_hidden(&self, mut line: u32) -> u32 {
        'outer: loop {
            let mut kids = self.root.as_slice();
            loop {
                // Find last child with start_line <= line
                let idx = kids.partition_point(|&c| self.arena[c as usize].fold.start_line() <= line);
                if idx == 0 {
                    return line;
                }
                let cand = &self.arena[kids[idx - 1] as usize];
                let f = &cand.fold;

                if line > f.end_line() {
                    return line; // past this fold; siblings start later, done
                }
                // line is within [f.start_line ..= f.end_line]
                if f.hides_line(line) {
                    // Jump past the fold and restart from the top
                    line = f.end_line() + 1;
                    continue 'outer;
                }
                // Visible at this level; descend into its children
                kids = cand.children.as_slice();
            }
        }
    }
}

pub(super) struct VisibleLines<'a> {
    folds: &'a Folds,
    line: u32,
}

impl<'a> Iterator for VisibleLines<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let visible = self.folds.skip_hidden(self.line);
        self.line = visible.checked_add(1)?;
        Some(visible)
    }
}

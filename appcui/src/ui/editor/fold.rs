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

#[derive(Copy, Clone, Eq, PartialEq)]
struct ArenaIndex {
    index: u32,
}
impl ArenaIndex {
    const INVALID: Self = Self { index: u32::MAX };
    #[inline(always)]
    const fn new(index: u32) -> Self {
        Self { index }
    }
    #[inline(always)]
    fn index(&self) -> usize {
        self.index as usize
    }
    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}
enum NodeVec {
    Stack { ids: [ArenaIndex; MAX_CHILDREN], len: u8 },
    Heap(Vec<ArenaIndex>),
}

impl NodeVec {
    fn new() -> Self {
        Self::Stack {
            ids: [ArenaIndex::INVALID; MAX_CHILDREN],
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
    #[inline(always)]
    fn children_count(&self) -> usize {
        match self {
            NodeVec::Stack { len, .. } => *len as usize,
            NodeVec::Heap(items) => items.len(),
        }
    }
    #[inline(always)]
    fn as_slice(&self) -> &[ArenaIndex] {
        match self {
            NodeVec::Stack { ids, len } => &ids[..*len as usize],
            NodeVec::Heap(items) => items.as_slice(),
        }
    }
    #[inline(always)]
    fn index_of(&self, id: ArenaIndex) -> Option<usize> {
        match self {
            NodeVec::Stack { ids, .. } => ids.iter().position(|&i| i == id),
            NodeVec::Heap(items) => items.iter().position(|&i| i == id),
        }
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        match self {
            NodeVec::Stack { len, .. } => *len == 0,
            NodeVec::Heap(items) => items.is_empty(),
        }
    }
}

struct Node {
    fold: Fold,
    children: NodeVec,
    parent: ArenaIndex,
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
                Some(id) => self.arena[id.index()].children.as_slice(),
            };

            let idx = kids.partition_point(|&c| self.arena[c.index()].fold.start_line() <= f.start_line());

            if idx > 0 {
                let cand_id = kids[idx - 1];
                let cand = self.arena[cand_id.index()].fold;

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
                let next = self.arena[next_id.index()].fold;
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
            Some(id) => self.arena[id.index()].children.as_slice(),
        };
        let mut adopt_end = idx;
        while adopt_end < kids_slice.len() {
            let c = self.arena[kids_slice[adopt_end].index()].fold;
            if f.start_line() < c.start_line() && c.end_line() < f.end_line() {
                adopt_end += 1;
            } else if c.start_line() > f.end_line() {
                break;
            } else {
                return false; // partial overlap
            }
        }

        // Build new node, adopting the contiguous range
        let new_id = ArenaIndex::new(self.arena.len() as u32);
        let mut new_children = NodeVec::new();
        let adopted: Vec<ArenaIndex> = kids_slice[idx..adopt_end].to_vec();
        for (i, &c) in adopted.iter().enumerate() {
            new_children.insert(i, c);
        }
        self.arena.push(Node {
            fold: f,
            children: new_children,
            parent: parent.unwrap_or(ArenaIndex::INVALID),
        });

        // Splice in: remove adopted from parent, insert new_id at idx
        let parent_list = match parent {
            None => &mut self.root,
            Some(id) => &mut self.arena[id.index()].children,
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
            let idx = kids.partition_point(|&c| self.arena[c.index()].fold.start_line() <= line);
            if idx == 0 {
                return best;
            }
            let cand = &self.arena[kids[idx - 1].index()];
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

    #[cfg(test)]
    pub(super) fn folds(&self) -> Vec<Fold> {
        let mut out = Vec::with_capacity(self.arena.len());
        for &id in self.root.as_slice() {
            self.collect_subtree(id, &mut out);
        }
        out
    }
    #[cfg(test)]
    fn collect_subtree(&self, id: ArenaIndex, out: &mut Vec<Fold>) {
        let node = &self.arena[id.index()];
        out.push(node.fold);
        for &c in node.children.as_slice() {
            self.collect_subtree(c, out);
        }
    }

    /// Iterate visible line numbers >= start_line.
    pub(super) fn visible_lines_from(&self, start_line: u32) -> VisibleLines<'_> {
        VisibleLines::new(&self, start_line)
    }
    fn line_to_arena_index(&self, line: u32) -> ArenaIndex {
        if self.root.is_empty() {
            return ArenaIndex::INVALID;
        }
        let mut nodes = self.root.as_slice();
        let mut best: ArenaIndex = ArenaIndex::INVALID;
        loop {
            let idx = nodes.partition_point(|&c| self.arena[c.index()].fold.start_line() <= line);
            if idx == 0 {
                return best;
            }
            let arena_id = nodes[idx - 1];
            let node = &self.arena[arena_id.index()];
            if line > node.fold.end_line() {
                return best;
            }
            // linia este in interiorul uuni fold
            best = arena_id;
            nodes = node.children.as_slice();
        }
    }
    fn is_folded(&self, id: ArenaIndex) -> (bool, ArenaIndex) {
        if id == ArenaIndex::INVALID {
            return (false, ArenaIndex::INVALID);
        }
        let mut is_folded = false;
        let mut current = id;
        let mut upper_fold = ArenaIndex::INVALID;
        while current != ArenaIndex::INVALID {
            let node = &self.arena[current.index()];
            if node.fold.is_folded() {
                is_folded = true;
                upper_fold = current;
            }
            current = node.parent;
        }
        (is_folded, upper_fold)
    }
    fn node_vec_interval(&self, line: u32, nodes: &NodeVec, min: u32, max: u32) -> (u32, u32) {
        let len = nodes.as_slice().len();
        if len == 0 {
            return (min, max);
        }
        // find the index where the line should be inserted based on the start_line of the folds
        let nodes = nodes.as_slice();
        let idx = nodes.partition_point(|&c| self.arena[c.index()].fold.start_line() <= line);
        let start = {
            if idx == 0 {
                0
            } else if idx >= len {
                self.arena[nodes[len - 1].index()].fold.end_line().saturating_add(1)
            } else {
                self.arena[nodes[idx - 1].index()].fold.end_line().saturating_add(1)
            }
        };
        let end = {
            if idx == 0 {
                self.arena[nodes[0].index()].fold.start_line().saturating_sub(1)
            } else if idx >= len {
                max
            } else {
                self.arena[nodes[idx].index()].fold.start_line().saturating_sub(1)
            }
        };
        (start.clamp(min, max), end.clamp(min, max))
    }
    /*
        1. daca nu am un fold -> sunt top level -> returnez node_vec_interval
        2. daca am un fold:
            2.1. daca sunt vizibil
                2.1.1 daca am copii - returnez node_vec_interval (cu min/max intervalul din fold)
                2.1.2 daca NU am copii - returnez chiar fold-ul meu
            2.2. daca NU sunt vizibil
                2.2.1 daca nu sunt vizibil din cauza unui parinte - returnz fold-ul parintelui
                2.2.2 daca nu sint vizibil din caza mea
                    2.2.2.1 daca sunt pe prima linie - returnez un fold de o linie (prima) vizibil
                    2.2.2.2 altfel returnz tot fold-ul meu, mai putin pima linia
    */
    fn line_visibility_interval(&self, line: u32) -> (u32, u32, bool) {
        let id = self.line_to_arena_index(line);
        if !id.is_valid() {
            // 1
            let (start, end) = self.node_vec_interval(line, &self.root, 0, u32::MAX);
            (start, end, false)
        } else {
            // 2
            let (is_folded, upper_fold) = self.is_folded(id);
            let node = &self.arena[id.index()];
            if !is_folded {
                // 2.1
                let max = node.fold.end_line();
                let min = node.fold.start_line();
                if node.children.is_empty() {
                    // 2.1.2
                    (min, max, is_folded)
                } else {
                    let (start, end) = self.node_vec_interval(line, &node.children, min, max);
                    (start, end, is_folded)
                }
            } else {
                // 2.2
                if upper_fold != id {
                    // 2.2.1
                    let parent = &self.arena[upper_fold.index()];
                    (parent.fold.start_line().saturating_add(1), parent.fold.end_line(), is_folded)
                } else {
                    // 2.2.2
                    if line == node.fold.start_line() {
                        // 2.2.2.1
                        (line, line, false)
                    } else {
                        // 2.2.2.2
                        (node.fold.start_line().saturating_add(1), node.fold.end_line(), is_folded)
                    }
                }
            }
        }
    }
    fn visible_line_upper_limit(&self, line: u32) -> (u32, u32) {
        let mut ln = line;
        let upper_limit;
        loop {
            let (_, max, folded) = self.line_visibility_interval(ln);
            if folded {
                ln = max.saturating_add(1);
                if ln == u32::MAX {
                    upper_limit = u32::MAX;
                    break;
                }
            } else {
                upper_limit = max;
                break;
            }
        }
        (ln, upper_limit)
    }
    pub(super) fn next_visible_nth_line(&self, from_line: u32, n: u32) -> u32 {
        let (mut ln, mut upper_limit) = self.visible_line_upper_limit(from_line);
        let mut to_process = if ln != from_line { n.saturating_sub(1) } else { n };
        while to_process > 0 {
            let dif = upper_limit - ln;
            if to_process <= dif {
                ln += to_process;
                break;
            } else {
                to_process -= dif + 1;
                let next_start = upper_limit.saturating_add(1);
                let (next_line, next_upper_limit) = self.visible_line_upper_limit(next_start);
                ln = next_line;
                upper_limit = next_upper_limit;
            }
        }
        ln
    }
    fn visible_line_lower_limit(&self, line: u32) -> (u32, u32) {
        let mut ln = line;
        let lower_limit;
        loop {
            let (min, _, folded) = self.line_visibility_interval(ln);
            if folded {
                if min == 0 {
                    ln = 0;
                    lower_limit = 0;
                    break;
                }
                ln = min - 1;
            } else {
                lower_limit = min;
                break;
            }
        }
        (ln, lower_limit)
    }
    pub(super) fn previous_visible_nth_line(&self, from_line: u32, n: u32) -> u32 {
        let (mut ln, mut lower_limit) = self.visible_line_lower_limit(from_line);
        let mut to_process = if ln != from_line { n.saturating_sub(1) } else { n };
        while to_process > 0 {
            let dif = ln - lower_limit;
            if to_process <= dif {
                ln -= to_process;
                break;
            } else {
                to_process -= dif + 1;
                if lower_limit == 0 {
                    return 0;
                }
                let (prev_line, prev_lower_limit) = self.visible_line_lower_limit(lower_limit - 1);
                ln = prev_line;
                lower_limit = prev_lower_limit;
            }
        }
        ln
    }
}

pub(super) struct VisibleLines<'a> {
    folds: &'a Folds,
    line: u32,
    upper_limit: u32,
}
impl<'a> VisibleLines<'a> {
    fn new(folds: &'a Folds, line: u32) -> Self {
        let (line, upper_limit) = folds.visible_line_upper_limit(line);
        Self { folds, line, upper_limit }
    }
}

impl<'a> Iterator for VisibleLines<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.line == u32::MAX {
            return None;
        }
        let res = self.line;
        if self.line < self.upper_limit {
            self.line += 1;
        } else {
            let (line, upper_limit) = self.folds.visible_line_upper_limit(self.line.saturating_add(1));
            self.line = line;
            self.upper_limit = upper_limit;
        }
        Some(res)
    }
}

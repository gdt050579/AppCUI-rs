use crate::prelude::*;
use crate::ui::textfield::selection::Selection;
use crate::ui::textfield::CharClass;

pub(super) const MAX_UNDO_DEPTH: usize = 200;

#[derive(Clone)]
pub(super) enum UndoOp {
    Insert {
        pos: usize,
        chars: Vec<Character>,
    },
    Delete {
        pos: usize,
        chars: Vec<Character>,
    },
    Replace {
        pos: usize,
        old_chars: Vec<Character>,
        new_chars: Vec<Character>,
    },
}

#[derive(Clone)]
pub(super) struct UndoEntry {
    pub op: UndoOp,
    pub cursor_before: usize,
    pub cursor_after: usize,
    pub selection_before: Selection,
    pub selection_after: Selection,
}

#[derive(Clone, PartialEq)]
pub(super) enum LastAction {
    None,
    AddChar(CharClass),
    Delete,
    Other,
}

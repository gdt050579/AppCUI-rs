use flat_string::FlatString;

use crate::prelude::Symbol;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Type {
    Standard,
    Ascii,
    Checkbox,
    CheckMark,
}

impl Type {
    pub(super) fn check_symbol(&self) -> &str {
        match self {
            Type::Standard => "[X]",
            Type::Ascii => "[*]",
            Type::Checkbox => "☑",
            Type::CheckMark => "✓",
        }
    }
    pub(super) fn un_check_symbol(&self) -> &str {
        match self {
            Type::Standard => "[ ]",
            Type::Ascii => "[ ]",
            Type::Checkbox => "☐",
            Type::CheckMark => "x",
        }
    }
}
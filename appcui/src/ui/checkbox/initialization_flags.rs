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
            Type::Standard => "[\u{221A}]",
            Type::Ascii => "[ ]",
            Type::Checkbox => "☑",
            Type::CheckMark => "✓",
        }
    }
    pub(super) fn uncheck_symbol(&self) -> &str {
        match self {
            Type::Standard => "[ ]",
            Type::Ascii => "[ ]",
            Type::Checkbox => "☐",
            Type::CheckMark => "x",
        }
    }
}
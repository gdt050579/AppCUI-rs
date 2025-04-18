#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Type {
    Standard,
    Ascii,
    CheckBox,
    CheckMark,
}

impl Type {
    pub(super) fn check_symbol(&self) -> &str {
        match self {
            Type::Standard => "[\u{221A}]",
            Type::Ascii => "[X]",
            Type::CheckBox => "☑",
            Type::CheckMark => "\u{221A}",
        }
    }
    pub(super) fn uncheck_symbol(&self) -> &str {
        match self {
            Type::Standard => "[ ]",
            Type::Ascii => "[ ]",
            Type::CheckBox => "☐",
            Type::CheckMark => "x",
        }
    }
}
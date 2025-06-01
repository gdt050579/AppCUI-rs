#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
    Checked,
    Unchecked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Standard,
    Ascii,
    CheckBox,
    CheckMark,
    FilledBox,
    YesNo,
    PlusMinus,
}

impl Type {
    pub fn check_symbol(&self) -> &str {
        match self {
            Type::Standard => "[\u{221A}]",
            Type::Ascii => "[X]",
            Type::CheckBox => "☑ ",
            Type::CheckMark => "\u{221A}",
            Type::FilledBox => "▣",
            Type::YesNo => "[Y]",
            Type::PlusMinus => "+",
        }
    }

    pub fn uncheck_symbol(&self) -> &str {
        match self {
            Type::Standard => "[ ]",
            Type::Ascii => "[ ]",
            Type::CheckBox => "☐ ",
            Type::CheckMark => "x",
            Type::FilledBox => "▢",
            Type::YesNo => "[N]",
            Type::PlusMinus => "-",
        }
    }

    pub fn unknown_symbol(&self) -> &str {
        match self {
            Type::Standard => "[?]",
            Type::Ascii => "[?]",
            Type::CheckBox => "⍰ ", 
            Type::CheckMark => "?",
            Type::FilledBox => "◪",
            Type::YesNo => "[?]",
            Type::PlusMinus => "±",
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Type {
    Standard,
    Ascii,
    CheckBox,
    CheckMark,
    FilledBox,
    YesNo,
    PlusMinus,
}
// ✅ 🔲 🗹 🞎 🞏 🞕
impl Type {
    pub(super) fn check_symbol(&self) -> &str {
        match self {
            Type::Standard => "[\u{221A}]",
            Type::Ascii => "[X]",
            Type::CheckBox => "🗹 ",
            Type::CheckMark => "\u{221A}",
            Type::FilledBox => "🞕 ",
            Type::YesNo => "[Y]",
            Type::PlusMinus => "➕ ",
        }
    }
    pub(super) fn uncheck_symbol(&self) -> &str {
        match self {
            Type::Standard => "[ ]",
            Type::Ascii => "[ ]",
            Type::CheckBox => "🞎 ",
            Type::CheckMark => "x",
            Type::FilledBox => "🞏 ",
            Type::YesNo => "[N]",
            Type::PlusMinus => "➖ ",
        }
    }
}
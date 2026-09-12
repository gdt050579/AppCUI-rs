#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// The three possible values of a [`super::ThreeStateBox`].
///
/// Cycles between checked, unchecked, and an indeterminate (unknown) state.
pub enum State {
    /// The box is checked (for example `[√]` or `[X]`, depending on [`Type`]).
    Checked,
    /// The box is empty (for example `[ ]`).
    Unchecked,
    /// The box is indeterminate (for example `[?]`).
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Visual style of a [`super::ThreeStateBox`] check mark.
///
/// Variants choose the glyphs used for checked, unchecked, and unknown states.
pub enum Type {
    /// Boxed marks: `[√]` checked, `[ ]` unchecked, `[?]` unknown.
    Standard,
    /// ASCII boxed marks: `[X]` checked, `[ ]` unchecked, `[?]` unknown.
    Ascii,
    /// Unicode checkboxes: `☑` checked, `☐` unchecked, `⍰` unknown.
    CheckBox,
    /// Bare marks: `√` checked, `x` unchecked, `?` unknown.
    CheckMark,
    /// Geometric boxes: `▣` checked, `▢` unchecked, `◪` unknown.
    FilledBox,
    /// Letter boxes: `[Y]` checked, `[N]` unchecked, `[?]` unknown.
    YesNo,
    /// Arithmetic marks: `+` checked, `-` unchecked, `±` unknown.
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

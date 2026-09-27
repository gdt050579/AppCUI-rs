#[derive(Copy, Clone, Eq, PartialEq)]
/// How a boolean list-item value is rendered as text.
///
/// Choose True/False, Yes/No, or compact glyphs such as `X`/`-` or a check mark.
pub enum BoolFormat {
    /// The words `True` / `False`.
    TrueFalse,
    /// The words `Yes` / `No`.
    YesNo,
    /// Compact `X` / `-`.
    XMinus,
    /// Compact `√` / `-`.
    CheckmarkMinus,
}

impl BoolFormat {
    pub(crate) fn text(&self, value: bool) -> &'static str {
        match self {
            BoolFormat::TrueFalse => {
                if value {
                    "True"
                } else {
                    "False"
                }
            }
            BoolFormat::YesNo => {
                if value {
                    "Yes"
                } else {
                    "No"
                }
            }
            BoolFormat::XMinus => {
                if value {
                    "X"
                } else {
                    "-"
                }
            }
            BoolFormat::CheckmarkMinus => {
                if value {
                    "\u{221A}"
                } else {
                    "-"
                }
            }
        }
    }
}

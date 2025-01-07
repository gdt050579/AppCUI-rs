#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoolFormat {
    TrueFalse,
    YesNo,
    XMinus,
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

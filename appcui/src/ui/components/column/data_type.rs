#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoolFormat {
    TrueFalse,
    YesNo,
    OnOff,
    CheckBox,
    CheckMark,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Type {
    String,
    Bool(BoolFormat),
}

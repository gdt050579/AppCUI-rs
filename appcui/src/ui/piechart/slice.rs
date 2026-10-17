use crate::graphics::{CharAttribute, Color};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Slice {
    pub(crate) value: u64,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) percent: u16,
    pub(crate) color: Color,
}

impl Slice {
    pub(crate) fn new(value: u64, label: &str, attr: Option<CharAttribute>) -> Self {
        Self {
            value,
            label: String::from(label),
            attr,
            start: 0,
            end: 0,
            percent: 0,
            color: Color::Black,
        }
    }
}

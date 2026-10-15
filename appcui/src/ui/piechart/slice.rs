use crate::graphics::CharAttribute;

/// One value of a [`PieChart`](super::PieChart): its value, label and optional attribute.
///
/// An empty `label` means the slice has no label. An `attr` of `None` means the
/// slice is drawn with an automatic color.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Slice {
    pub(crate) value: u64,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
}

impl Slice {
    pub(crate) fn new(value: u64, label: &str, attr: Option<CharAttribute>) -> Self {
        Self {
            value,
            label: String::from(label),
            attr,
        }
    }
}

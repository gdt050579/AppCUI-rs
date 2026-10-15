use crate::graphics::CharAttribute;

/// One value of a [`PieChart`](super::PieChart): its value, label and optional attribute,
/// plus the proportions computed from it.
///
/// An empty `label` means the slice has no label. An `attr` of `None` means the
/// slice is drawn with an automatic color.
///
/// `start` and `end` are cumulative positions in the range `0..=FULL`
/// (the whole chart). A slice with value `0` has `start == end`. `percent` is the
/// slice's share of the total in hundredths of a percent (`0..=10000`); the
/// percents of all slices always add up to exactly `10000` (or `0` if the total is `0`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Slice {
    pub(crate) value: u64,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) percent: u16,
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
        }
    }
}

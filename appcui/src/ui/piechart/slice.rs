use crate::graphics::{CharAttribute, Color, Point};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Slice {
    pub(crate) value: u64,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) percent: u16,
    pub(crate) color: Color,
    // a cell (control coordinates) that shows this slice, at (or near) the average of all its cells
    pub(crate) center: Option<Point>,
    pub(crate) sum_x: u64,
    pub(crate) sum_y: u64,
    pub(crate) cells: u64,
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
            center: None,
            sum_x: 0,
            sum_y: 0,
            cells: 0,
        }
    }
}

use crate::graphics::{CharAttribute, Color, Point};
use crate::system::Theme;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Slice {
    pub(crate) value: u64,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) percent: u16,
    // a cell (control coordinates) that shows this slice, at (or near) the average of all its cells
    pub(crate) center: Option<Point>,
    pub(crate) sum_x: u64,
    pub(crate) sum_y: u64,
    pub(crate) cells: u64,
    pub(crate) text: String,
    pub(crate) tooltip: String,
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
            center: None,
            sum_x: 0,
            sum_y: 0,
            cells: 0,
            text: String::new(),
            tooltip: String::new(),
        }
    }

    pub(crate) fn color(&self, index: usize, theme: &Theme) -> Color {
        match self.attr {
            Some(attr) => attr.foreground,
            None => theme.chart.bar[index % 4].foreground,
        }
    }
}

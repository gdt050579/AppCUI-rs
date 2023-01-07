use super::LayoutMode;

pub struct Layout<'a> {
    format: &'a str,
}

impl Layout<'_> {
    pub fn new(format: &str) -> Layout {
        Layout { format: format }
    }
}

pub(crate) struct ControlLayout {
    mode: LayoutMode,
    x: i32,
    y: i32,
    width: u16,
    height: u16,
    min_width: u16,
    max_width: u16,
    min_height: u16,
    max_height: u16,
}

impl ControlLayout {
    pub(crate) fn new(format: &str) -> ControlLayout {
        ControlLayout {
            mode: LayoutMode::new(format),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            min_width: 1,
            min_height: 1,
            max_width: u16::MAX,
            max_height: u16::MAX,
        }
    }
}

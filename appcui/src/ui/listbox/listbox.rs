use AppCUIProcMacro::*;

use super::Flags;
use super::Item;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct ListBox {
    items: Vec<Item>,
    flags: Flags,
    start_view: usize,
}
impl ListBox {
    pub fn new(layout: Layout, flags: Flags)->Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }
        Self {
            base: ControlBase::with_status_flags(layout, status_flags), 
            items: Vec::new(),
            start_view: 0,
            flags
        }
    }
    pub fn add(&mut self, value: &str) {
        self.items.push(Item::new(value));
    }
}
impl OnPaint for ListBox {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}


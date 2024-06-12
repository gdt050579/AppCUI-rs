use AppCUIProcMacro::*;

use super::Flags;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct ListBox {
    flags: Flags
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
            flags
        }
    }
}
impl OnPaint for ListBox {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}


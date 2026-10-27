use super::{Flags, SegmentFillType, SortType};
use appcui_proc_macro::CustomControl;

struct Segment {
    x: i32,
    value: u64,
    len: u16,
    label: String,
    char: SegmentFillType,
    attr: Option<CharAttribute>,
}

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct UsageChart {
    flags: Flags,
    segments: Vec<Segment>,
    surface: Surface,
    sort: SortType,
}
impl UsageChart {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::new(layout, false),
            segments: Vec::new(),
            surface: Surface::new(1,1),
            sort: SortType::None,
            flags,
        }
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        
    }
}
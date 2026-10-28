use super::{Flags, SegmentFillType, SortType};
use appcui_proc_macro::CustomControl;

struct Segment {
    order: u16,
    x: i32,
    value: u64,
    len: u16,
    label: String,
    char: SegmentFillType,
    attr: Option<CharAttribute>,
}

#[CustomControl(overwrite = OnPaint + OnResize, internal = true)]
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
    fn recompite_positions(&mut self) {
        let width = self.surface.size().width as u128;
        if width == 0 {
            return;
        }
        let mut total_value = 0u128;
        for segment in &self.segments {
            total_value += segment.value as u128;
        }
        // sort from biggest to smallest
        self.segments.sort_by_key(|segment| u64::MAX - segment.value);
        let mut sz = 0;
        for segment in &mut self.segments {
            if sz >= width {
                segment.len = 0;
                continue;
            }
            let mut l = segment.value as u128 * width / total_value;
            if (l == 0) && (sz < width) {
                l = 1;
            }
            segment.len = l as u16;
            sz += l;
        }
        // resort based on sort type
        match self.sort {
            SortType::None => self.segments.sort_by_key(|segment| segment.order),
            SortType::HighestToLowest => self.segments.sort_by_key(|segment| u64::MAX - segment.value),
            SortType::LowestToHighest => self.segments.sort_by_key(|segment| segment.value),
            SortType::Alphabetical => self.segments.sort_by(|a, b| a.label.cmp(&b.label)),
            SortType::ReverseAlphabetical => self.segments.sort_by(|a, b| b.label.cmp(&a.label)),
        }
        // compute the x position
        let mut x = 0;
        for segment in &mut self.segments {
            segment.x = x;
            x += segment.len as i32;
        }
    }
    fn update_surface_content(&mut self) {
        self.recompite_positions();
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        
    }
}
impl OnResize for UsageChart {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_surface_content();
    }
}
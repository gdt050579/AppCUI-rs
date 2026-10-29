use super::{Flags, SegmentFillType, SortType, TotalUsage, Segment};
use appcui_proc_macro::CustomControl;

#[CustomControl(overwrite = OnPaint + OnResize, internal = true)]
pub struct UsageChart {
    flags: Flags,
    segments: Vec<Segment>,
    surface: Surface,
    sort: SortType,
    total_usage: TotalUsage
}
impl UsageChart {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::new(layout, false),
            segments: Vec::new(),
            surface: Surface::new(1,1),
            sort: SortType::None,
            flags,
            total_usage: TotalUsage::Auto,
        }
    }
    pub fn add_segment<T: Into<Segment>>(&mut self, segment: T) {
        self.segments.push(segment.into());
        self.update_surface_content();
    }
    pub fn set_total_usage(&mut self, total_usage: TotalUsage) {
        self.total_usage = total_usage;
        self.update_surface_content();
    }
    fn recompite_positions(&mut self) {
        let width = self.surface.size().width as u128;
        if width == 0 {
            return;
        }
        let total_value = match self.total_usage {
            TotalUsage::Auto => {
                let mut v = 0u128;
                for segment in &self.segments {
                    v += segment.value as u128;
                }
                v
            },
            TotalUsage::Fixed(value) => value as u128,
        };

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
        let mut poz = 0;
        for segment in &mut self.segments {
            segment.poz = poz;
            poz += segment.len as i32;
        }
    }
    fn update_surface_content(&mut self) {
        self.recompite_positions();
        self.surface.clear(Character::with_attributes(' ', self.theme().chart.background));
        let h = self.surface.size().height as u16;
        let attr_list = self.theme().chart.bar;
        let mut attr_index = 0;
        for segment in &self.segments {
            if segment.len == 0 {
                continue;
            }
            let r = Rect::with_size(segment.poz, 0, segment.len, h);
            let ch = match segment.char {
                SegmentFillType::Solid => '█',
                SegmentFillType::Shade75 => '▓',
                SegmentFillType::Shade50 => '▒',
                SegmentFillType::Shade25 => '░',
                SegmentFillType::Braille => '⣿',
                SegmentFillType::Checkerboard => '⣿',
                SegmentFillType::Grid => '┼',
                SegmentFillType::GridDouble => '╬',
                SegmentFillType::CrossHatch => '╳',
                SegmentFillType::Dashed => '┇',
                SegmentFillType::DiagonalUp => '╱',
                SegmentFillType::DiagonalDown => '╲',
                SegmentFillType::Notched => '⣿',
                SegmentFillType::Custom(ch) => ch,
            };
            let attr = segment.attr.unwrap_or(attr_list[attr_index]);
            attr_index = (attr_index + 1) % attr_list.len();
            self.surface.fill_rect(r, Character::with_attributes(ch, attr));
        }
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}
impl OnResize for UsageChart {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_surface_content();
    }
}
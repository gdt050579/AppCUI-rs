use super::{Flags, Segment, SegmentFillType, SortType, TotalUsage};
use appcui_proc_macro::CustomControl;

#[CustomControl(overwrite = OnPaint + OnResize + OnMouseEvent, internal = true)]
pub struct UsageChart {
    flags: Flags,
    segments: Vec<Segment>,
    surface: Surface,
    sort: SortType,
    total_usage: TotalUsage,
    hovered_segment: Option<usize>,
    selected_segment: Option<usize>,
}
impl UsageChart {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut me = Self {
            base: ControlBase::new(layout, true),
            segments: Vec::new(),
            surface: Surface::new(1, 1),
            sort: SortType::None,
            flags,
            total_usage: TotalUsage::Auto,
            hovered_segment: None,
            selected_segment: None,
        };
        if flags.contains(Flags::Margins) {
            me.set_size_bounds(3, 3, u16::MAX, u16::MAX);
        } else {
            me.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        };
        me.update_surface_content();
        me
    }
    pub fn add_segment<T: Into<Segment>>(&mut self, segment: T) {
        self.segments.push(segment.into());
        self.update_surface_content();
    }
    pub fn set_total_usage(&mut self, total_usage: TotalUsage) {
        self.total_usage = total_usage;
        self.update_surface_content();
    }
    fn recompute_positions(&mut self) {
        let width = (self.surface.size().width as u128).saturating_sub(if self.flags.contains(Flags::Margins) { 2 } else { 0 });
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
            }
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
        let mut poz = if self.flags.contains(Flags::Margins) { 1 } else { 0 };
        for segment in &mut self.segments {
            segment.poz = poz;
            poz += segment.len as i32;
        }
    }
    fn update_surface_content(&mut self) {
        self.hovered_segment = None;
        self.recompute_positions();
        self.surface.clear(Character::with_attributes(' ', self.theme().chart.background));
        let h = (self.surface.size().height as u16).saturating_sub(if self.flags.contains(Flags::Margins) { 2 } else { 0 });
        let attr_list = self.theme().chart.bar;
        let y = if self.flags.contains(Flags::Margins) { 1 } else { 0 };
        let mut attr_index = 0;
        for segment in &self.segments {
            if segment.len == 0 {
                continue;
            }
            let r = Rect::with_size(segment.poz, y, segment.len, h);
            let ch = segment.fill_type.as_char();
            let attr = segment.attr.unwrap_or(attr_list[attr_index]);
            attr_index = (attr_index + 1) % attr_list.len();
            self.surface.fill_rect(r, Character::with_attributes(ch, attr));
        }
    }
    fn pos_to_segment_index(&self, x: i32, cached_idx: Option<usize>) -> Option<usize> {
        if let Some(idx) = cached_idx {
            let seg = &self.segments[idx];
            let (start, end) = seg.range();
            if (x >= start) && (x < end) {
                return Some(idx);
            }
        }
        for (idx, seg) in self.segments.iter().enumerate() {
            if x < seg.poz {
                break;
            }
            let (start, end) = seg.range();
            if (x >= start) && (x < end) {
                return Some(idx);
            }
        }
        None
    }
    fn update_tooltip(&self) {
        if let Some(idx) = self.hovered_segment {
            let seg = &self.segments[idx];
            let tooltip = if seg.label.is_empty() {
                format!("{}", seg.value)
            } else {
                format!("{}: {}", seg.label, seg.value)
            };
            self.show_tooltip_on_point(&tooltip, seg.poz + seg.len as i32 / 2, self.surface.size().height as i32 / 2);
        } else {
            self.hide_tooltip();
        }
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        } else if self.has_focus() {
            if let Some(idx) = self.selected_segment {
                let seg = &self.segments[idx];
                let (start, end) = seg.range();
                let r = Rect::new(start.saturating_sub(1), 0, end, (self.size().height as i32).saturating_sub(1));
                if self.flags.contains(Flags::DimSegmentsOnSelection) {
                    let attr = theme.chart.inactive;
                    surface.draw_surface_with_transform(0, 0, &self.surface, |ch, p| {
                        if (p.x < start) || (p.x >= end) {
                            Some(Character::with_attributes(ch.code, attr))
                        } else {
                            Some(ch)
                        }
                    });
                } else {
                    surface.draw_surface(0, 0, &self.surface);
                }
                surface.draw_rect(r, LineType::Single, theme.chart.selection_border);
            } else {
                surface.draw_surface(0, 0, &self.surface);
            }
        } else {
            surface.draw_surface(0, 0, &self.surface);
        }
    }
}
impl OnResize for UsageChart {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_surface_content();
    }
}
impl OnMouseEvent for UsageChart {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.hovered_segment = None;
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let idx = self.pos_to_segment_index(point.x, self.hovered_segment);
                if idx != self.hovered_segment {
                    self.hovered_segment = idx;
                    self.update_tooltip();
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(ed) => {
                let idx = self.pos_to_segment_index(ed.x, self.selected_segment);
                if idx != self.selected_segment {
                    self.selected_segment = idx;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

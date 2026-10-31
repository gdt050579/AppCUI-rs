use super::events::EventData;
use super::{Flags, LabelContent, Segment, SortType, TotalUsage};
use crate::utils::FormatNumber;
use appcui_proc_macro::CustomControl;

// hundredths of a percent, so two decimals are exact (see the pie chart)
const PERCENTAGE_FORMAT: FormatNumber = FormatNumber::new(10).decimals(2).suffix("%");

#[CustomControl(overwrite = OnPaint + OnResize + OnMouseEvent + OnKeyPressed, internal = true)]
pub struct UsageChart {
    pub(super) flags: Flags,
    pub(super) segments: Vec<Segment>,
    surface: Surface,
    pub(super) sort: SortType,
    pub(super) total_usage: TotalUsage,
    pub(super) number_format: FormatNumber,
    pub(super) label_content: LabelContent,
    hovered_segment: Option<usize>,
    pub(super) selected_segment: Option<usize>,
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
            number_format: FormatNumber::new(10).group(3, b','),
            label_content: LabelContent::LabelAndValue,
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
    /// Appends a segment and repaints the chart.
    ///
    /// The segment may be a `u64` (also `u8`, `u16` or `u32`) or a [`Segment`].
    pub fn add_segment<T: Into<Segment>>(&mut self, segment: T) {
        self.add_segments([segment]);
    }
    /// Appends several segments and repaints the chart once.
    ///
    /// Each item may be a `u64` (also `u8`, `u16` or `u32`) or a [`Segment`].
    /// [`SortType::None`](super::SortType::None) keeps the order of this list.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
    /// chart.add_segments([10u64, 20, 30]);
    /// ```
    pub fn add_segments<T, I>(&mut self, segments: I)
    where
        T: Into<Segment>,
        I: IntoIterator<Item = T>,
    {
        let start = self.segments.len();
        self.segments.extend(segments.into_iter().enumerate().map(|(index, segment)| {
            let mut segment = segment.into();
            segment.order = (start + index) as u16;
            segment
        }));
        self.update_surface_content();
    }
    /// Removes all segments from the chart and repaints it.
    pub fn clear(&mut self) {
        self.segments.clear();
        self.hovered_segment = None;
        self.selected_segment = None;
        self.hide_tooltip();
        self.update_surface_content();
    }
    /// Sets the chart flags and repaints it.
    pub fn set_flags(&mut self, flags: Flags) {
        if self.flags == flags {
            return;
        }
        self.flags = flags;
        if flags.contains(Flags::Margins) {
            self.set_size_bounds(3, 3, u16::MAX, u16::MAX);
        } else {
            self.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        }
        self.update_surface_content();
    }
    pub fn set_total_usage(&mut self, total_usage: TotalUsage) {
        self.total_usage = total_usage;
        self.update_surface_content();
    }
    pub fn set_sort_type(&mut self, sort_type: SortType) {
        self.sort = sort_type;
        self.update_surface_content();
    }
    /// Sets how segment values are written, including in hover tooltips.
    ///
    /// The default groups digits by thousands (`1,234,567`).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
    /// chart.set_number_format(FormatNumber::new(10).group(3, b',').suffix(" GB"));
    /// ```
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
        self.update_tooltip();
    }
    /// Sets what the hover tooltip of a segment shows.
    ///
    /// The default is [`LabelContent::LabelAndValue`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
    /// chart.set_label_content(usagechart::LabelContent::Percentage);
    /// ```
    pub fn set_label_content(&mut self, content: LabelContent) {
        self.label_content = content;
        self.update_tooltip();
    }
    /// Returns what the hover tooltip of a segment shows.
    pub fn label_content(&self) -> LabelContent {
        self.label_content
    }
    pub fn set_selected_segment(&mut self, index: Option<usize>) {
        self.select_segment_internal(index, false);
    }
    fn recompute_positions(&mut self) {
        let width = (self.surface.size().width as u128).saturating_sub(if self.flags.contains(Flags::Margins) { 2 } else { 0 });
        if width == 0 {
            return;
        }
        let total_value = self.total_value();

        // sort from biggest to smallest
        self.segments.sort_by_key(|segment| u64::MAX - segment.value);
        let mut sz = 0;
        for segment in &mut self.segments {
            if sz >= width {
                segment.len = 0;
                continue;
            }
            // a total of zero (no values, or a fixed total of 0) leaves every segment empty
            let mut l = if total_value == 0 {
                0
            } else {
                segment.value as u128 * width / total_value
            };
            if (l == 0) && (total_value > 0) && (sz < width) {
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
    fn total_value(&self) -> u128 {
        match self.total_usage {
            TotalUsage::Auto => self.segments.iter().map(|segment| segment.value as u128).sum(),
            TotalUsage::Fixed(value) => value as u128,
        }
    }
    fn percentage_text<'a>(&self, value: u64, buffer: &'a mut [u8]) -> &'a str {
        let total = self.total_value();
        let percent = if total == 0 {
            0
        } else {
            u64::try_from(value as u128 * 10_000 / total).unwrap_or(u64::MAX)
        };
        PERCENTAGE_FORMAT.write_fraction(percent, 100u64, buffer).unwrap_or("")
    }
    fn update_tooltip(&self) {
        let Some(idx) = self.hovered_segment else {
            self.hide_tooltip();
            return;
        };
        let seg = &self.segments[idx];
        let mut value_buffer = [0u8; 96];
        let mut percentage_buffer = [0u8; 32];
        let value = self.number_format.write_number(seg.value, &mut value_buffer).unwrap_or("");
        let percentage = self.percentage_text(seg.value, &mut percentage_buffer);
        let tooltip = match self.label_content {
            LabelContent::Label => seg.label.clone(),
            LabelContent::Value => value.to_string(),
            LabelContent::LabelAndValue => {
                if seg.label.is_empty() {
                    value.to_string()
                } else {
                    format!("{}: {value}", seg.label)
                }
            }
            LabelContent::Percentage => percentage.to_string(),
            LabelContent::LabelAndPercentage => {
                if seg.label.is_empty() {
                    percentage.to_string()
                } else {
                    format!("{}: {percentage}", seg.label)
                }
            }
        };
        if tooltip.is_empty() {
            self.hide_tooltip();
            return;
        }
        self.show_tooltip_on_point(&tooltip, seg.poz + seg.len as i32 / 2, self.surface.size().height as i32 / 2);
    }
    fn raise_selection_event(&self, index: Option<usize>) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::UsageChart(EventData {
                index: index.map(|value| value as u32),
            }),
        });
    }
    fn select_segment_internal(&mut self, index: Option<usize>, raise_event: bool) -> bool {
        if self.selected_segment == index {
            return false;
        }
        self.selected_segment = index;
        if raise_event {
            self.raise_selection_event(index);
        }
        true
    }
    fn goto_segment_by(&mut self, forward: bool) {
        if self.segments.is_empty() {
            self.select_segment_internal(None, true);
            return;
        }
        let mut pos = if let Some(idx) = self.selected_segment {
            idx
        } else {
            if forward {
                // so that next will be 0
                self.segments.len().saturating_sub(1)
            } else {
                // so the previous will be the last
                0
            }
        };
        let mut count = 0;
        loop {
            if forward {
                pos += 1;
                if pos >= self.segments.len() {
                    pos = 0;
                }
            } else {
                if pos == 0 {
                    pos = self.segments.len().saturating_sub(1);
                } else {
                    pos -= 1;
                }
            };
            if self.segments[pos].len > 0 {
                break;
            }
            count += 1;
            if count >= self.segments.len() {
                self.select_segment_internal(None, true);
                return;
            }
        }
        self.select_segment_internal(Some(pos), true);
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
            return;
        }
        // the selection stays visible after the chart loses the focus, as in the bar and line charts
        let Some(idx) = self.selected_segment else {
            surface.draw_surface(0, 0, &self.surface);
            return;
        };
        let seg = &self.segments[idx];
        let (start, end) = seg.range();
        let sz = self.size();
        let right_most = ((sz.width as i32) - 1).max(0);
        let bottom_most = ((sz.height as i32) - 1).max(0);
        let r = Rect::new((start - 1).max(0), 0, end.min(right_most), bottom_most);
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
        match r.height() {
            0 => {}
            1 => {
                surface.write_char(r.left(), 0, Character::with_attributes('[', theme.chart.selection_border));
                surface.write_char(r.right(), 0, Character::with_attributes(']', theme.chart.selection_border));
            }
            2 => {
                surface.write_char(r.left(), 0, Character::with_attributes('┌', theme.chart.selection_border));
                surface.write_char(r.left(), 1, Character::with_attributes('└', theme.chart.selection_border));
                surface.write_char(r.right(), 0, Character::with_attributes('┐', theme.chart.selection_border));
                surface.write_char(r.right(), 1, Character::with_attributes('┘', theme.chart.selection_border));
            }
            _ => {
                surface.draw_rect(r, LineType::Single, theme.chart.selection_border);
            }
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
                if self.select_segment_internal(idx, true) {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnKeyPressed for UsageChart {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        match key.code {
            KeyCode::Left => {
                self.goto_segment_by(false);
                EventProcessStatus::Processed
            }
            KeyCode::Right => {
                self.goto_segment_by(true);
                EventProcessStatus::Processed
            }
            KeyCode::Escape => {
                if self.select_segment_internal(None, true) {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

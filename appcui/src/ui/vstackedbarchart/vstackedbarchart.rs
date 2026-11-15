use crate::prelude::*;
use super::mapping::ValueMapping;
use super::stackbar::StackWithLayout;
use super::{BarsStack, Flags, StackBarDrawMode, StackBarScale, XAxisLabelFormat, XAxisLabelMode};
use crate::ui::components::BarSpan;
use crate::system::RuntimeManager;
use flat_string::FlatString;

#[derive(Copy, Clone)]
pub(super) struct Defaults {
    pub(super) thickness: u8,
    pub(super) spacing: u8,
    pub(super) draw_mode: StackBarDrawMode,
    pub(super) attr: Option<CharAttribute>,
}

pub(super) struct YAxis {
    pub(super) width: u8,
    pub(super) step: u8,
    pub(super) visible: bool,
    pub(super) show_grid: bool,
    pub(super) zero: i32,
    pub(super) bottom_value: f64,
    pub(super) bottom_step: f64,
}

pub(super) struct XAxis {
    pub(super) label_format: XAxisLabelFormat,
    pub(super) spans: Vec<BarSpan>,
}

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

#[CustomControl(overwrite = OnPaint + OnResize, internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    pub(super) stacks: Vec<StackWithLayout<T>>,
    pub(super) stacks_width: u32,
    pub(super) left_scroll: i32,
    pub(super) first_visible_stack: u32,
    pub(super) surface: Surface,
    pub(super) defaults: Defaults,
    pub(super) scale: StackBarScale<T>,
    pub(super) number_format: FormatNumber,
    pub(super) percentage_format: FormatNumber,
    pub(super) yaxis: YAxis,
    pub(super) xaxis: XAxis,
}

impl<T> VStackedBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let extra = if flags.contains(Flags::ScrollBars) {
            StatusFlags::IncreaseBottomMarginOnFocus
        } else {
            StatusFlags::None
        };
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
            flags,
            stacks: Vec::new(),
            stacks_width: 0,
            left_scroll: 0,
            first_visible_stack: 0,
            surface: Surface::new(1, 1),
            defaults: Defaults {
                thickness: 1,
                spacing: 1,
                draw_mode: StackBarDrawMode::default(),
                attr: None,
            },
            scale: StackBarScale::FromZero,
            number_format: if T::is_float() {
                FormatNumber::new(10).decimals(2)
            } else {
                FormatNumber::new(10).group(3, b',')
            },
            percentage_format: FormatNumber::new(10).suffix("%"),
            yaxis: YAxis {
                width: 6,
                step: 3,
                visible: true,
                show_grid: true,
                zero: 0,
                bottom_value: 0.0,
                bottom_step: 0.0,
            },
            xaxis: XAxis {
                label_format: XAxisLabelFormat::None,
                spans: Vec::new(),
            },
        }
    }
    pub fn add_stack<S>(&mut self, stack: S)
    where
        S: Into<BarsStack<T>>,
    {
        let entry = self.new_entry(stack.into());
        self.stacks.push(entry);
        self.repaint_surface();
    }
    pub fn add_stacks<S>(&mut self, stacks: impl IntoIterator<Item = S>)
    where
        S: Into<BarsStack<T>>,
    {
        for stack in stacks {
            let entry = self.new_entry(stack.into());
            self.stacks.push(entry);
        }
        self.repaint_surface();
    }
    #[inline(always)]
    pub fn stacks_count(&self) -> usize {
        self.stacks.len()
    }
    #[inline(always)]
    pub fn get_stack(&self, index: usize) -> Option<&BarsStack<T>> {
        self.stacks.get(index).map(|entry| &entry.stack)
    }
    pub fn update_stack<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(&mut BarsStack<T>),
    {
        let defaults = self.defaults;
        if let Some(entry) = self.stacks.get_mut(index) {
            f(&mut entry.stack);
            entry.stack.update_actual_thickness(defaults.thickness, defaults.draw_mode);
            self.repaint_surface();
        }
    }
    pub fn clear(&mut self) {
        self.stacks.clear();
        self.repaint_surface();
    }
    pub fn set_stacks_scale(&mut self, scale: StackBarScale<T>) {
        self.scale = scale;
        self.repaint_surface();
    }
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
        self.repaint_surface();
    }
    pub fn set_percentage_format(&mut self, format: FormatNumber) {
        self.percentage_format = format;
        self.repaint_surface();
    }
    pub fn set_default_bar_width(&mut self, width: u8) {
        self.defaults.thickness = width.max(1);
        self.refresh_thickness();
        self.repaint_surface();
    }
    pub fn set_default_bar_spacing(&mut self, spacing: u8) {
        self.defaults.spacing = spacing;
        self.repaint_surface();
    }
    pub fn set_default_bar_drawmode(&mut self, draw_mode: StackBarDrawMode) {
        self.defaults.draw_mode = draw_mode;
        self.refresh_thickness();
        self.repaint_surface();
    }
    pub fn set_default_bar_attr(&mut self, attr: CharAttribute) {
        self.defaults.attr = Some(attr);
        self.repaint_surface();
    }
    pub fn set_yaxis_visible(&mut self, visible: bool) {
        self.yaxis.visible = visible;
        self.repaint_surface();
    }
    pub fn set_yaxis_show_grid(&mut self, show_grid: bool) {
        self.yaxis.show_grid = show_grid;
        self.repaint_surface();
    }
    pub fn set_yaxis_width(&mut self, width: u8) {
        self.yaxis.width = width.max(1);
        self.repaint_surface();
    }
    pub fn set_yaxis_step(&mut self, step: u8) {
        self.yaxis.step = step.max(1);
        self.repaint_surface();
    }
    pub fn set_xaxis_label_mode(&mut self, xaxis: XAxisLabelMode) {
        match xaxis {
            XAxisLabelMode::None => self.xaxis.label_format = XAxisLabelFormat::None,
            XAxisLabelMode::Index(start) => self.xaxis.label_format = XAxisLabelFormat::Index(start),
            XAxisLabelMode::StackLabels => self.xaxis.label_format = XAxisLabelFormat::StackLabels,
            XAxisLabelMode::Custom(spans) => {
                self.xaxis.label_format = XAxisLabelFormat::Custom;
                self.xaxis.spans.clear();
                self.xaxis.spans.extend(spans);
                self.xaxis.spans.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
                let spans = &mut self.xaxis.spans;
                let mut write = 0usize;
                for read in 0..spans.len() {
                    let overlaps = write > 0 && spans[read].start <= spans[write - 1].end;
                    if !overlaps {
                        if read != write {
                            spans.swap(read, write);
                        }
                        write += 1;
                    }
                }
                spans.truncate(write);
            }
        }
        self.repaint_surface();
    }
    #[inline(always)]
    pub(super) fn yaxis_format(&self) -> &FormatNumber {
        if matches!(self.scale, StackBarScale::Percentage) {
            &self.percentage_format
        } else {
            &self.number_format
        }
    }
    fn refresh_thickness(&mut self) {
        let defaults = self.defaults;
        for entry in self.stacks.iter_mut() {
            entry.stack.update_actual_thickness(defaults.thickness, defaults.draw_mode);
        }
    }
    #[inline(always)]
    pub(super) fn visible_height(&self) -> u32 {
        self.size().height.saturating_sub(self.xaxis.label_format.height() as u32 + 1)
    }
    #[inline(always)]
    pub(super) fn x_axis_left_margin(&self) -> i32 {
        if self.yaxis.visible && self.yaxis.width > 0 {
            self.yaxis.width as i32 + 2
        } else {
            0
        }
    }
    fn repaint_surface(&mut self) {
        self.update_layout();
        let height = self.visible_height();
        self.update_plot_layout(height);
        if !RuntimeManager::is_instantiated() {
            return;
        }
        let theme = self.theme();
        let background = theme.chart.background;
        let grid_attr = theme.chart.grid;
        let axis_attr = theme.chart.axis;
        let label_attr = theme.chart.label;
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', background));
        self.paint_yaxis(axis_attr, grid_attr, label_attr);
        self.paint_xaxis(axis_attr, label_attr);
        let left_margin = self.x_axis_left_margin();
        if (left_margin > 0) && !self.xaxis.label_format.is_none() {
            self.surface.reset_clip();
            self.surface.fill_horizontal_line(
                0,
                self.size().height as i32 - 1,
                left_margin.saturating_sub(1),
                Character::with_attributes(' ', background),
            );
        }
    }
    fn paint_yaxis(&mut self, axis_attr: CharAttribute, grid_attr: CharAttribute, label_attr: CharAttribute) {
        let bottom = self.size().height as i32 - if self.xaxis.label_format.is_none() { 1 } else { 2 };
        if self.yaxis.visible && self.yaxis.width > 0 {
            self.surface
                .draw_vertical_line(self.yaxis.width as i32 + 1, 0, bottom, LineType::Single, axis_attr);
            if !self.xaxis.label_format.is_none() {
                self.surface.write_char(
                    self.yaxis.width as i32 + 1,
                    bottom,
                    Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, axis_attr),
                );
            }
        }
        if self.yaxis.show_grid && self.yaxis.step > 0 {
            let x_poz = self.x_axis_left_margin();
            let right = self.size().width as i32;
            let ch = Character::with_attributes('┈', grid_attr);
            let format = if matches!(self.scale, StackBarScale::Percentage) {
                &self.percentage_format
            } else {
                &self.number_format
            };
            let mut buffer: [u8; 32] = [0u8; 32];
            let mut y = bottom;
            let mut bottom_value = self.yaxis.bottom_value;
            while y >= 0 {
                self.surface.fill_horizontal_line(x_poz, y, right, ch);
                if let Some(value) = format.write_float(bottom_value, &mut buffer) {
                    self.surface
                        .write_ascii(x_poz - value.len() as i32 - 1, y, value.as_bytes(), label_attr, false);
                }
                y -= self.yaxis.step as i32;
                bottom_value -= self.yaxis.bottom_step;
            }
        }
    }
    fn paint_xaxis(&mut self, axis_attr: CharAttribute, label_attr: CharAttribute) {
        if self.xaxis.label_format.is_none() {
            return;
        }
        let left = self.x_axis_left_margin();
        let right = self.size().width as i32;
        let y = self.size().height as i32 - 2;
        self.surface.draw_horizontal_line(left, y, right, LineType::Single, axis_attr);
        match self.xaxis.label_format {
            XAxisLabelFormat::None => (),
            XAxisLabelFormat::Index(start) => self.paint_xaxis_index(start, label_attr),
            XAxisLabelFormat::StackLabels => self.paint_xaxis_stack_labels(label_attr),
            XAxisLabelFormat::Custom => self.paint_xaxis_custom(label_attr),
        }
    }
    fn print_label(&mut self, x: i32, y: i32, start_stack_index: usize, end_stack_index: usize, label: &str, attr: CharAttribute) {
        let default_spacing = self.defaults.spacing;
        let first = &self.stacks[start_stack_index];
        let last = &self.stacks[end_stack_index];
        let width = (last.stack.actual_thickness() as i32) + last.pos - first.pos;
        let left_space = first.stack.actual_spacing(default_spacing) as i32;
        let right_space = if end_stack_index + 1 < self.stacks.len() {
            self.stacks[end_stack_index + 1].stack.actual_spacing(default_spacing) as i32
        } else {
            left_space
        };
        let left = left_space / 2 - (1 - (left_space & 1));
        let right = right_space / 2 - (1 - (right_space & 1));
        let extra_space = left.min(right);
        let total_space = width + extra_space * 2;
        if total_space < 1 {
            return;
        }
        let (text, count, truncated) = {
            let mut count = 0usize;
            let mut end = label.len();
            let mut truncated = false;
            for (byte_idx, _) in label.char_indices() {
                if count == total_space as usize {
                    end = byte_idx;
                    truncated = true;
                    break;
                }
                count += 1;
            }
            (&label[..end], count, truncated)
        };
        let x = x - extra_space;
        if truncated {
            self.surface.write_string(x, y, text, attr, false);
            self.surface.write_char(
                x + total_space - 1,
                y,
                Character::with_attributes(SpecialChar::ThreePointsHorizontal, attr),
            );
        } else {
            self.surface.write_string(x + (total_space - count as i32) / 2, y, text, attr, false);
        }
    }
    fn paint_xaxis_index(&mut self, start: i32, attr: CharAttribute) {
        let mut temp: [u8; 16] = [0u8; 16];
        let mut idx_start = self.first_visible_stack as usize;
        let mut stack_idx = (start as i64) + self.first_visible_stack as i64;
        let len = self.stacks.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let x = self.stacks[idx_start].pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if let Some(result) = INT_FORMAT.write_number(stack_idx, &mut temp) {
                self.print_label(x, y, idx_start, idx_start, result, attr);
            }
            idx_start += 1;
            stack_idx += 1;
        }
    }
    fn paint_xaxis_stack_labels(&mut self, attr: CharAttribute) {
        let mut temp_str: FlatString<254> = FlatString::new();
        let mut idx_start = self.first_visible_stack as usize;
        let len = self.stacks.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let entry = &self.stacks[idx_start];
            let x = entry.pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if !entry.stack.label().is_empty() {
                temp_str.set(entry.stack.label());
                self.print_label(x, y, idx_start, idx_start, temp_str.as_str(), attr);
            }
            idx_start += 1;
        }
    }
    fn paint_xaxis_custom(&mut self, attr: CharAttribute) {
        let len = self.stacks.len();
        if len == 0 {
            return;
        }
        let left_margin = self.x_axis_left_margin();
        let width = self.size().width as i32;
        let y = self.size().height as i32 - 1;
        let spans_count = self.xaxis.spans.len();
        for i in 0..spans_count {
            let span = &self.xaxis.spans[i];
            let start_index = span.start as usize;
            if start_index >= len {
                break;
            }
            let x = self.stacks[start_index].pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            let end_index = (span.end as usize).min(len - 1);
            let end_x = self.stacks[end_index].pos - self.left_scroll + self.stacks[end_index].stack.actual_thickness() as i32;
            if end_x < 0 {
                continue;
            }
            let span_copy = *span;
            self.print_label(x, y, start_index, end_index, span_copy.label.as_str(), attr);
        }
    }
    fn update_layout(&mut self) {
        self.update_stack_sums();
        self.update_columns_layout();
    }
    pub(super) fn compute_range(&self) -> (f64, f64) {
        match &self.scale {
            StackBarScale::FromZero => self.range_from_zero(0.0, 0.0),
            StackBarScale::FromZeroMinRange { min, max } => {
                let min = (*min).to_f64();
                let max = (*max).to_f64();
                self.range_from_zero(if min.is_finite() { min } else { 0.0 }, if max.is_finite() { max } else { 0.0 })
            }
            StackBarScale::FitData => self.range_fit_data(),
            StackBarScale::Fixed { min, max } => {
                let min = (*min).to_f64();
                let max = (*max).to_f64();
                if min.is_finite() && max.is_finite() {
                    (min, max)
                } else {
                    (0.0, 0.0)
                }
            }
            StackBarScale::Percentage => self.range_percentage(),
        }
    }
    fn range_from_zero(&self, min: f64, max: f64) -> (f64, f64) {
        let mut lo = min.min(0.0);
        let mut hi = max.max(0.0);
        for entry in self.stacks.iter() {
            lo = lo.min(entry.negative_sum);
            hi = hi.max(entry.positive_sum);
        }
        (lo, hi)
    }
    fn range_fit_data(&self) -> (f64, f64) {
        let mut lo = f64::MAX;
        let mut hi = f64::MIN;
        let mut found = false;
        for entry in self.stacks.iter() {
            if entry.has_positive() {
                lo = lo.min(entry.positive_sum);
                hi = hi.max(entry.positive_sum);
                found = true;
            }
            if entry.has_negative() {
                lo = lo.min(entry.negative_sum);
                hi = hi.max(entry.negative_sum);
                found = true;
            }
        }
        if found {
            (lo, hi)
        } else {
            (0.0, 0.0)
        }
    }
    fn range_percentage(&self) -> (f64, f64) {
        let mut up = 0.0f64;
        let mut down = 0.0f64;
        for entry in self.stacks.iter() {
            let total = entry.abs_total();
            if total > 0.0 {
                up = up.max(entry.positive_sum / total * 100.0);
                down = down.max(-entry.negative_sum / total * 100.0);
            }
        }
        (-down, up)
    }
    pub(super) fn value_mapping(&self, height: u32) -> ValueMapping {
        let (lo, hi) = self.compute_range();
        ValueMapping::new(lo, hi, height)
    }
    pub(super) fn update_segments_layout(&mut self, height: u32) {
        let mapping = self.value_mapping(height);
        self.apply_segments(&mapping);
    }
    pub(super) fn update_yaxis_data(&mut self, height: u32) {
        let mapping = self.value_mapping(height);
        self.apply_yaxis(&mapping);
    }
    pub(super) fn update_plot_layout(&mut self, height: u32) {
        let mapping = self.value_mapping(height);
        self.apply_yaxis(&mapping);
        self.apply_segments(&mapping);
    }
    fn apply_segments(&mut self, mapping: &ValueMapping) {
        let percentage = matches!(self.scale, StackBarScale::Percentage);
        for entry in self.stacks.iter_mut() {
            entry.update_segments(mapping, percentage);
        }
    }
    fn apply_yaxis(&mut self, mapping: &ValueMapping) {
        self.yaxis.zero = mapping.zero() as i32;
        self.yaxis.bottom_value = mapping.bottom_value();
        self.yaxis.bottom_step = -(mapping.value_per_cell() * self.yaxis.step as f64);
    }
    pub(super) fn update_stack_sums(&mut self) {
        for entry in self.stacks.iter_mut() {
            entry.update_sums();
        }
    }
    pub(super) fn update_columns_layout(&mut self) {
        if self.stacks.is_empty() {
            self.stacks_width = 0;
            return;
        }
        let default_spacing = self.defaults.spacing;
        let mut x: i32 = 0;
        for entry in self.stacks.iter_mut() {
            x += entry.stack.actual_spacing(default_spacing) as i32;
            entry.pos = x;
            x += entry.stack.actual_thickness() as i32;
        }
        self.stacks_width = x as u32 + self.stacks[0].stack.actual_spacing(default_spacing) as u32;
    }
    fn new_entry(&self, mut stack: BarsStack<T>) -> StackWithLayout<T> {
        stack.update_actual_thickness(self.defaults.thickness, self.defaults.draw_mode);
        StackWithLayout::new(stack)
    }
}

impl<T> OnPaint for VStackedBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        } else {
            surface.draw_surface(0, 0, &self.surface);
        }
    }
}

impl<T> OnResize for VStackedBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.repaint_surface();
    }
}

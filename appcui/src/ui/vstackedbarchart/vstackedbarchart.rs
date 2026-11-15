use crate::prelude::*;
use super::stackbar::StackWithLayout;
use super::{BarsStack, Flags, StackBarDrawMode, StackBarScale, XAxisLabelFormat, XAxisLabelMode};
use crate::ui::components::BarSpan;

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
}

pub(super) struct XAxis {
    pub(super) label_format: XAxisLabelFormat,
    pub(super) spans: Vec<BarSpan>,
}

#[CustomControl(internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    pub(super) stacks: Vec<StackWithLayout<T>>,
    pub(super) stacks_width: u32,
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
        self.update_layout();
    }
    pub fn add_stacks<S>(&mut self, stacks: impl IntoIterator<Item = S>)
    where
        S: Into<BarsStack<T>>,
    {
        for stack in stacks {
            let entry = self.new_entry(stack.into());
            self.stacks.push(entry);
        }
        self.update_layout();
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
            self.update_layout();
        }
    }
    pub fn clear(&mut self) {
        self.stacks.clear();
        self.update_layout();
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
    fn repaint_surface(&mut self) {
        self.update_layout();
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

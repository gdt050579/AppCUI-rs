use super::events::{EventData, EventType};
use super::stackbar::{paint_segment, resolve_attr, segment_rows, StackWithLayout, ValueMapping};
use super::{BarsStack, Flags, StackBarDrawMode, StackBarScale, XAxisLabelFormat, XAxisLabelMode};
use crate::prelude::*;
use crate::ui::components::BarSpan;
use flat_string::FlatString;
use std::any::TypeId;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(super) enum Selection {
    #[default]
    None,
    Stack(u32),
    Cell(u32, u32),
}

#[CustomControl(overwrite = OnPaint + OnResize + OnKeyPressed + OnMouseEvent, internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    pub(super) flags: Flags,
    pub(super) stacks: Vec<StackWithLayout<T>>,
    pub(super) stacks_width: u32,
    pub(super) left_scroll: i32,
    pub(super) first_visible_stack: u32,
    pub(super) scroll_anchor: u32,
    keep_scroll_anchor: bool,
    pub(super) selection: Selection,
    pub(super) hovered: Option<(u32, u32)>,
    pub(super) tooltip_text: String,
    pub(super) scrollbars: ScrollBars,
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
            scroll_anchor: 0,
            keep_scroll_anchor: false,
            selection: Selection::None,
            hovered: None,
            tooltip_text: String::new(),
            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
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
    pub fn ensure_visible(&mut self, index: usize) {
        if index >= self.stacks.len() {
            return;
        }
        self.update_layout();
        let plot_width = (self.size().width as i32 - self.x_axis_left_margin()).max(0);
        if plot_width <= 0 {
            return;
        }
        let left = self.stacks[index].pos;
        let right = left + self.stacks[index].stack.actual_thickness() as i32;
        let view_left = self.left_scroll;
        let view_right = view_left + plot_width;
        let new_scroll = if right - left <= plot_width {
            if left >= view_left && right <= view_right {
                return;
            }
            if left < view_left {
                left
            } else {
                right - plot_width
            }
        } else if right <= view_left {
            right - plot_width
        } else if left >= view_right {
            left
        } else {
            return;
        };
        let new_scroll = new_scroll.clamp(0, self.max_left_scroll());
        if new_scroll != self.left_scroll {
            self.left_scroll = new_scroll;
            self.repaint_surface();
        }
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
            self.clamp_selection();
            self.clear_hovered();
            self.repaint_surface();
        }
    }
    pub fn clear(&mut self) {
        self.stacks.clear();
        self.selection = Selection::None;
        self.clear_hovered();
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
    #[inline(always)]
    pub fn selected_stack(&self) -> Option<u32> {
        match self.selection {
            Selection::None => None,
            Selection::Stack(index) | Selection::Cell(index, _) => Some(index),
        }
    }
    #[inline(always)]
    pub fn selected_cell(&self) -> Option<u32> {
        match self.selection {
            Selection::Cell(_, index) => Some(index),
            Selection::None | Selection::Stack(_) => None,
        }
    }
    pub fn select_stack(&mut self, index: usize) {
        if index < self.stacks.len() {
            self.set_selection(Selection::Stack(index as u32));
        }
    }
    pub fn select_cell(&mut self, stack_index: usize, cell_index: usize) {
        if let Some(entry) = self.stacks.get(stack_index) {
            if cell_index < entry.stack.count() {
                self.set_selection(Selection::Cell(stack_index as u32, cell_index as u32));
            }
        }
    }
    pub fn clear_selection(&mut self) {
        self.set_selection(Selection::None);
    }
    pub(super) fn set_selection(&mut self, selection: Selection) -> Option<EventType> {
        if self.selection == selection {
            return None;
        }
        self.selection = selection;
        match self.selection {
            Selection::None => Some(EventType::ClearSelection),
            Selection::Stack(index) => Some(EventType::StackSelected(index)),
            Selection::Cell(stack_index, cell_index) => Some(EventType::CellSelected { stack_index, cell_index }),
        }
    }
    pub(super) fn clamp_selection(&mut self) {
        self.selection = match self.selection {
            Selection::None => Selection::None,
            Selection::Stack(i) => {
                if (i as usize) < self.stacks.len() {
                    Selection::Stack(i)
                } else {
                    Selection::None
                }
            }
            Selection::Cell(i, j) => match self.stacks.get(i as usize) {
                None => Selection::None,
                Some(entry) if (j as usize) < entry.stack.count() => Selection::Cell(i, j),
                Some(_) => Selection::Stack(i),
            },
        };
    }
    fn zero_row(&self) -> i32 {
        self.visible_height() as i32 - self.yaxis.zero
    }
    fn stack_column(&self, entry: &StackWithLayout<T>) -> (i32, i32) {
        let x = entry.pos - self.left_scroll + self.x_axis_left_margin();
        (x, x + entry.stack.actual_thickness().max(1) as i32 - 1)
    }
    pub(super) fn cell_rect(&self, stack_index: usize, cell_index: usize) -> Option<Rect> {
        let entry = self.stacks.get(stack_index)?;
        let zero_row = self.zero_row();
        let segment = entry.layout.get(cell_index)?;
        let (top, bottom) = segment_rows(zero_row - segment.offset as i32, segment.len).unwrap_or((zero_row, zero_row));
        let (left, right) = self.stack_column(entry);
        Some(Rect::new(left, top, right, bottom))
    }
    pub(super) fn stack_rect(&self, stack_index: usize) -> Option<Rect> {
        let entry = self.stacks.get(stack_index)?;
        let zero_row = self.zero_row();
        let (top, bottom) = entry
            .layout
            .iter()
            .filter_map(|segment| segment_rows(zero_row - segment.offset as i32, segment.len))
            .fold(None, |acc: Option<(i32, i32)>, (t, b)| {
                Some(acc.map_or((t, b), |(at, ab)| (at.min(t), ab.max(b))))
            })
            .unwrap_or((zero_row, zero_row));
        let (left, right) = self.stack_column(entry);
        Some(Rect::new(left, top, right, bottom))
    }
    pub(super) fn selection_rect(&self) -> Option<Rect> {
        match self.selection {
            Selection::None => None,
            Selection::Stack(i) => self.stack_rect(i as usize),
            Selection::Cell(i, j) => self.cell_rect(i as usize, j as usize),
        }
    }
    fn format_value(format: &FormatNumber, value: f64) -> String {
        let mut buffer = [0u8; 64];
        format.write_float(value, &mut buffer).unwrap_or("").to_string()
    }
    pub(super) fn tooltip_for(&self, stack_index: usize, cell_index: usize) -> Option<String> {
        let entry = self.stacks.get(stack_index)?;
        let bar = entry.stack.get(cell_index)?;
        let mut text = String::new();
        if !entry.stack.label().is_empty() {
            text.push_str(entry.stack.label());
            text.push('\n');
        }
        text.push_str("Stack: ");
        text.push_str(&Self::format_value(&self.number_format, entry.positive_sum + entry.negative_sum));
        text.push('\n');
        if !bar.label().is_empty() {
            text.push_str(bar.label());
            text.push('\n');
        }
        let value = bar.value().to_f64();
        text.push_str("Value: ");
        text.push_str(&Self::format_value(&self.number_format, value));
        if matches!(self.scale, StackBarScale::Percentage) {
            let total = entry.abs_total();
            if total > 0.0 {
                text.push_str(" (");
                text.push_str(&Self::format_value(&self.percentage_format, value.abs() / total * 100.0));
                text.push(')');
            }
        }
        Some(text)
    }
    pub(super) fn clear_hovered(&mut self) {
        if self.hovered.take().is_some() {
            self.hide_tooltip();
        }
    }
    pub(super) fn update_hovered(&mut self, x: i32, y: i32) -> bool {
        let hit = self.cell_index_at(x, y).map(|(stack, cell)| (stack as u32, cell as u32));
        if hit == self.hovered {
            return false;
        }
        match hit {
            Some((stack, cell)) => {
                self.hovered = hit;
                if let (Some(text), Some(rect)) = (
                    self.tooltip_for(stack as usize, cell as usize),
                    self.cell_rect(stack as usize, cell as usize),
                ) {
                    self.tooltip_text = text;
                    self.show_tooltip_on_rect(&self.tooltip_text, &rect);
                }
            }
            None => self.clear_hovered(),
        }
        true
    }
    pub(super) fn paint_selection(&self, surface: &mut Surface, theme: &Theme) {
        let Some(rect) = self.selection_rect() else {
            return;
        };
        let margin = self.x_axis_left_margin();
        let left = if margin > 0 { margin - 1 } else { 0 };
        let right = self.size().width.saturating_sub(1) as i32;
        let bottom = self.visible_height() as i32;
        let border_bottom = (bottom + 1).min(self.size().height as i32 - 1);
        surface.set_relative_clip(left, 0, right, border_bottom);
        let mut border = rect;
        border.inflate_width(1, 1, 1, 1);
        surface.draw_rect(border, LineType::Single, theme.chart.selection_border);
        if self.flags.contains(Flags::DimBarsOnSelection) {
            let attr = theme.chart.inactive;
            surface.transform_rect(Rect::new(left, 0, right, border_bottom), |ch, p| {
                if border.contains(p) {
                    None
                } else {
                    Some(Character::with_attributes(ch.code, attr))
                }
            });
        }
    }
    pub(super) fn stack_index_at(&self, x: i32, y: i32) -> Option<usize> {
        let left_margin = self.x_axis_left_margin();
        let plot_bottom = self.visible_height() as i32;
        if x < left_margin || x >= self.size().width as i32 || y < 0 || y > plot_bottom {
            return None;
        }
        let content_x = x + self.left_scroll - left_margin;
        let index = self.stacks.partition_point(|s| s.pos <= content_x).checked_sub(1)?;
        let entry = &self.stacks[index];
        if content_x >= entry.pos + entry.stack.actual_thickness() as i32 {
            return None;
        }
        Some(index)
    }
    pub(super) fn cell_index_at(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        let stack_index = self.stack_index_at(x, y)?;
        let zero_row = self.visible_height() as i32 - self.yaxis.zero;
        let entry = &self.stacks[stack_index];
        entry.layout.iter().enumerate().find_map(|(bar_index, segment)| {
            let (top, bottom) = segment_rows(zero_row - segment.offset as i32, segment.len)?;
            ((top..=bottom).contains(&y)).then_some((stack_index, bar_index))
        })
    }
    fn raise_selection_event(&self, event_type: EventType) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::VStackedBarChart(EventData {
                type_id: TypeId::of::<T>(),
                event_type,
            }),
        });
    }
    fn change_selection(&mut self, selection: Selection) {
        let Some(event_type) = self.set_selection(selection) else {
            return;
        };
        self.raise_selection_event(event_type);
        if let Selection::Stack(index) | Selection::Cell(index, _) = selection {
            self.ensure_visible(index as usize);
        }
    }
    pub(super) fn move_stack_selection(&mut self, delta: i32) {
        let count = self.stacks.len();
        if count == 0 {
            return;
        }
        let target = match self.selection {
            Selection::None => 0,
            Selection::Stack(index) | Selection::Cell(index, _) => (index as i64 + delta as i64).clamp(0, count as i64 - 1) as u32,
        };
        self.change_selection(Selection::Stack(target));
    }
    fn first_cell_of(&self, stack_index: usize) -> Option<usize> {
        let entry = self.stacks.get(stack_index)?;
        let first = entry.layout.iter().position(|segment| segment.len != 0);
        first.or(if entry.stack.count() > 0 { Some(0) } else { None })
    }
    fn cell_order_key(&self, stack_index: usize, cell_index: usize) -> Option<(i32, usize)> {
        Some((self.cell_rect(stack_index, cell_index)?.top(), cell_index))
    }
    pub(super) fn move_cell_selection(&mut self, up: bool) {
        let (stack_index, current) = match self.selection {
            Selection::None if self.stacks.is_empty() => return,
            Selection::None => (0, None),
            Selection::Stack(index) => (index as usize, None),
            Selection::Cell(index, cell) => (index as usize, Some(cell as usize)),
        };
        let Some(current) = current else {
            match self.first_cell_of(stack_index) {
                Some(cell) => self.change_selection(Selection::Cell(stack_index as u32, cell as u32)),
                None => self.change_selection(Selection::Stack(stack_index as u32)),
            }
            return;
        };
        let Some(current_key) = self.cell_order_key(stack_index, current) else {
            return;
        };
        let Some(entry) = self.stacks.get(stack_index) else {
            return;
        };
        let keys = entry
            .layout
            .iter()
            .enumerate()
            .filter(|(_, segment)| segment.len != 0)
            .filter_map(|(cell, _)| self.cell_order_key(stack_index, cell));
        let target = if up {
            keys.filter(|key| *key < current_key).max()
        } else {
            keys.filter(|key| *key > current_key).min()
        };
        if let Some((_, cell)) = target {
            self.change_selection(Selection::Cell(stack_index as u32, cell as u32));
        }
    }
    pub(super) fn max_left_scroll(&self) -> i32 {
        let content_width = self.stacks_width as i32 + self.x_axis_left_margin() + 1;
        (content_width - self.size().width as i32).max(0)
    }
    pub(super) fn update_first_visible_stack(&mut self) {
        let left_scroll = self.left_scroll;
        self.first_visible_stack = (self.stacks.partition_point(|s| s.pos < left_scroll) as u32).saturating_sub(1);
    }
    pub(super) fn sync_scrollbars(&mut self) {
        let content_width = self.stacks_width as u64 + self.x_axis_left_margin() as u64 + 1;
        let height = self.size().height as u64;
        self.scrollbars.resize(content_width, height, &self.base);
        self.scrollbars.set_indexes(self.left_scroll as u64, 0);
    }
    pub(super) fn update_scroll_state(&mut self) {
        let max_scroll = self.max_left_scroll();
        let upper_bound = if max_scroll > 0 {
            max_scroll.max(self.stacks.last().map_or(0, |s| s.pos))
        } else {
            0
        };
        self.left_scroll = self.left_scroll.clamp(0, upper_bound);
        self.update_first_visible_stack();
        if self.keep_scroll_anchor {
            self.keep_scroll_anchor = false;
        } else {
            self.scroll_anchor = self.first_visible_stack;
        }
        self.sync_scrollbars();
    }
    pub(super) fn after_horizontal_scroll(&mut self) {
        self.clear_hovered();
        self.repaint_surface();
    }
    pub(super) fn scroll_by_columns(&mut self, delta: i32) {
        if (delta < 0 && self.left_scroll > 0) || (delta > 0 && self.left_scroll < self.max_left_scroll()) {
            self.left_scroll += delta;
            self.after_horizontal_scroll();
        }
    }
    pub(super) fn scroll_to_next_stack(&mut self) {
        self.clear_hovered();
        let next = self.scroll_anchor as usize + 1;
        if let Some(entry) = self.stacks.get(next) {
            self.scroll_anchor = next as u32;
            self.keep_scroll_anchor = true;
            self.left_scroll = entry.pos;
            self.repaint_surface();
        }
    }
    pub(super) fn scroll_to_previous_stack(&mut self) {
        self.clear_hovered();
        if self.scroll_anchor > 0 {
            self.scroll_anchor -= 1;
            self.keep_scroll_anchor = true;
            self.left_scroll = self.stacks[self.scroll_anchor as usize].pos;
            self.repaint_surface();
        }
    }
    pub(super) fn update_scroll_pos_from_scrollbars(&mut self) {
        self.left_scroll = self.scrollbars.horizontal_index() as i32;
        self.update_first_visible_stack();
        self.repaint_surface();
    }
    fn repaint_surface(&mut self) {
        self.update_layout();
        self.update_scroll_state();
        let height = self.visible_height();
        self.update_plot_layout(height);
        let theme = self.theme();
        let background = theme.chart.background;
        let grid_attr = theme.chart.grid;
        let axis_attr = theme.chart.axis;
        let label_attr = theme.chart.label;
        let theme_bars = theme.chart.bar;
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', background));
        let size = self.size();
        self.paint_yaxis(size.width as i32, size.height as i32, axis_attr, grid_attr, label_attr);
        self.paint_xaxis(axis_attr, label_attr);
        let bar_colors = theme_bars;
        let width = self.size().width as i32;
        let plot_bottom = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        self.paint_stacks(&bar_colors, width, plot_bottom);
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
    pub(super) fn paint_stacks(&mut self, bar_colors: &[CharAttribute], width: i32, plot_bottom: i32) {
        let left_margin = self.x_axis_left_margin();
        let zero_row = plot_bottom - self.yaxis.zero;
        let defaults = self.defaults;
        self.surface.set_relative_clip(left_margin, 0, width, plot_bottom);
        let mut index = self.first_visible_stack as usize;
        while index < self.stacks.len() {
            let entry = &self.stacks[index];
            let x = entry.pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            let thickness = entry.stack.actual_thickness();
            for (bar_index, (bar, segment)) in entry.stack.bars.iter().zip(entry.layout.iter()).enumerate() {
                if segment.len == 0 {
                    continue;
                }
                let theme_attr = if bar_colors.is_empty() {
                    CharAttribute::default()
                } else {
                    bar_colors[bar_index % bar_colors.len()]
                };
                let attr = resolve_attr(bar.attr(), defaults.attr, theme_attr);
                let mode = bar.draw_mode().unwrap_or(defaults.draw_mode);
                paint_segment(&mut self.surface, x, zero_row - segment.offset as i32, thickness, segment.len, mode, attr);
            }
            index += 1;
        }
        self.surface.reset_clip();
    }
    pub(super) fn paint_yaxis(&mut self, width: i32, height: i32, axis_attr: CharAttribute, grid_attr: CharAttribute, label_attr: CharAttribute) {
        let bottom = height - if self.xaxis.label_format.is_none() { 1 } else { 2 };
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
            let right = width;
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
            if self.flags.contains(Flags::ShowZeroLineOnYAxis) && self.yaxis.bottom_step != 0.0 {
                let y_zero = bottom - ((self.yaxis.bottom_value * self.yaxis.step as f64) / self.yaxis.bottom_step).round() as i32;
                if (0..=bottom).contains(&y_zero) {
                    self.surface.draw_horizontal_line(x_poz, y_zero, right, LineType::Single, grid_attr);
                    if let Some(value) = format.write_float(0f64, &mut buffer) {
                        self.surface
                            .write_ascii(x_poz - value.len() as i32 - 1, y_zero, value.as_bytes(), label_attr, false);
                    }
                }
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
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        } else {
            surface.draw_surface(0, 0, &self.surface);
            self.paint_selection(surface, theme);
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

impl<T> OnKeyPressed for VStackedBarChart<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.move_stack_selection(-1);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.move_stack_selection(1);
                EventProcessStatus::Processed
            }
            key!("Up") => {
                self.move_cell_selection(true);
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.move_cell_selection(false);
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.left_scroll = 0;
                self.after_horizontal_scroll();
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.left_scroll = self.max_left_scroll();
                self.after_horizontal_scroll();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Left") => {
                self.scroll_to_previous_stack();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
                self.scroll_to_next_stack();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnMouseEvent for VStackedBarChart<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.clear_hovered();
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.clear_hovered();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(pos) => {
                if self.update_hovered(pos.x, pos.y) {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(data) if data.button == MouseButton::Left => {
                let selection = match self.cell_index_at(data.x, data.y) {
                    Some((stack, bar)) => Selection::Cell(stack as u32, bar as u32),
                    None => Selection::None,
                };
                self.clear_hovered();
                self.change_selection(selection);
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Left | MouseWheelDirection::Up => self.scroll_by_columns(-1),
                    MouseWheelDirection::Right | MouseWheelDirection::Down => self.scroll_by_columns(1),
                }
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(data) if data.button == MouseButton::Left => {
                let Some(stack) = self.stack_index_at(data.x, data.y) else {
                    return EventProcessStatus::Ignored;
                };
                self.clear_hovered();
                self.change_selection(Selection::Stack(stack as u32));
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

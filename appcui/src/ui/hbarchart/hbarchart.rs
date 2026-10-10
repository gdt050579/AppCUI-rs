use super::{Bar, BarDrawMode, BarScale, BarSpan, Flags, YAxisLabelFormat, YAxisLabelMode};
use super::events::{EventData, EventType};
use crate::prelude::*;
use crate::ui::components::{BarDefaults, BarLayout, ScrollBars, BarWithLayout, Bars};

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');


struct XAxis {
    step: u8,
    zero: i32,
    left_value: f64,
    left_step: f64,
    visible: bool,
    show_grid: bool,
}

struct YAxis {
    width: u8,
    label_format: YAxisLabelFormat,
    spans: Vec<BarSpan>,
}

#[CustomControl(overwrite = OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal = true)]
/// A horizontal bar chart for a numeric series of type `T`.
///
/// `HBarChart` holds one horizontal bar per value. Scale, axis labels, the default
/// bar appearance, and number formatting are set through the methods on this type.
/// Optional behavior, such as scroll bars, a zero line, and dimming unselected bars,
/// is controlled by [`Flags`].
///
/// Clicking a bar selects it and raises a bar-selected event. Clicking outside any
/// bar clears the selection. Hovering a bar shows its value, and its label when one
/// is set, in a tooltip. When the chart has focus, Up and Down scroll by one row,
/// Home and End jump to the ends, and Ctrl+Up and Ctrl+Down move by one bar.
pub struct HBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    bars: Vec<BarWithLayout<T>>,
    bars_height: u32,
    pub(super) top_scroll: i32,
    pub(super) first_visible_bar: u32,
    xaxis: XAxis,
    scale: BarScale<T>,
    number_format: FormatNumber,
    defaults: BarDefaults,
    surface: Surface,
    use_theme_colors_for_bars: bool,
    yaxis: YAxis,
    scrollbars: ScrollBars,
    selected_bar: Option<u32>,
    hovered_bar: Option<u32>,
    tooltip_text: String,
}

impl<T> HBarChart<T>
where
    T: Number + 'static,
{
    /// Creates an empty horizontal bar chart.
    ///
    /// `layout` places the control. `flags` selects optional behavior; see [`Flags`].
    ///
    /// A new chart scales bars with [`BarScale::FromZero`], shows the X axis and its
    /// grid, and formats labels with two decimals when `T` is a floating-point type
    /// or with thousands separators when `T` is an integer. Bars use the theme color,
    /// a thickness of 1, and a spacing of 1 until a default is changed. Category
    /// labels are hidden until [`Self::set_yaxis_label_mode`] is called. The column
    /// reserved for those labels is 6 characters wide, and grid lines are 3 columns
    /// apart.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = HBarChart::<i32>::new(
    ///     layout!("d:f"),
    ///     hbarchart::Flags::ScrollBars | hbarchart::Flags::ShowZeroLineOnXAxis,
    /// );
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let extra = if flags.contains(Flags::ScrollBars) {
            StatusFlags::IncreaseRightMarginOnFocus
        } else {
            StatusFlags::None
        };
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
            flags,
            bars: Vec::new(),
            xaxis: XAxis {
                step: 3,
                zero: 0,
                left_value: 0.0,
                left_step: 0.0,
                visible: true,
                show_grid: true,
            },
            defaults: BarDefaults {
                attr: CharAttribute::default(),
                thickness: 1,
                spacing: 1,
                vertical: false,
                draw_mode: BarDrawMode::default(),
            },
            yaxis: YAxis {
                width: 6,
                label_format: YAxisLabelFormat::None,
                spans: Vec::new(),
            },
            scale: BarScale::FromZero,
            number_format: if T::is_float() {
                FormatNumber::new(10).decimals(2)
            } else {
                FormatNumber::new(10).group(3, b',')
            },
            surface: Surface::new(1, 1),
            use_theme_colors_for_bars: true,
            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            selected_bar: None,
            top_scroll: 0,
            first_visible_bar: 0,
            bars_height: 0,
            hovered_bar: None,
            tooltip_text: String::new(),
        }
    }
    /// Appends one bar and repaints the chart.
    ///
    /// `bar` may be a value of type `T` or any type that converts into a [`Bar`],
    /// such as the result of [`BarBuilder::build`](BarBuilder::build).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bar(10);
    /// chart.add_bar(hbarchart::BarBuilder::new(20).label("Feb").build());
    /// ```
    pub fn add_bar<B>(&mut self, bar: B)
    where
        B: Into<Bar<T>>,
    {
        self.bars.push(BarWithLayout::new(bar.into()));
        self.repaint_surface();
    }
    /// Appends several bars and repaints the chart once.
    ///
    /// Each item may be a value of type `T` or any type that converts into a [`Bar`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bars(1..=12);
    /// ```
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.bars.extend(bars.into_iter().map(|bar| BarWithLayout::new(bar.into())));
        self.repaint_surface();
    }
    /// Returns the number of bars in the chart.
    #[inline(always)]
    pub fn bars_count(&self) -> usize {
        self.bars.len()
    }
    /// Returns the index of the currently selected bar, or `None` if no bar is selected.
    #[inline(always)]
    pub fn selected_bar(&self) -> Option<u32> {
        self.selected_bar
    }
    /// Sets how many characters are reserved for the labels beside the bars.
    ///
    /// The default is 6. Values below 1 are treated as 1. One extra column is kept
    /// for the axis line. This width is used only while a Y-axis label mode is set.
    /// A longer label is cut off and its last visible character is replaced with an
    /// ellipsis.
    pub fn set_xaxis_width(&mut self, value: u8) {
        self.yaxis.width = value.max(1);
        self.repaint_surface();
    }
    /// Scrolls the chart vertically so the bar at `index` is visible.
    ///
    /// When the bar fits in the plot area, the chart scrolls the minimum amount needed to show
    /// it entirely. A bar that is already fully visible leaves the scroll position unchanged.
    /// A bar taller than the plot area is scrolled until part of it is on screen.
    /// Does nothing if `index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bars(1..=50);
    /// chart.ensure_visible(49);
    /// ```
    pub fn ensure_visible(&mut self, index: usize) {
        if index >= self.bars.len() {
            return;
        }
        self.update_bars_layout();
        let plot_height = self.plot_rows().max(0);
        if plot_height <= 0 {
            return;
        }
        let bar = &self.bars[index];
        let bar_top = bar.pos;
        let bar_bottom = bar_top + bar.bar.actual_thickness(&self.defaults) as i32;
        let view_top = self.top_scroll;
        let view_bottom = view_top + plot_height;
        let new_scroll = if bar_bottom - bar_top <= plot_height {
            if bar_top >= view_top && bar_bottom <= view_bottom {
                return;
            }
            if bar_top < view_top {
                bar_top
            } else {
                bar_bottom - plot_height
            }
        } else if bar_bottom <= view_top {
            bar_bottom - plot_height
        } else if bar_top >= view_bottom {
            bar_top
        } else {
            return;
        };
        let new_scroll = new_scroll.clamp(0, self.max_top_scroll());
        if new_scroll != self.top_scroll {
            self.top_scroll = new_scroll;
            self.after_vertical_scroll();
        }
    }
    /// Returns an immutable reference to the bar at `index`, or `None` if out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bar(3);
    /// if let Some(bar) = chart.get_bar(0) {
    ///     assert_eq!(bar.value(), 3);
    /// }
    /// ```
    #[inline(always)]
    pub fn get_bar(&self, index: usize) -> Option<&Bar<T>> {
        self.bars.get(index).map(|item| &item.bar)
    }
    /// Mutates the bar at `index`, then relayouts and repaints the chart.
    ///
    /// Does nothing if `index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bar(1);
    /// chart.update_bar(0, |bar| {
    ///     bar.set_value(10);
    ///     bar.set_label("Jun");
    ///     bar.set_thickness(4);
    /// });
    /// ```
    pub fn update_bar<F>(&mut self, index: usize, f: F)
    where
        F: FnOnce(&mut Bar<T>),
    {
        if let Some(item) = self.bars.get_mut(index) {
            f(&mut item.bar);
            self.repaint_surface();
        }
    }
    /// Mutates the bar series in place, then relayouts and repaints once.
    ///
    /// Use this to insert, delete, replace, or edit several bars without repainting
    /// after each change. The closure receives a [`Bars`] view over the current series.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bars(&[1, 2, 3]);
    /// chart.update_bars(|bars| {
    ///     bars.get_mut(1).unwrap().set_value(20);
    ///     bars.insert(0, 0);
    ///     bars.delete(3);
    /// });
    /// ```
    pub fn update_bars<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Bars<'_, T>),
    {
        f(&mut Bars { inner: &mut self.bars });
        self.clamp_selected_bar();
        self.repaint_surface();
    }
    /// Sets how bar values are mapped onto the bar length.
    ///
    /// * [`BarScale::FromZero`] draws every bar from zero. The visible range includes
    ///   zero and every bar value.
    /// * [`BarScale::FromZeroMinRange`] does the same, and also expands the range so
    ///   that it covers `min` and `max`.
    /// * [`BarScale::FitData`] stretches the smallest and largest values across the
    ///   full plot length.
    /// * [`BarScale::Fixed`] uses the given `min` and `max`. Values outside that
    ///   range are drawn at the corresponding edge of the plot.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.set_bars_scale(hbarchart::BarScale::Fixed { min: 0, max: 100 });
    /// ```
    pub fn set_bars_scale(&mut self, scale: BarScale<T>) {
        self.scale = scale;
        self.repaint_surface();
    }
    /// Sets the format used for axis labels, the zero-line label, and hover tooltips.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<f64>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.set_number_format(FormatNumber::new(10).decimals(1));
    /// ```
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
        self.repaint_surface();
    }
    /// Sets the thickness, in cells, of bars that do not specify their own.
    ///
    /// Values below 1 are treated as 1. A bar can override this with
    /// [`Bar::set_thickness`](Bar::set_thickness).
    pub fn set_default_bar_width(&mut self, width: u8) {
        self.defaults.thickness = width.max(1);
        self.repaint_surface();
    }
    /// Sets the gap, in cells, before bars that do not specify their own spacing.
    ///
    /// A bar can override this with [`Bar::set_spacing`](Bar::set_spacing).
    pub fn set_default_bar_spacing(&mut self, spacing: u8) {
        self.defaults.spacing = spacing;
        self.repaint_surface();
    }
    /// Sets the draw mode of bars that do not specify their own.
    ///
    /// A bar can override this with [`Bar::set_draw_mode`](Bar::set_draw_mode).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.set_default_bar_drawmode(hbarchart::BarDrawMode::Fill(hbarchart::BarFillType::Shade50));
    /// ```
    pub fn set_default_bar_drawmode(&mut self, draw_mode: BarDrawMode) {
        self.defaults.draw_mode = draw_mode;
        self.repaint_surface();
    }
    /// Sets the character attribute of bars that do not specify their own.
    ///
    /// After this call, bars no longer use the theme bar color. A bar can still
    /// override the default with [`Bar::set_attr`](Bar::set_attr).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.set_default_bar_attr(CharAttribute::with_fore_color(Color::Yellow));
    /// ```
    pub fn set_default_bar_attr(&mut self, attr: CharAttribute) {
        self.defaults.attr = attr;
        self.use_theme_colors_for_bars = false;
        self.repaint_surface();
    }
    /// Shows or hides the X axis and the two rows reserved for its line and numeric labels.
    ///
    /// The grid is controlled separately by [`Self::set_xaxis_show_grid`]. Hiding the
    /// axis gives those rows back to the plot.
    pub fn set_xaxis_visible(&mut self, visible: bool) {
        self.xaxis.visible = visible;
        self.repaint_surface();
    }
    /// Shows or hides the vertical grid lines and the numeric labels under them.
    pub fn set_xaxis_show_grid(&mut self, show_grid: bool) {
        self.xaxis.show_grid = show_grid;
        self.repaint_surface();
    }
    /// Sets the distance, in columns, between vertical grid lines.
    ///
    /// Values below 1 are treated as 1. Each numeric label under the grid is
    /// truncated to this many characters. The default is 3.
    pub fn set_xaxis_step(&mut self, step: u8) {
        self.xaxis.step = step.max(1);
        self.repaint_surface();
    }
    /// Sets how labels are drawn beside the bars.
    ///
    /// * [`YAxisLabelMode::None`] draws no category axis.
    /// * [`YAxisLabelMode::Index`] labels each bar with `start + bar index`.
    /// * [`YAxisLabelMode::BarLabels`] uses each bar's own label and skips empty ones.
    /// * [`YAxisLabelMode::Custom`] copies the given spans. Spans are ordered by
    ///   start index, then by end index. A span that overlaps an earlier one is dropped.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = HBarChart::<i32>::new(layout!("d:f"), hbarchart::Flags::None);
    /// chart.add_bars(1..=6);
    /// let spans = [hbarchart::BarSpan::new(0, 3, "Q1"), hbarchart::BarSpan::new(3, 3, "Q2")];
    /// chart.set_yaxis_label_mode(hbarchart::YAxisLabelMode::Custom(&spans));
    /// ```
    pub fn set_yaxis_label_mode(&mut self, yaxis: YAxisLabelMode) {
        match yaxis {
            YAxisLabelMode::None => self.yaxis.label_format = YAxisLabelFormat::None,
            YAxisLabelMode::Index(start) => self.yaxis.label_format = YAxisLabelFormat::Index(start),
            YAxisLabelMode::BarLabels => self.yaxis.label_format = YAxisLabelFormat::BarLabels,
            YAxisLabelMode::Custom(spans) => {
                self.yaxis.label_format = YAxisLabelFormat::Custom;
                self.yaxis.spans.clear();
                self.yaxis.spans.extend(spans);
                self.yaxis.spans.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
                let spans = &mut self.yaxis.spans;
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
    fn plot_rows(&self) -> i32 {
        let height = self.size().height as i32;
        if self.xaxis.visible {
            height.saturating_sub(2)
        } else {
            height
        }
    }
    #[inline(always)]
    fn y_label_margin(&self) -> i32 {
        if self.yaxis.label_format.is_none() {
            return 0;
        }
        self.yaxis.width.max(1) as i32 + 1
    }
    #[inline(always)]
    fn visible_length(&self) -> u32 {
        // one space from the right and the columns reserved for the Y labels
        self.size().width.saturating_sub(self.y_label_margin() as u32 + 1)
    }
    fn update_xaxis_scale(&mut self, left_value: f64, right_value: f64) {
        let width = self.visible_length() as f64;
        self.xaxis.left_value = left_value;
        self.xaxis.left_step = if width > 0.0 {
            (right_value - left_value) / width * (self.xaxis.step as f64)
        } else {
            0.0
        };
    }
    fn update_bars_length_from_zero(&mut self, min: f64, max: f64) {
        let lo = min.min(0.0);
        let hi = max.max(0.0);
        let total = hi - lo;
        self.update_xaxis_scale(lo, hi);
        if total > 0.0 {
            let width = self.visible_length() as f64;
            let cells_left = (-lo / total * width).round();
            let cells_right = width - cells_left;
            self.xaxis.zero = cells_left as i32;
            for item in self.bars.iter_mut() {
                let v = item.bar.value.to_f64();
                let len = if v >= 0.0 {
                    if hi > 0.0 {
                        v / hi * cells_right
                    } else {
                        0.0
                    }
                } else {
                    -(v / lo * cells_left)
                };
                item.len = len.round() as i16;
            }
        }
    }
    fn update_bars_length_fit_data(&mut self, min: f64, max: f64) {
        let width = self.visible_length() as f64;
        let range = max - min;
        self.xaxis.zero = 0;
        self.update_xaxis_scale(min, max);
        if range > 0.0 {
            for item in self.bars.iter_mut() {
                let v = item.bar.value.to_f64();
                item.len = ((v - min) / range * width).trunc() as i16;
            }
        } else {
            let uniform = (width * 0.5).trunc() as i16;
            for item in self.bars.iter_mut() {
                item.len = uniform;
            }
        }
    }
    fn update_bars_length_fixed(&mut self, min: f64, max: f64) {
        let width = self.visible_length() as f64;
        let total = max - min;
        self.update_xaxis_scale(min, max);
        if total <= 0.0 {
            self.xaxis.zero = 0;
            let uniform = (width * 0.5).trunc() as i16;
            for item in self.bars.iter_mut() {
                item.len = uniform;
            }
            return;
        }
        let zero_cell = ((0.0 - min) / total * width).clamp(0.0, width);
        let zero_vis = zero_cell;
        self.xaxis.zero = zero_vis.round() as i32;
        for item in self.bars.iter_mut() {
            let v = item.bar.value.to_f64();
            let tip = ((v - min) / total * width).clamp(0.0, width);
            item.len = (tip - zero_vis).trunc() as i16;
        }
    }
    fn update_bars_layout(&mut self) {
        if self.bars.is_empty() {
            self.bars_height = 0;
            return;
        }
        let mut y = 0i32;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;
        for item in self.bars.iter_mut() {
            y += item.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
            item.pos = y;
            item.len = 0;
            y += item.bar.actual_thickness(&self.defaults) as i32;
            let value = item.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        self.bars_height = y as u32 + self.bars[0].bar.spacing.unwrap_or(self.defaults.spacing) as u32;
        match self.scale {
            BarScale::FromZero => self.update_bars_length_from_zero(v_min, v_max),
            BarScale::FromZeroMinRange { min, max } => self.update_bars_length_from_zero(min.to_f64().min(v_min), max.to_f64().max(v_max)),
            BarScale::FitData => self.update_bars_length_fit_data(v_min, v_max),
            BarScale::Fixed { min, max } => self.update_bars_length_fixed(min.to_f64(), max.to_f64()),
        }
    }
    fn repaint_surface(&mut self) {
        let theme = self.theme();
        let background = theme.chart.background;
        let bar_attr = theme.chart.bar;
        let grid_attr = theme.chart.grid;
        let axis_attr = theme.chart.axis;
        let label_attr = theme.chart.label;
        self.update_bars_layout();
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', background));
        self.paint_value_axis(axis_attr, grid_attr, label_attr);
        self.paint_category_axis(axis_attr, label_attr);
        let plot_rows = self.plot_rows();
        if plot_rows <= 0 || self.bars.is_empty() {
            return;
        }
        let left = self.y_label_margin();
        let width = self.size().width as i32;
        let mut defaults = self.defaults;
        if self.use_theme_colors_for_bars {
            defaults.attr = bar_attr;
        }
        self.surface.set_relative_clip(left, 0, width - 1, plot_rows - 1);
        let mut layout = BarLayout {
            x: left + self.xaxis.zero,
            surface_size: self.size(),
            ..Default::default()
        };
        let len = self.bars.len();
        let mut start = self.first_visible_bar as usize;
        while start < len {
            layout.y = self.bars[start].pos - self.top_scroll;
            if layout.y >= plot_rows {
                break;
            }
            let bar = &self.bars[start];
            layout.length = bar.len;
            bar.bar.paint_horizontal(&mut self.surface, &layout, &defaults);
            start += 1;
        }
    }
    fn paint_value_axis(&mut self, axis_attr: CharAttribute, grid_attr: CharAttribute, label_attr: CharAttribute) {
        let left = self.y_label_margin();
        let right = self.size().width as i32 - 1;
        let plot_rows = self.plot_rows();
        if right < left {
            return;
        }
        if self.xaxis.show_grid && self.xaxis.step > 0 && plot_rows > 0 {
            let ch = Character::with_attributes('┊', grid_attr); //┊ or ⁞
            let mut buffer: [u8; 32] = [0u8; 32];
            let mut x = left;
            let mut value = self.xaxis.left_value;
            let format = self.number_format;
            let bottom = plot_rows - 1;
            let label_y = plot_rows + 1;
            let label_width = self.xaxis.step as i32;
            while x <= right {
                self.surface.fill_vertical_line(x, 0, bottom, ch);
                if self.xaxis.visible {
                    if let Some(text) = format.write_float(value, &mut buffer) {
                        self.write_axis_value(x, label_y, label_width, text, label_attr);
                    }
                }
                x += self.xaxis.step as i32;
                value += self.xaxis.left_step;
            }
            if self.flags.contains(Flags::ShowZeroLineOnXAxis) && self.xaxis.left_step != 0.0 {
                let x_zero = left + ((-self.xaxis.left_value) / self.xaxis.left_step * (self.xaxis.step as f64)).round() as i32;
                if (left..=right).contains(&x_zero) {
                    self.surface.draw_vertical_line(x_zero, 0, bottom, LineType::Single, grid_attr);
                    if self.xaxis.visible {
                        if let Some(text) = format.write_float(0f64, &mut buffer) {
                            self.write_axis_value(x_zero, label_y, label_width, text, label_attr);
                        }
                    }
                }
            }
        }
        if self.xaxis.visible && plot_rows >= 0 && self.size().height >= 2 {
            let axis_x = if left > 0 { left - 1 } else { 0 };
            self.surface.draw_horizontal_line(axis_x, plot_rows, right, LineType::Single, axis_attr);
        }
    }
    fn write_axis_value(&mut self, tick_x: i32, y: i32, label_width: i32, text: &str, attr: CharAttribute) {
        let mut end = text.len();
        let mut count = 0i32;
        for (byte_idx, _) in text.char_indices() {
            if count == label_width {
                end = byte_idx;
                break;
            }
            count += 1;
        }
        let shown = &text[..end];
        self.surface.write_ascii(tick_x - count / 2, y, shown.as_bytes(), attr, false);
    }
    fn paint_category_axis(&mut self, axis_attr: CharAttribute, label_attr: CharAttribute) {
        let margin = self.y_label_margin();
        let plot_rows = self.plot_rows();
        if margin > 0 {
            let x = margin - 1;
            if plot_rows > 0 {
                self.surface.draw_vertical_line(x, 0, plot_rows - 1, LineType::Single, axis_attr);
            }
            if self.xaxis.visible && self.size().height >= 2 {
                self.surface.write_char(
                    x,
                    plot_rows,
                    Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, axis_attr),
                );
            }
        }
        match self.yaxis.label_format {
            YAxisLabelFormat::None => {}
            YAxisLabelFormat::Index(start) => self.paint_index_labels(start, label_attr),
            YAxisLabelFormat::BarLabels => self.paint_bar_labels(label_attr),
            YAxisLabelFormat::Custom => self.paint_span_labels(label_attr),
        }
    }
    fn paint_index_labels(&mut self, start: i32, attr: CharAttribute) {
        let mut temp: [u8; 16] = [0u8; 16];
        let len = self.bars.len();
        let mut idx = self.first_visible_bar as usize;
        let mut index = start as i64 + self.first_visible_bar as i64;
        let plot_rows = self.plot_rows();
        while idx < len {
            let y = self.bar_label_row(idx);
            if y >= plot_rows {
                break;
            }
            if let Some(text) = INT_FORMAT.write_number(index, &mut temp) {
                self.write_category_label(y, text, attr);
            }
            idx += 1;
            index += 1;
        }
    }
    fn paint_bar_labels(&mut self, attr: CharAttribute) {
        let len = self.bars.len();
        let mut idx = self.first_visible_bar as usize;
        let plot_rows = self.plot_rows();
        while idx < len {
            let y = self.bar_label_row(idx);
            if y >= plot_rows {
                break;
            }
            let label = self.bars[idx].bar.label.clone();
            if !label.is_empty() {
                self.write_category_label(y, &label, attr);
            }
            idx += 1;
        }
    }
    fn paint_span_labels(&mut self, attr: CharAttribute) {
        let len = self.bars.len();
        if len == 0 {
            return;
        }
        let plot_rows = self.plot_rows();
        let count = self.yaxis.spans.len();
        for i in 0..count {
            let span = self.yaxis.spans[i];
            let start = span.start as usize;
            if start >= len {
                break;
            }
            let top = self.bars[start].pos - self.top_scroll;
            if top >= plot_rows {
                break;
            }
            let end = (span.end as usize).min(len - 1);
            let bottom = self.bars[end].pos - self.top_scroll + self.bars[end].bar.actual_thickness(&self.defaults) as i32 - 1;
            if bottom < 0 {
                continue;
            }
            self.write_category_label((top + bottom) / 2, span.label.as_str(), attr);
        }
    }
    fn bar_label_row(&self, index: usize) -> i32 {
        let item = &self.bars[index];
        let thickness = item.bar.actual_thickness(&self.defaults) as i32;
        item.pos - self.top_scroll + (thickness - 1) / 2
    }
    fn write_category_label(&mut self, y: i32, label: &str, attr: CharAttribute) {
        let plot_rows = self.plot_rows();
        if y < 0 || y >= plot_rows {
            return;
        }
        let margin = self.y_label_margin();
        if margin <= 1 || label.is_empty() {
            return;
        }
        let max_chars = (margin - 1) as usize;
        let mut end = label.len();
        let mut count = 0usize;
        let mut truncated = false;
        for (byte_idx, _) in label.char_indices() {
            if count == max_chars {
                end = byte_idx;
                truncated = true;
                break;
            }
            count += 1;
        }
        if count == 0 {
            return;
        }
        let x = margin - 1 - count as i32;
        self.surface.write_string(x, y, &label[..end], attr, false);
        if truncated {
            self.surface
                .write_char(margin - 2, y, Character::with_attributes(SpecialChar::ThreePointsHorizontal, attr));
        }
    }
    fn bar_rect(&self, index: usize) -> Rect {
        let bar = &self.bars[index];
        let left_margin = self.y_label_margin();
        let y = bar.pos - self.top_scroll;
        let thickness = bar.bar.actual_thickness(&self.defaults) as u16;
        let baseline = left_margin + self.xaxis.zero;
        let len = bar.len;
        if len == 0 {
            Rect::with_size(baseline, y, 1, thickness)
        } else if len > 0 {
            Rect::with_size(baseline + 1, y, len as u16, thickness)
        } else {
            let abs_len = len.unsigned_abs();
            Rect::with_size(baseline + 1 + len as i32, y, abs_len, thickness)
        }
    }    
    fn clamp_selected_bar(&mut self) {
        if let Some(index) = self.selected_bar {
            if (index as usize) >= self.bars.len() {
                self.selected_bar = None;
            }
        }
    }
    #[inline(always)]
    fn update_first_visible_bar(&mut self) {
        self.first_visible_bar = (self.bars.partition_point(|b| b.pos < self.top_scroll) as u32).saturating_sub(1);
    }
    #[inline(always)]
    fn y_axis_bottom_margin(&self) -> i32 {
        if self.xaxis.visible {
            2
        } else {
            0
        }
    }
    fn sync_vertical_scrollbar(&mut self) {
        let sz = self.size();
        self.scrollbars
            .update(sz.width as u64, self.bars_height as u64 + self.y_axis_bottom_margin() as u64 + 1, sz);
        self.scrollbars.set_indexes(0, self.top_scroll as u64);
    }
    #[inline(always)]
    fn max_top_scroll(&self) -> i32 {
        let content_height = self.bars_height as i32 + self.y_axis_bottom_margin() + 1;
        (content_height - self.size().height as i32).max(0)
    }
    fn after_vertical_scroll(&mut self) {
        self.update_first_visible_bar();
        self.sync_vertical_scrollbar();
        self.repaint_surface();
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.top_scroll = self.scrollbars.vertical_index() as i32;
        // binary search - cea mai apropiata bara
        self.update_first_visible_bar();
        self.repaint_surface();
    }    
    fn align_scroll_to_first_visible_bar(&mut self) {
        if let Some(bar) = self.bars.get(self.first_visible_bar as usize) {
            self.top_scroll = bar.pos.max(0);
        }
        self.sync_vertical_scrollbar();
        self.repaint_surface();
    }
    fn show_hovered_bar_tooltip(&mut self, index: u32) {
        let Some(bar) = self.bars.get(index as usize) else {
            return;
        };
        let r = self.bar_rect(index as usize);
        let mut buf = [0u8; 64];
        let value = self.number_format.write_float(bar.bar.value.to_f64(), &mut buf).unwrap_or("");
        if bar.bar.label.is_empty() {
            self.show_tooltip_on_rect(value, &r);
        } else {
            self.tooltip_text.clear();
            self.tooltip_text.push_str(&bar.bar.label);
            self.tooltip_text.push('\n');
            self.tooltip_text.push_str(value);
            self.show_tooltip_on_rect(&self.tooltip_text, &r);
        }
    }    
    fn clear_hovered_bar(&mut self) {
        if self.hovered_bar.take().is_some() {
            self.hide_tooltip();
        }
    }  
    fn bar_index_at(&self, x: i32, y: i32) -> Option<u32> {
        let left_margin = self.y_label_margin();
        let plot_bottom = self.plot_rows() - 1;
        if x < left_margin || y < 0 || y > plot_bottom {
            return None;
        }
        let content_y = y + self.top_scroll;
        let idx = self.bars.partition_point(|b| b.pos <= content_y).saturating_sub(1);
        let bar = self.bars.get(idx)?;
        let thickness = bar.bar.actual_thickness(&self.defaults) as i32;
        if content_y < bar.pos || content_y >= bar.pos + thickness {
            return None;
        }
        let baseline = left_margin + self.xaxis.zero;
        let len = bar.len;
        let (left, right) = if len == 0 {
            (baseline, baseline)
        } else if len > 0 {
            (baseline + 1, baseline + len as i32)
        } else {
            (baseline + 1 + len as i32, baseline)
        };
        if x < left || x > right {
            return None;
        }
        Some(idx as u32)
    }  
    fn update_selected_bar_from_click(&mut self, x: i32, y: i32) {
        match self.bar_index_at(x, y) {
            Some(index) => {
                if self.selected_bar != Some(index) {
                    self.selected_bar = Some(index);
                    self.raise_bar_selected_event(index);
                }
            }
            None => {
                if self.selected_bar.is_some() {
                    self.selected_bar = None;
                    self.raise_clear_selection_event();
                }
            }
        }
    }  
    fn raise_bar_selected_event(&mut self, index: u32) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HBarChart(EventData {
                event_type: EventType::BarSelected(index),
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn raise_clear_selection_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HBarChart(EventData {
                event_type: EventType::ClearSelection,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }          
}

impl<T> OnPaint for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 0);
        }
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        } else {
            surface.draw_surface(0, 0, &self.surface);
            if let Some(index) = self.selected_bar {
                // clip to the plot area bounded by the category and value axes (inclusive)
                let margin = self.y_label_margin();
                let left = if margin > 0 { margin - 1 } else { 0 };
                let bottom = self.size().height as i32 - if self.xaxis.visible { 2 } else { 1 };
                let right = self.size().width.saturating_sub(1) as i32;
                surface.set_relative_clip(left, 0, right, bottom);
                let mut r = self.bar_rect(index as usize);
                r.inflate_width(1, 1, 1, 1);
                surface.draw_rect(r, LineType::Single, theme.chart.selection_border);
                if self.flags.contains(Flags::DimBarsOnSelection) {
                    let attr = theme.chart.inactive;
                    surface.transform_rect(Rect::new(left, 0, right, bottom), |ch, p| {
                        if r.contains(p) {
                            None
                        } else {
                            Some(Character::with_attributes(ch.code, attr))
                        }
                    });
                }
            }
        }
    }
}

impl<T> OnResize for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_bars_layout();
        self.scrollbars.resize(
            new_size.width as u64,
            self.bars_height as u64 + self.y_axis_bottom_margin() as u64 + 1,
            &self.base,
        );
        self.update_scroll_pos_from_scrollbars();
    }
}

impl<T> OnMouseEvent for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.clear_hovered_bar();
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.clear_hovered_bar();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(pos) => {
                let idx = self.bar_index_at(pos.x, pos.y);
                if idx != self.hovered_bar {
                    self.hovered_bar = idx;
                    if let Some(index) = idx {
                        self.show_hovered_bar_tooltip(index);
                    } else {
                        self.hide_tooltip();
                    }
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(data) | MouseEvent::DoubleClick(data) => {
                self.update_selected_bar_from_click(data.x, data.y);
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(wheel) => match wheel {
                MouseWheelDirection::Left | MouseWheelDirection::Up => {
                    if self.top_scroll > 0 {
                        self.top_scroll -= 1;
                        self.after_vertical_scroll();
                    }
                    EventProcessStatus::Processed
                }
                MouseWheelDirection::Right | MouseWheelDirection::Down => {
                    if self.top_scroll < self.max_top_scroll() {
                        self.top_scroll += 1;
                        self.after_vertical_scroll();
                    }
                    EventProcessStatus::Processed
                }
            },
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnKeyPressed for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                if self.top_scroll > 0 {
                    self.top_scroll -= 1;
                    self.after_vertical_scroll();
                }
                EventProcessStatus::Processed
            }
            key!("Down") => {
                if self.top_scroll < self.max_top_scroll() {
                    self.top_scroll += 1;
                    self.after_vertical_scroll();
                }
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.top_scroll = 0;
                self.after_vertical_scroll();
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.top_scroll = self.max_top_scroll();
                self.after_vertical_scroll();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Up") => {
                if self.first_visible_bar > 0 {
                    self.first_visible_bar -= 1;
                    self.align_scroll_to_first_visible_bar();
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") => {
                if !self.bars.is_empty() && (self.first_visible_bar as usize + 1) < self.bars.len() {
                    self.first_visible_bar += 1;
                    self.align_scroll_to_first_visible_bar();
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

use super::{Bar, BarDrawMode, BarScale, BarSpan, Flags, YAxisLabelFormat, YAxisLabelMode};
use crate::prelude::*;
use crate::ui::components::{BarDefaults, BarLayout, ScrollBars};

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

fn index_width(value: i64) -> i32 {
    let mut n = value.unsigned_abs();
    let mut width = if value < 0 { 2 } else { 1 };
    while n >= 10 {
        n /= 10;
        width += 1;
    }
    width
}

struct BarWithLayout<T: Number + 'static> {
    bar: Bar<T>,
    y: i32,
    len: i16,
}
impl<T> BarWithLayout<T>
where
    T: Number + 'static,
{
    #[inline(always)]
    fn new(bar: Bar<T>) -> Self {
        Self { bar, y: 0, len: 0 }
    }
}

/// A mutable view of the bars stored in an [`HBarChart`].
///
/// `Bars` is passed to [`HBarChart::update_bars`] so several inserts, deletes, and
/// in-place edits can be applied before the chart relayouts and repaints once.
///
/// Custom category spans use bar indices; inserting or deleting bars may require
/// updating those spans afterwards.
pub struct Bars<'a, T>
where
    T: Number + 'static,
{
    inner: &'a mut Vec<BarWithLayout<T>>,
}
impl<'a, T> Bars<'a, T>
where
    T: Number + 'static,
{
    /// Returns the number of bars in the series.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if the series contains no bars.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Returns an immutable reference to the bar at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&Bar<T>> {
        self.inner.get(index).map(|item| &item.bar)
    }
    /// Returns a mutable reference to the bar at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar<T>> {
        self.inner.get_mut(index).map(|item| &mut item.bar)
    }
    /// Appends a bar at the end of the series.
    #[inline(always)]
    pub fn add<B>(&mut self, bar: B)
    where
        B: Into<Bar<T>>,
    {
        self.inner.push(BarWithLayout::new(bar.into()));
    }
    /// Appends several bars at the end of the series.
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.inner.extend(bars.into_iter().map(|bar| BarWithLayout::new(bar.into())));
    }
    /// Inserts a bar at `index`. Returns `false` if `index` is greater than [`len`](Self::len).
    pub fn insert<B>(&mut self, index: usize, bar: B) -> bool
    where
        B: Into<Bar<T>>,
    {
        if index > self.inner.len() {
            return false;
        }
        self.inner.insert(index, BarWithLayout::new(bar.into()));
        true
    }
    /// Removes the bar at `index` and returns it, or `None` if out of range.
    pub fn delete(&mut self, index: usize) -> Option<Bar<T>> {
        if index >= self.inner.len() {
            return None;
        }
        Some(self.inner.remove(index).bar)
    }
    /// Replaces the bar at `index` and returns the previous bar, or `None` if out of range.
    pub fn set<B>(&mut self, index: usize, bar: B) -> Option<Bar<T>>
    where
        B: Into<Bar<T>>,
    {
        let slot = self.inner.get_mut(index)?;
        Some(std::mem::replace(&mut slot.bar, bar.into()))
    }
    /// Removes all bars from the series.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Iterates over the bars in the series.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Bar<T>> {
        self.inner.iter().map(|item| &item.bar)
    }
    /// Iterates mutably over the bars in the series.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bar<T>> {
        self.inner.iter_mut().map(|item| &mut item.bar)
    }
}

struct XAxis {
    width: u8,
    step: u8,
    zero: i32,
    left_value: f64,
    left_step: f64,
    visible: bool,
    show_grid: bool,
}

struct YAxis {
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
pub struct HBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    bars: Vec<BarWithLayout<T>>,
    xaxis: XAxis,
    scale: BarScale<T>,
    number_format: FormatNumber,
    defaults: BarDefaults,
    surface: Surface,
    use_theme_colors_for_bars: bool,
    yaxis: YAxis,
    scrollbars: ScrollBars,
    selected_bar: Option<u32>,
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
    /// a thickness of 1, and a spacing of 1 until a default is changed.
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
                width: 6,
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
    /// Scrolls the chart so the bar at `index` is visible.
    ///
    /// Does nothing if `index` is out of range.
    pub fn ensure_visible(&mut self, index: usize) {
        if index >= self.bars.len() {
            return;
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
    /// Shows or hides the X axis and the space reserved for its labels.
    ///
    /// The grid is controlled separately by [`Self::set_xaxis_show_grid`].
    pub fn set_xaxis_visible(&mut self, visible: bool) {
        self.xaxis.visible = visible;
        self.repaint_surface();
    }
    /// Shows or hides the grid lines and the numeric labels beside them.
    pub fn set_xaxis_show_grid(&mut self, show_grid: bool) {
        self.xaxis.show_grid = show_grid;
        self.repaint_surface();
    }
    /// Sets how many characters are reserved for X-axis labels.
    ///
    /// Values below 1 are treated as 1. This width is used only while the X axis
    /// is visible.
    pub fn set_xaxis_width(&mut self, width: u8) {
        self.xaxis.width = width.max(1);
        self.repaint_surface();
    }
    /// Sets the distance, in cells, between grid lines and their labels.
    ///
    /// Values below 1 are treated as 1.
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
        if self.xaxis.visible { height.saturating_sub(2) } else { height }
    }
    #[inline(always)]
    fn y_label_margin(&self) -> i32 {
        let cols = match self.yaxis.label_format {
            YAxisLabelFormat::None => 0,
            YAxisLabelFormat::Index(start) => {
                let last = start as i64 + self.bars.len().saturating_sub(1) as i64;
                index_width(start as i64).max(index_width(last))
            }
            YAxisLabelFormat::BarLabels => self
                .bars
                .iter()
                .map(|item| item.bar.label.chars().count() as i32)
                .max()
                .unwrap_or(0),
            YAxisLabelFormat::Custom => self
                .yaxis
                .spans
                .iter()
                .map(|span| span.label.as_str().chars().count() as i32)
                .max()
                .unwrap_or(0),
        };
        if cols <= 0 {
            return 0;
        }
        let margin = cols + 1;
        let limit = (self.size().width as i32 / 2).max(1);
        margin.min(limit)
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
                    if hi > 0.0 { v / hi * cells_right } else { 0.0 }
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
            return;
        }
        let mut y = 0i32;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;
        for item in self.bars.iter_mut() {
            y += item.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
            item.y = y;
            item.len = 0;
            y += item.bar.actual_thickness(&self.defaults) as i32;
            let value = item.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        match self.scale {
            BarScale::FromZero => self.update_bars_length_from_zero(v_min, v_max),
            BarScale::FromZeroMinRange { min, max } => {
                self.update_bars_length_from_zero(min.to_f64().min(v_min), max.to_f64().max(v_max))
            }
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
        for item in &self.bars {
            layout.y = item.y;
            layout.length = item.len;
            item.bar.paint_horizontal(&mut self.surface, &layout, &defaults);
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
            let label_width = self.xaxis.width as i32;
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
                        if let Some(text) = format.write_float(value, &mut buffer) {
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
        let mut index = start as i64;
        for i in 0..len {
            if let Some(text) = INT_FORMAT.write_number(index, &mut temp) {
                let y = self.bar_label_row(i);
                self.write_category_label(y, text, attr);
            }
            index += 1;
        }
    }
    fn paint_bar_labels(&mut self, attr: CharAttribute) {
        let len = self.bars.len();
        for i in 0..len {
            let label = self.bars[i].bar.label.clone();
            if label.is_empty() {
                continue;
            }
            let y = self.bar_label_row(i);
            self.write_category_label(y, &label, attr);
        }
    }
    fn paint_span_labels(&mut self, attr: CharAttribute) {
        let len = self.bars.len();
        if len == 0 {
            return;
        }
        let count = self.yaxis.spans.len();
        for i in 0..count {
            let span = self.yaxis.spans[i];
            let start = span.start as usize;
            if start >= len {
                break;
            }
            let end = (span.end as usize).min(len - 1);
            let top = self.bars[start].y;
            let bottom = self.bars[end].y + self.bars[end].bar.actual_thickness(&self.defaults) as i32 - 1;
            self.write_category_label((top + bottom) / 2, span.label.as_str(), attr);
        }
    }
    fn bar_label_row(&self, index: usize) -> i32 {
        let item = &self.bars[index];
        let thickness = item.bar.actual_thickness(&self.defaults) as i32;
        item.y + (thickness - 1) / 2
    }
    fn write_category_label(&mut self, y: i32, label: &str, attr: CharAttribute) {
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
            self.surface.write_char(
                margin - 2,
                y,
                Character::with_attributes(SpecialChar::ThreePointsHorizontal, attr),
            );
        }
    }
    fn clamp_selected_bar(&mut self) {
        if let Some(index) = self.selected_bar {
            if (index as usize) >= self.bars.len() {
                self.selected_bar = None;
            }
        }
    }
}

impl<T> OnPaint for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        surface.draw_surface(0, 0, &self.surface);
    }
}

impl<T> OnResize for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.scrollbars.resize(new_size.width as u64, new_size.height as u64, &self.base);
        self.repaint_surface();
    }
}

impl<T> OnMouseEvent for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl<T> OnKeyPressed for HBarChart<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

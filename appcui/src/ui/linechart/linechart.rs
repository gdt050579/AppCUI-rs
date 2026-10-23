use flat_string::FlatString;

use super::{Flags, LineMode, Marker, Series, XAxisLabelFormat, XAxisLabelMode};
use crate::prelude::*;

struct YAxis {
    width: u8,
    step: u8,
    visible: bool,
    show_grid: bool,
}
struct XAxis {
    label_format: XAxisLabelFormat,
    spans: Vec<BarSpan>,
}
/// Geometry computed by `update_layout` and used while painting.
#[derive(Default)]
struct PlotLayout {
    /// width of every column (the widest marker used), in cells
    column_width: u8,
    /// height of the tallest marker used (1 or 2 rows)
    marker_height: u8,
    /// number of distinct rows a marker (its top row) can be drawn on
    levels: i32,
    /// value drawn on level 0 (bottom)
    low: f64,
    /// value drawn on the last level (top)
    high: f64,
    /// total width of all columns (with spacing), in cells
    content_width: u32,
}

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

#[CustomControl(overwrite = OnPaint + OnResize, internal = true)]
/// A line chart for one or more numeric series of type `T`.
///
/// Each [`Series`] is a list of points that share the same X axis, so several lines
/// can be drawn over the same range. The X range is as long as the longest series;
/// shorter series simply end earlier. Optional behavior is controlled by [`Flags`].
///
/// Data is always added to a series by its index, so at least one series must be
/// added with [`add_series`](Self::add_series) before any value.
///
/// All series share the same columns: every column is as wide as the widest [`Marker`]
/// in the chart, and columns are separated by the default spacing. The vertical position
/// of a point is computed from its value and the chart scale ([`BarScale`]), using the
/// minimum and maximum over all series.
pub struct LineChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    series: Vec<Series<T>>,
    default_line: LineMode,
    default_marker: Marker,
    default_spacing: u8,
    scale: BarScale<T>,
    number_format: FormatNumber,
    yaxis: YAxis,
    xaxis: XAxis,
    plot: PlotLayout,
    surface: Surface,
}

impl<T> LineChart<T>
where
    T: Number + 'static,
{
    /// Creates an empty line chart.
    ///
    /// `layout` places the control. `flags` selects optional behavior; see [`Flags`].
    ///
    /// A new chart has no series. Points are drawn with [`Marker::default`], connected with
    /// [`LineMode::default`], and separated by 2 empty cells. The chart uses [`BarScale::FromZero`],
    /// shows the Y axis and its grid, draws no X axis, and formats labels with two decimals when `T`
    /// is a floating-point type or with thousands separators when `T` is an integer.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            series: Vec::new(),
            default_line: LineMode::default(),
            default_marker: Marker::default(),
            default_spacing: 2,
            scale: BarScale::FromZero,
            number_format: if T::is_float() {
                FormatNumber::new(10).decimals(2)
            } else {
                FormatNumber::new(10).group(3, b',')
            },
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
            plot: PlotLayout::default(),
            surface: Surface::new(1, 1),
        }
    }

    /// Returns the flags the chart was created with.
    #[inline(always)]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    // ------------------------------------------------------------------ series

    /// Appends a series and returns its index.
    ///
    /// `series` may be a [`Series`] or a [`SeriesBuilder`](super::SeriesBuilder).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// let cpu = chart.add_series(linechart::Series::with_values("CPU", [10, 14, 9]));
    /// let ram = chart.add_series(linechart::SeriesBuilder::new("RAM").line(linechart::LineMode::Braille));
    /// assert_eq!((cpu, ram), (0, 1));
    /// ```
    pub fn add_series<S>(&mut self, series: S) -> usize
    where
        S: Into<Series<T>>,
    {
        self.series.push(series.into());
        self.repaint_surface();
        self.series.len() - 1
    }

    /// Returns the number of series.
    #[inline(always)]
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Returns the number of points on the X axis: the length of the longest series.
    pub fn points_count(&self) -> usize {
        self.series.iter().map(|s| s.len()).max().unwrap_or(0)
    }

    /// Returns the series at `series_index`, or `None` if out of range.
    #[inline(always)]
    pub fn get_series(&self, series_index: usize) -> Option<&Series<T>> {
        self.series.get(series_index)
    }

    /// Mutates the series at `series_index`, then relayouts and repaints the chart.
    ///
    /// Does nothing if `series_index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::with_values("CPU", [10, 14, 9]));
    /// chart.update_series(0, |s| {
    ///     s.set_line(Some(linechart::LineMode::Braille));
    ///     s.get_mut(1).unwrap().set_value(20);
    /// });
    /// ```
    pub fn update_series<F>(&mut self, series_index: usize, f: F)
    where
        F: FnOnce(&mut Series<T>),
    {
        if let Some(series) = self.series.get_mut(series_index) {
            f(series);
            self.repaint_surface();
        }
    }

    /// Calls `f` for every series (with its index), then relayouts and repaints once.
    ///
    /// Use this to change several series without repainting after each change, for
    /// example to append one new sample to every series of a live chart.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::new("CPU"));
    /// chart.add_series(linechart::Series::new("RAM"));
    /// let sample = [12, 40];
    /// chart.update_all_series(|index, s| {
    ///     s.add(sample[index]);
    ///     if s.len() > 100 {
    ///         s.remove_first(1);
    ///     }
    /// });
    /// ```
    pub fn update_all_series<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut Series<T>),
    {
        if self.series.is_empty() {
            return;
        }
        for (index, series) in self.series.iter_mut().enumerate() {
            f(index, series);
        }
        self.repaint_surface();
    }

    /// Removes the series at `series_index` and returns it, or `None` if out of range.
    ///
    /// The series after it move down by one index.
    pub fn remove_series(&mut self, series_index: usize) -> Option<Series<T>> {
        if series_index >= self.series.len() {
            return None;
        }
        let series = self.series.remove(series_index);
        self.repaint_surface();
        Some(series)
    }

    // ------------------------------------------------------------------ values

    /// Appends one point to the series at `series_index` and repaints the chart.
    ///
    /// `value` may be a value of type `T` or anything that converts into a [`Bar`].
    /// Does nothing if `series_index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::new("CPU"));
    /// chart.add_value(0, 10);
    /// chart.add_value(0, BarBuilder::new(20).label("Feb").build());
    /// ```
    pub fn add_value<B>(&mut self, series_index: usize, value: B)
    where
        B: Into<Bar<T>>,
    {
        if let Some(series) = self.series.get_mut(series_index) {
            series.add(value);
            self.repaint_surface();
        }
    }

    /// Appends several points to the series at `series_index` and repaints the chart once.
    ///
    /// Each item may be a value of type `T` or anything that converts into a [`Bar`].
    /// Does nothing if `series_index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::new("CPU"));
    /// chart.add_values(0, [10, 14, 9, 22]);
    /// ```
    pub fn add_values<B>(&mut self, series_index: usize, values: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        if let Some(series) = self.series.get_mut(series_index) {
            series.add_values(values);
            self.repaint_surface();
        }
    }

    // ------------------------------------------------------------------ defaults

    /// Returns how points are connected for series that do not set their own [`LineMode`].
    #[inline(always)]
    pub fn default_line(&self) -> LineMode {
        self.default_line
    }
    /// Sets how points are connected for series that do not set their own [`LineMode`].
    pub fn set_default_line(&mut self, line: LineMode) {
        self.default_line = line;
        self.repaint_surface();
    }

    /// Returns the marker used for points that do not set their own marker.
    #[inline(always)]
    pub fn default_marker(&self) -> Marker {
        self.default_marker
    }
    /// Sets the marker used for points that do not set their own marker.
    ///
    /// The marker also sets the width of every column (see [`Marker::width`]).
    pub fn set_default_marker(&mut self, marker: Marker) {
        self.default_marker = marker;
        self.repaint_surface();
    }

    /// Returns the number of empty cells between two consecutive points.
    #[inline(always)]
    pub fn default_spacing(&self) -> u8 {
        self.default_spacing
    }
    /// Sets the number of empty cells between two consecutive points.
    ///
    /// This is the only spacing used by the chart: the spacing stored on individual points is ignored,
    /// because all series share the same columns.
    pub fn set_default_spacing(&mut self, spacing: u8) {
        self.default_spacing = spacing;
        self.repaint_surface();
    }

    // ------------------------------------------------------------------ scale & axes

    /// Sets how values are mapped onto the plot height.
    ///
    /// * [`BarScale::FromZero`] uses the range from the smallest to the largest value of all
    ///   series, extended so that it always includes zero.
    /// * [`BarScale::FromZeroMinRange`] does the same, and also extends the range so that it covers `min` and `max`.
    /// * [`BarScale::FitData`] stretches the smallest and largest values across the full plot height.
    /// * [`BarScale::Fixed`] uses the given `min` and `max`. Values outside that range are drawn
    ///   at the corresponding edge of the plot.
    ///
    /// When the range is empty (for example every value is equal with [`BarScale::FitData`]),
    /// all points are drawn in the middle of the plot.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.set_scale(BarScale::Fixed { min: 0, max: 100 });
    /// ```
    pub fn set_scale(&mut self, scale: BarScale<T>) {
        self.scale = scale;
        self.repaint_surface();
    }
    /// Sets the format used for Y-axis labels and the zero-line label.
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
        self.repaint_surface();
    }
    /// Shows or hides the Y axis and the column reserved for its labels.
    ///
    /// The horizontal grid is controlled separately by [`Self::set_yaxis_show_grid`].
    pub fn set_yaxis_visible(&mut self, visible: bool) {
        self.yaxis.visible = visible;
        self.repaint_surface();
    }
    /// Shows or hides the horizontal grid lines and the numeric labels beside them.
    pub fn set_yaxis_show_grid(&mut self, show_grid: bool) {
        self.yaxis.show_grid = show_grid;
        self.repaint_surface();
    }
    /// Sets how many characters are reserved for Y-axis labels. Values below 1 are treated as 1.
    pub fn set_yaxis_width(&mut self, width: u8) {
        self.yaxis.width = width.max(1);
        self.repaint_surface();
    }
    /// Sets the distance, in rows, between horizontal grid lines. Values below 1 are treated as 1.
    pub fn set_yaxis_step(&mut self, step: u8) {
        self.yaxis.step = step.max(1);
        self.repaint_surface();
    }
    /// Sets how labels are drawn under the points.
    ///
    /// * [`XAxisLabelMode::None`] draws no X axis.
    /// * [`XAxisLabelMode::Index`] labels each point with `start + point index`.
    /// * [`XAxisLabelMode::PointLabels`] uses the first non-empty label found among the series at each index.
    /// * [`XAxisLabelMode::Custom`] copies the given spans. Spans are ordered by
    ///   start index, then by end index. A span that overlaps an earlier one is dropped.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::with_values("CPU", 1..=6));
    /// let spans = [BarSpan::new(0, 3, "Q1"), BarSpan::new(3, 3, "Q2")];
    /// chart.set_xaxis_label_mode(linechart::XAxisLabelMode::Custom(&spans));
    /// ```
    pub fn set_xaxis_label_mode(&mut self, mode: XAxisLabelMode) {
        match mode {
            XAxisLabelMode::None => self.xaxis.label_format = XAxisLabelFormat::None,
            XAxisLabelMode::Index(start) => self.xaxis.label_format = XAxisLabelFormat::Index(start),
            XAxisLabelMode::PointLabels => self.xaxis.label_format = XAxisLabelFormat::PointLabels,
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

    // ------------------------------------------------------------------ layout

    /// Maps `value` to a level in `0..levels` for the range `low..=high` (clamped).
    /// An empty range (or a single level) puts every value in the middle.
    pub(super) fn value_to_level(value: f64, low: f64, high: f64, levels: i32) -> i16 {
        if levels <= 1 {
            return 0;
        }
        let range = high - low;
        if range <= 0.0 {
            return ((levels - 1) / 2) as i16;
        }
        let level = ((value - low) / range * (levels - 1) as f64).round();
        level.clamp(0.0, (levels - 1) as f64) as i16
    }
    /// Value drawn on `level` (inverse of `value_to_level`).
    fn level_to_value(&self, level: i32) -> f64 {
        if self.plot.levels <= 1 {
            return self.plot.low;
        }
        self.plot.low + (self.plot.high - self.plot.low) * level as f64 / (self.plot.levels - 1) as f64
    }
    /// Marker used by a point: its own Point/LargePoint draw mode, or the chart default.
    #[inline(always)]
    fn marker_for(&self, bar: &Bar<T>) -> Marker {
        bar.draw_mode.and_then(Marker::from_draw_mode).unwrap_or(self.default_marker)
    }
    #[inline(always)]
    fn x_axis_left_margin(&self) -> i32 {
        if self.yaxis.visible && self.yaxis.width > 0 {
            self.yaxis.width as i32 + 2
        } else {
            0
        }
    }
    /// Last row of the plot area (just above the X axis).
    #[inline(always)]
    fn plot_bottom(&self) -> i32 {
        self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1)
    }
    /// Row of the top of a marker drawn on level 0.
    #[inline(always)]
    fn level0_row(&self) -> i32 {
        self.plot_bottom() - (self.plot.marker_height as i32 - 1)
    }
    /// Distance, in cells, between the starts of two consecutive columns.
    #[inline(always)]
    fn column_step(&self) -> i32 {
        self.plot.column_width as i32 + self.default_spacing as i32
    }
    /// X coordinate (in the content, before scrolling and the left margin) of column `index`.
    #[inline(always)]
    fn column_x(&self, index: usize) -> i32 {
        self.default_spacing as i32 + index as i32 * self.column_step()
    }
    fn update_layout(&mut self) {
        let points = self.points_count();
        // column width & marker height: the largest marker used anywhere
        let mut width = self.default_marker.width();
        let mut height = self.default_marker.height();
        let mut v_min = f64::MAX;
        let mut v_max = f64::MIN;
        for series in self.series.iter() {
            for bar in series.iter() {
                let marker = self.marker_for(bar);
                width = width.max(marker.width());
                height = height.max(marker.height());
                let value = bar.value.to_f64();
                v_min = v_min.min(value);
                v_max = v_max.max(value);
            }
        }
        if v_min > v_max {
            // no values
            v_min = 0.0;
            v_max = 0.0;
        }
        self.plot.column_width = width;
        self.plot.marker_height = height;
        self.plot.content_width = if points == 0 {
            0
        } else {
            (self.default_spacing as u32) + (points as u32) * (width as u32 + self.default_spacing as u32)
        };
        // one empty row at the top, the X axis (if any) at the bottom
        let plot_rows = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        self.plot.levels = (plot_rows - (height as i32 - 1)).max(0);
        let (low, high) = match self.scale {
            BarScale::FromZero => (v_min.min(0.0), v_max.max(0.0)),
            BarScale::FromZeroMinRange { min, max } => (min.to_f64().min(v_min).min(0.0), max.to_f64().max(v_max).max(0.0)),
            BarScale::FitData => (v_min, v_max),
            BarScale::Fixed { min, max } => (min.to_f64(), max.to_f64()),
        };
        self.plot.low = low;
        self.plot.high = high;
        let levels = self.plot.levels;
        for series in self.series.iter_mut() {
            for point in series.data.iter_mut() {
                point.len = Self::value_to_level(point.bar.value.to_f64(), low, high, levels);
            }
        }
    }

    // ------------------------------------------------------------------ painting

    /// Recomputes the layout and repaints the internal surface.
    fn repaint_surface(&mut self) {
        let theme = self.theme();
        let background = theme.chart.background;
        let axis_attr = theme.chart.axis;
        let grid_attr = theme.chart.grid;
        let label_attr = theme.chart.label;
        let series_colors = theme.chart.bar;
        self.update_layout();
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', background));
        // always Y first, then X (the X axis overwrites the bottom of the Y axis)
        self.paint_yaxis(axis_attr, grid_attr, label_attr);
        self.paint_xaxis(axis_attr, label_attr);
        self.paint_markers(&series_colors);
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
            self.surface.draw_vertical_line(self.yaxis.width as i32 + 1, 0, bottom, LineType::Single, axis_attr);
            if !self.xaxis.label_format.is_none() {
                self.surface.write_char(
                    self.yaxis.width as i32 + 1,
                    bottom,
                    Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, axis_attr),
                );
            }
        }
        if !self.yaxis.show_grid || self.yaxis.step == 0 || self.plot.levels == 0 {
            return;
        }
        let x_poz = self.x_axis_left_margin();
        let right = self.size().width as i32;
        let level0_row = self.level0_row();
        let ch = Character::with_attributes('┈', grid_attr);
        let mut buffer: [u8; 32] = [0u8; 32];
        let mut level = 0;
        while level < self.plot.levels {
            let y = level0_row - level;
            self.surface.fill_horizontal_line(x_poz, y, right, ch);
            if let Some(value) = self.number_format.write_float(self.level_to_value(level), &mut buffer) {
                self.surface.write_ascii(x_poz - value.len() as i32 - 1, y, value.as_bytes(), label_attr, false);
            }
            level += self.yaxis.step as i32;
        }
        if self.flags.contains(Flags::ShowZeroLineOnYAxis) && self.plot.low <= 0.0 && self.plot.high >= 0.0 && self.plot.high > self.plot.low {
            let y = level0_row - Self::value_to_level(0.0, self.plot.low, self.plot.high, self.plot.levels) as i32;
            self.surface.draw_horizontal_line(x_poz, y, right, LineType::Single, grid_attr);
            if let Some(value) = self.number_format.write_float(0f64, &mut buffer) {
                self.surface.write_ascii(x_poz - value.len() as i32 - 1, y, value.as_bytes(), label_attr, false);
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
            XAxisLabelFormat::PointLabels => self.paint_xaxis_point_labels(label_attr),
            XAxisLabelFormat::Custom => self.paint_xaxis_custom(label_attr),
        }
    }
    /// Writes `label` centered under the columns `start..=end` (truncated with `…` if it does not fit).
    fn print_label(&mut self, start: usize, end: usize, label: &str, attr: CharAttribute) {
        let x = self.column_x(start) + self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        let width = (end - start) as i32 * self.column_step() + self.plot.column_width as i32;
        let space = self.default_spacing as i32;
        // odd: 5 -> 5/2 - 0 = 2; even: 6 -> 6/2 - 1 = 2
        let extra_space = space / 2 - (1 - (space & 1));
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
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        for index in 0..self.points_count() {
            if self.column_x(index) + left_margin >= width {
                break;
            }
            if let Some(result) = INT_FORMAT.write_number(start as i64 + index as i64, &mut temp) {
                self.print_label(index, index, result, attr);
            }
        }
    }
    fn paint_xaxis_point_labels(&mut self, attr: CharAttribute) {
        let mut temp_str: FlatString<254> = FlatString::new();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        for index in 0..self.points_count() {
            if self.column_x(index) + left_margin >= width {
                break;
            }
            let label = self
                .series
                .iter()
                .filter_map(|s| s.get(index))
                .map(|b| b.label.as_str())
                .find(|l| !l.is_empty());
            if let Some(label) = label {
                temp_str.set(label);
                self.print_label(index, index, temp_str.as_str(), attr);
            }
        }
    }
    fn paint_xaxis_custom(&mut self, attr: CharAttribute) {
        let len = self.points_count();
        if len == 0 {
            return;
        }
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        for i in 0..self.xaxis.spans.len() {
            let span = self.xaxis.spans[i]; // only 32 bytes - cheap to copy
            let start_index = span.start as usize;
            if start_index >= len || self.column_x(start_index) + left_margin >= width {
                break;
            }
            let end_index = (span.end as usize).min(len - 1);
            self.print_label(start_index, end_index, span.label.as_str(), attr);
        }
    }
    fn paint_markers(&mut self, series_colors: &[CharAttribute]) {
        let left_margin = self.x_axis_left_margin();
        let width = self.size().width as i32;
        let level0_row = self.level0_row();
        let column_width = self.plot.column_width as i32;
        if self.plot.levels == 0 {
            return;
        }
        self.surface.set_relative_clip(left_margin, 0, width, self.plot_bottom());
        for (series_index, series) in self.series.iter().enumerate() {
            let series_attr = series.attr().unwrap_or(series_colors[series_index % series_colors.len()]);
            for (index, point) in series.data.iter().enumerate() {
                let x = self.column_x(index) + left_margin;
                if x >= width {
                    break;
                }
                let y = level0_row - point.len as i32;
                let attr = point.bar.attr.unwrap_or(series_attr);
                match self.marker_for(&point.bar) {
                    Marker::None => {}
                    Marker::Small(point_type) => {
                        // centered in the column (columns are wider when large markers are used)
                        self.surface.write_char(x + (column_width - 1) / 2, y, point_type.character(attr));
                    }
                    Marker::Large(point_type) => {
                        let chars = point_type.characters();
                        for (i, ch) in chars.iter().enumerate() {
                            let dx = (i % 3) as i32;
                            let dy = (i / 3) as i32;
                            self.surface.write_char(x + dx, y + dy, Character::with_attributes(*ch, attr));
                        }
                    }
                }
            }
        }
        self.surface.reset_clip();
    }
}

impl<T> OnPaint for LineChart<T>
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

impl<T> OnResize for LineChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.repaint_surface();
    }
}

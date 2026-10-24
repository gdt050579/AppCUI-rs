use flat_string::FlatString;

use super::{
    events::{EventData, EventType},
    Flags, LineMode, Marker, Series, XAxisLabelFormat, XAxisLabelMode,
};

/// A point of the chart: a series and the index of the point inside it.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) struct PointPos {
    pub(super) series: u32,
    pub(super) index: u32,
}
impl PointPos {
    #[inline(always)]
    pub(super) const fn new(series: u32, index: u32) -> Self {
        Self { series, index }
    }
}

/// What is under the mouse.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum Hover {
    /// a whole X position (with [`Flags::GroupHover`])
    Group(usize),
    /// a single point
    Point(PointPos),
}
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
    /// names of the X positions (used by `XAxisLabelMode::Labels` and in tooltips)
    labels: Vec<String>,
}
/// Geometry computed by `update_layout` and used while painting.
#[derive(Default)]
struct PlotLayout {
    /// width of every column (the widest marker used), in cells
    column_width: u8,
    /// height of the tallest marker used (1 or 3 rows); its middle row is on the value
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

#[CustomControl(overwrite = OnPaint + OnResize + OnMouseEvent + OnKeyPressed, internal = true)]
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
///
/// When the points are wider than the control, the chart scrolls horizontally with the
/// arrow keys (`Left`/`Right` by one cell, `Ctrl+Left`/`Ctrl+Right` by one point,
/// `Home`/`End`) and the mouse wheel. With [`Flags::ScrollBars`], a horizontal scroll bar
/// is shown while the chart has focus.
///
/// The names of the X positions (for example months) belong to the chart, not to the points:
/// set them with [`set_xaxis_labels`](Self::set_xaxis_labels) and show them under the points with
/// [`XAxisLabelMode::Labels`]. The label of an individual point is a note shown only in its tooltip.
///
/// Hovering a point with the mouse shows a tooltip with its series, X label, value, and note.
/// With [`Flags::GroupHover`], hovering acts on a whole X position instead: the column is
/// highlighted and the tooltip shows the value of every series at that position.
///
/// Clicking a point selects it (one series, one index) and raises a point-selected event;
/// clicking away from every point clears the selection. While a point is selected,
/// `Left`/`Right` move along its series, `Up`/`Down` move to the point above or below at
/// the same X position, and `Escape` clears the selection. With [`Flags::DimPointsOnSelection`],
/// everything except the selected point is drawn with the inactive chart color.
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
    scrollbars: ScrollBars,
    /// horizontal scroll, in cells (0 = the first column is at the left edge of the plot)
    pub(super) left_scroll: i32,
    /// index of the first column that is (at least partially) visible
    pub(super) first_visible_point: usize,
    /// what is under the mouse, if anything
    pub(super) hovered: Option<Hover>,
    pub(super) tooltip_text: String,
    /// selected point, if any
    pub(super) selected_point: Option<PointPos>,
    /// junctions to draw on the sides of framed markers: (series index, position, character),
    /// computed while drawing the lines and applied right after the markers of that series
    junctions: Vec<(usize, Point, Character)>,
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
        let extra = if flags.contains(Flags::ScrollBars) {
            StatusFlags::IncreaseBottomMarginOnFocus
        } else {
            StatusFlags::None
        };
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
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
                labels: Vec::new(),
            },
            plot: PlotLayout::default(),
            surface: Surface::new(1, 1),
            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            left_scroll: 0,
            first_visible_point: 0,
            hovered: None,
            tooltip_text: String::new(),
            selected_point: None,
            junctions: Vec::new(),
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

    /// Returns the selected point as `(series_index, point_index)`, or `None` if nothing is selected.
    #[inline(always)]
    pub fn selected_point(&self) -> Option<(u32, u32)> {
        self.selected_point.map(|p| (p.series, p.index))
    }

    /// Selects point `point_index` of series `series_index` and scrolls it into view.
    ///
    /// No event is raised: this is meant for code that keeps another view (for example a
    /// property panel) in sync with the chart. If the point does not exist, the selection is cleared.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::with_values("CPU", [10, 14, 9]));
    /// chart.select_point(0, 2);
    /// assert_eq!(chart.selected_point(), Some((0, 2)));
    /// ```
    pub fn select_point(&mut self, series_index: u32, point_index: u32) {
        let point = PointPos::new(series_index, point_index);
        if self.point_exists(point) {
            self.selected_point = Some(point);
            self.ensure_visible(point_index as usize);
        } else {
            self.selected_point = None;
        }
    }

    /// Clears the selection. No event is raised.
    #[inline(always)]
    pub fn clear_selection(&mut self) {
        self.selected_point = None;
    }

    /// Scrolls the chart horizontally so the point at `index` (on the X axis) is visible.
    ///
    /// The chart scrolls the minimum amount needed to show the whole column of that point.
    /// A point that is already fully visible leaves the scroll position unchanged.
    /// Does nothing if `index` is not lower than [`points_count`](Self::points_count).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::ScrollBars);
    /// chart.add_series(linechart::Series::with_values("CPU", 1..=100));
    /// chart.ensure_visible(99);
    /// ```
    pub fn ensure_visible(&mut self, index: usize) {
        if index >= self.points_count() {
            return;
        }
        self.update_layout();
        // keep one free column on the right (same as the content width used by End / the scroll bar):
        // the scroll bar takes the last column while the chart has focus
        let plot_width = (self.size().width as i32 - self.x_axis_left_margin() - 1).max(0);
        if plot_width <= 0 {
            return;
        }
        let left = self.column_x(index);
        let right = left + self.plot.column_width as i32;
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
            self.after_horizontal_scroll();
        }
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
        // keep the selection on the same point (series after the removed one move down by one)
        if let Some(p) = self.selected_point {
            let removed = series_index as u32;
            self.selected_point = match p.series.cmp(&removed) {
                std::cmp::Ordering::Less => Some(p),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(PointPos::new(p.series - 1, p.index)),
            };
        }
        self.clear_hover();
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
    /// chart.add_value(0, BarBuilder::new(20).label("peak").build());
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

    // ------------------------------------------------------------------ X-axis labels

    /// Replaces the names of the X positions. Label `i` names the points at index `i` of every series.
    ///
    /// The labels are drawn under the points with [`XAxisLabelMode::Labels`] and shown in tooltips.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// chart.add_series(linechart::Series::with_values("CPU", [35, 42, 38]));
    /// chart.set_xaxis_labels(["Jan", "Feb", "Mar"]);
    /// chart.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);
    /// ```
    pub fn set_xaxis_labels<S>(&mut self, labels: impl IntoIterator<Item = S>)
    where
        S: AsRef<str>,
    {
        self.xaxis.labels.clear();
        self.xaxis.labels.extend(labels.into_iter().map(|l| l.as_ref().to_string()));
        self.repaint_surface();
    }
    /// Appends the name of the next X position.
    pub fn add_xaxis_label(&mut self, label: &str) {
        self.xaxis.labels.push(label.to_string());
        self.repaint_surface();
    }
    /// Appends the names of the next X positions.
    pub fn add_xaxis_labels<S>(&mut self, labels: impl IntoIterator<Item = S>)
    where
        S: AsRef<str>,
    {
        self.xaxis.labels.extend(labels.into_iter().map(|l| l.as_ref().to_string()));
        self.repaint_surface();
    }
    /// Returns the name of the X position `index`, or `None` if it has no (or an empty) label.
    pub fn xaxis_label(&self, index: usize) -> Option<&str> {
        self.xaxis.labels.get(index).map(|l| l.as_str()).filter(|l| !l.is_empty())
    }
    /// Returns the number of X-axis labels.
    #[inline(always)]
    pub fn xaxis_labels_count(&self) -> usize {
        self.xaxis.labels.len()
    }
    /// Removes the first `count` X-axis labels (or all of them if `count` is larger).
    pub fn remove_first_xaxis_labels(&mut self, count: usize) {
        let count = count.min(self.xaxis.labels.len());
        self.xaxis.labels.drain(..count);
        self.repaint_surface();
    }
    /// Removes all X-axis labels.
    pub fn clear_xaxis_labels(&mut self) {
        self.xaxis.labels.clear();
        self.repaint_surface();
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
    /// * [`XAxisLabelMode::Labels`] uses the labels set with [`set_xaxis_labels`](Self::set_xaxis_labels).
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
            XAxisLabelMode::Labels => self.xaxis.label_format = XAxisLabelFormat::Labels,
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
    fn level_to_value(&self, level: i32) -> f64 {
        if self.plot.levels <= 1 {
            return self.plot.low;
        }
        self.plot.low + (self.plot.high - self.plot.low) * level as f64 / (self.plot.levels - 1) as f64
    }
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
    #[inline(always)]
    fn plot_bottom(&self) -> i32 {
        self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1)
    }
    #[inline(always)]
    fn level0_row(&self) -> i32 {
        self.plot_bottom() - (self.plot.marker_height as i32 - 1) / 2
    }
    #[inline(always)]
    fn column_step(&self) -> i32 {
        self.plot.column_width as i32 + self.default_spacing as i32
    }
    #[inline(always)]
    fn column_x(&self, index: usize) -> i32 {
        self.default_spacing as i32 + index as i32 * self.column_step()
    }
    #[inline(always)]
    fn screen_x(&self, index: usize) -> i32 {
        self.column_x(index) - self.left_scroll + self.x_axis_left_margin()
    }

    // ------------------------------------------------------------------ scrolling

    #[inline(always)]
    fn content_width(&self) -> u64 {
        self.plot.content_width as u64 + self.x_axis_left_margin() as u64 + 1
    }
    #[inline(always)]
    fn max_left_scroll(&self) -> i32 {
        (self.content_width() as i32 - self.size().width as i32).max(0)
    }
    fn sync_horizontal_scrollbar(&mut self) {
        let sz = self.size();
        self.scrollbars.update(self.content_width(), sz.height as u64, sz);
        self.scrollbars.set_indexes(self.left_scroll as u64, 0);
    }
    fn update_first_visible_point(&mut self) {
        let step = self.column_step().max(1);
        // column j is visible when spacing + j * step + column_width > left_scroll
        let start = self.left_scroll - self.default_spacing as i32 - self.plot.column_width as i32;
        self.first_visible_point = if start < 0 { 0 } else { (start / step + 1) as usize };
    }
    fn after_horizontal_scroll(&mut self) {
        // the point under the mouse is no longer the same one
        self.clear_hover();
        self.update_first_visible_point();
        self.sync_horizontal_scrollbar();
        self.repaint_surface();
    }
    fn align_scroll_to_first_visible_point(&mut self) {
        let x = self.column_x(self.first_visible_point) - self.default_spacing as i32;
        self.left_scroll = x.clamp(0, self.max_left_scroll());
        self.update_first_visible_point();
        self.sync_horizontal_scrollbar();
        self.repaint_surface();
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.clear_hover();
        self.left_scroll = (self.scrollbars.horizontal_index() as i32).clamp(0, self.max_left_scroll());
        self.update_first_visible_point();
        self.repaint_surface();
    }
    fn scroll_by(&mut self, delta: i32) {
        let new_scroll = (self.left_scroll + delta).clamp(0, self.max_left_scroll());
        if new_scroll != self.left_scroll {
            self.left_scroll = new_scroll;
            self.after_horizontal_scroll();
        }
    }

    // ------------------------------------------------------------------ hovering

    pub(super) fn point_index_at(&self, x: i32, y: i32) -> Option<usize> {
        let left_margin = self.x_axis_left_margin();
        let points = self.points_count();
        if points == 0 || x < left_margin || x >= self.size().width as i32 || y < 0 || y > self.plot_bottom() {
            return None;
        }
        let step = self.column_step().max(1);
        let first_center = self.default_spacing as i32 + (self.plot.column_width as i32 - 1) / 2;
        let distance = x + self.left_scroll - left_margin - first_center;
        let index = (distance + step / 2).div_euclid(step);
        if index < 0 || index as usize >= points {
            return None;
        }
        Some(index as usize)
    }
    #[inline(always)]
    fn point_exists(&self, point: PointPos) -> bool {
        self.series.get(point.series as usize).is_some_and(|s| (point.index as usize) < s.len())
    }
    fn marker_rect(&self, point: PointPos) -> Rect {
        let Some(p) = self.series.get(point.series as usize).and_then(|s| s.data.get(point.index as usize)) else {
            return Rect::with_size(0, 0, 0, 0);
        };
        let x = self.screen_x(point.index as usize);
        let y = self.level0_row() - p.len as i32;
        match self.marker_for(&p.bar) {
            Marker::Framed(_) => Rect::with_size(x, y - 1, 5, 3),
            _ => Rect::with_size(x + (self.plot.column_width as i32 - 1) / 2, y, 1, 1),
        }
    }
    fn group_rect(&self, index: usize) -> Rect {
        let level0_row = self.level0_row();
        let mut top = i32::MAX;
        let mut bottom = i32::MIN;
        for series in self.series.iter() {
            if let Some(point) = series.data.get(index) {
                let y = level0_row - point.len as i32;
                let half = (self.marker_for(&point.bar).height() as i32 - 1) / 2;
                top = top.min(y - half);
                bottom = bottom.max(y + half);
            }
        }
        if top > bottom {
            top = level0_row;
            bottom = level0_row;
        }
        Rect::with_size(self.screen_x(index), top, self.plot.column_width as u16, (bottom - top + 1) as u16)
    }
    pub(super) fn point_at(&self, x: i32, y: i32) -> Option<PointPos> {
        let index = self.point_index_at(x, y)?;
        let mut best: Option<(i32, PointPos)> = None;
        for series_index in 0..self.series.len() {
            let point = PointPos::new(series_index as u32, index as u32);
            if !self.point_exists(point) {
                continue;
            }
            let r = self.marker_rect(point);
            let distance = if y < r.top() {
                r.top() - y
            } else if y > r.bottom() {
                y - r.bottom()
            } else {
                0
            };
            if distance <= 1 && best.is_none_or(|(d, _)| distance <= d) {
                best = Some((distance, point));
            }
        }
        best.map(|(_, point)| point)
    }
    fn hover_at(&self, x: i32, y: i32) -> Option<Hover> {
        if self.flags.contains(Flags::GroupHover) {
            self.point_index_at(x, y).map(Hover::Group)
        } else {
            self.point_at(x, y).map(Hover::Point)
        }
    }
    fn show_group_tooltip(&mut self, index: usize) {
        let mut text = std::mem::take(&mut self.tooltip_text);
        text.clear();
        let mut buffer = [0u8; 64];
        // header: the name of the X position
        if let Some(label) = self.xaxis_label(index) {
            text.push_str(label);
        }
        // one line per series: "series: value (note)"
        for series in self.series.iter() {
            let Some(point) = series.get(index) else {
                continue;
            };
            if !text.is_empty() {
                text.push('\n');
            }
            if !series.label().is_empty() {
                text.push_str(series.label());
                text.push_str(": ");
            }
            text.push_str(self.number_format.write_float(point.value.to_f64(), &mut buffer).unwrap_or(""));
            if !point.label().is_empty() {
                text.push_str(" (");
                text.push_str(point.label());
                text.push(')');
            }
        }
        self.tooltip_text = text;
        let r = self.group_rect(index);
        self.show_tooltip_on_rect(&self.tooltip_text, &r);
    }
    pub(super) fn show_point_tooltip(&mut self, point: PointPos) {
        let Some(series) = self.series.get(point.series as usize) else {
            return;
        };
        let Some(bar) = series.get(point.index as usize) else {
            return;
        };
        let mut text = std::mem::take(&mut self.tooltip_text);
        text.clear();
        let mut buffer = [0u8; 64];
        // series name, then "X label: value" (or just the value), then the note of the point
        if !series.label().is_empty() {
            text.push_str(series.label());
            text.push('\n');
        }
        if let Some(label) = self.xaxis_label(point.index as usize) {
            text.push_str(label);
            text.push_str(": ");
        }
        text.push_str(self.number_format.write_float(bar.value.to_f64(), &mut buffer).unwrap_or(""));
        if !bar.label().is_empty() {
            text.push('\n');
            text.push_str(bar.label());
        }
        self.tooltip_text = text;
        let r = self.marker_rect(point);
        self.show_tooltip_on_rect(&self.tooltip_text, &r);
    }
    fn show_hover_tooltip(&mut self, hover: Hover) {
        match hover {
            Hover::Group(index) => self.show_group_tooltip(index),
            Hover::Point(point) => self.show_point_tooltip(point),
        }
    }
    fn hover_is_valid(&self, hover: Hover) -> bool {
        match hover {
            Hover::Group(index) => index < self.points_count(),
            Hover::Point(point) => self.point_exists(point),
        }
    }
    fn clear_hover(&mut self) {
        if self.hovered.take().is_some() {
            self.hide_tooltip();
        }
    }

    // ------------------------------------------------------------------ selection

    fn select_point_by_user(&mut self, point: Option<PointPos>) {
        if point == self.selected_point {
            return;
        }
        self.selected_point = point;
        let event_type = match point {
            Some(p) => {
                self.ensure_visible(p.index as usize);
                EventType::PointSelected {
                    series_index: p.series,
                    point_index: p.index,
                }
            }
            None => EventType::ClearSelection,
        };
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::LineChart(EventData {
                event_type,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn vertical_neighbour(&self, current: PointPos, up: bool) -> Option<PointPos> {
        let key = |p: PointPos| (self.marker_rect(p).top(), p.series);
        let current_key = key(current);
        (0..self.series.len() as u32)
            .map(|s| PointPos::new(s, current.index))
            .filter(|p| *p != current && self.point_exists(*p))
            .filter(|p| if up { key(*p) < current_key } else { key(*p) > current_key })
            .reduce(|a, b| {
                let better = if up { key(b) > key(a) } else { key(b) < key(a) };
                if better {
                    b
                } else {
                    a
                }
            })
    }
    fn selection_clip(&self) -> Rect {
        let left = if self.yaxis.visible && self.yaxis.width > 0 {
            self.yaxis.width as i32 + 1
        } else {
            0
        };
        let bottom = self.size().height as i32 - if self.xaxis.label_format.is_none() { 1 } else { 2 };
        let right = self.size().width.saturating_sub(1) as i32;
        Rect::new(left, 0, right, bottom)
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
        if let Some(point) = self.selected_point {
            if !self.point_exists(point) {
                self.selected_point = None;
            }
        }
        // the content may have shrunk (fewer points, smaller spacing or markers) or the control may have grown
        self.left_scroll = self.left_scroll.clamp(0, self.max_left_scroll());
        self.update_first_visible_point();
        self.sync_horizontal_scrollbar();
    }

    // ------------------------------------------------------------------ painting

    fn repaint_surface(&mut self) {
        self.paint_surface();
        // the data under the mouse may have changed (or disappeared)
        if let Some(hover) = self.hovered {
            if self.hover_is_valid(hover) {
                self.show_hover_tooltip(hover);
            } else {
                self.clear_hover();
            }
        }
    }
    fn paint_surface(&mut self) {
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
        // lines first, markers on top (a line always runs from one marker center to the next)
        self.paint_lines(&series_colors);
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
            XAxisLabelFormat::Labels => self.paint_xaxis_labels(label_attr),
            XAxisLabelFormat::Custom => self.paint_xaxis_custom(label_attr),
        }
    }
    fn print_label(&mut self, start: usize, end: usize, label: &str, attr: CharAttribute) {
        let x = self.screen_x(start);
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
        for index in self.first_visible_point..self.points_count() {
            if self.screen_x(index) >= width {
                break;
            }
            if let Some(result) = INT_FORMAT.write_number(start as i64 + index as i64, &mut temp) {
                self.print_label(index, index, result, attr);
            }
        }
    }
    fn paint_xaxis_labels(&mut self, attr: CharAttribute) {
        let mut temp_str: FlatString<254> = FlatString::new();
        let width = self.size().width as i32;
        let end = self.points_count().min(self.xaxis.labels.len());
        for index in self.first_visible_point..end {
            if self.screen_x(index) >= width {
                break;
            }
            if !self.xaxis.labels[index].is_empty() {
                temp_str.set(self.xaxis.labels[index].as_str());
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
            if start_index >= len || self.screen_x(start_index) >= width {
                break;
            }
            let end_index = (span.end as usize).min(len - 1);
            if self.screen_x(end_index) + self.plot.column_width as i32 <= left_margin {
                continue; // scrolled out on the left
            }
            self.print_label(start_index, end_index, span.label.as_str(), attr);
        }
    }
    #[inline(always)]
    fn series_attr(series: &Series<T>, series_index: usize, series_colors: &[CharAttribute]) -> CharAttribute {
        series.attr().unwrap_or(series_colors[series_index % series_colors.len()])
    }
    #[inline(always)]
    fn line_anchor(&self, index: usize, point: &BarWithLayout<T>, outgoing: bool, level0_row: i32) -> Point {
        let y = level0_row - point.len as i32;
        let x = self.screen_x(index);
        match self.marker_for(&point.bar) {
            Marker::Framed(_) => Point::new(if outgoing { x + 5 } else { x - 1 }, y),
            _ => Point::new(x + (self.plot.column_width as i32 - 1) / 2, y),
        }
    }
    fn junction_for(&self, point: &BarWithLayout<T>, neighbour: Point, left_side: bool) -> Option<char> {
        let Marker::Framed(frame) = self.marker_for(&point.bar) else {
            return None;
        };
        let junctions = super::marker::frame_junctions(frame)?;
        let ch = self.surface.char(neighbour.x, neighbour.y)?.code;
        // a line on the left of the marker must have an arm towards the right (and vice versa)
        let weight = super::marker::horizontal_arm(ch, !left_side)?;
        let (left, right) = junctions[weight];
        Some(if left_side { left } else { right })
    }
    fn paint_lines(&mut self, series_colors: &[CharAttribute]) {
        if self.plot.levels == 0 {
            return;
        }
        let left_margin = self.x_axis_left_margin();
        let width = self.size().width as i32;
        let level0_row = self.level0_row();
        self.junctions.clear();
        self.surface.set_relative_clip(left_margin, 0, width, self.plot_bottom());
        for series_index in 0..self.series.len() {
            let series = &self.series[series_index];
            let line = series.line().unwrap_or(self.default_line);
            if line == LineMode::None || series.len() < 2 {
                continue;
            }
            let attr = Self::series_attr(series, series_index, series_colors);
            // start one point earlier: the segment that ends at the first visible point may still show
            let first = self.first_visible_point.saturating_sub(1);
            if first + 1 >= series.data.len() {
                continue;
            }
            for index in first + 1..self.series[series_index].data.len() {
                let series = &self.series[series_index];
                let start = self.line_anchor(index - 1, &series.data[index - 1], true, level0_row);
                if start.x >= width {
                    // the rest of the series is outside the visible area
                    break;
                }
                let end = self.line_anchor(index, &series.data[index], false, level0_row);
                if start.x > end.x {
                    // the markers touch each other: there is no room for a line
                    continue;
                }
                // a line attached to the side of a framed marker must leave / enter it horizontally
                let framed_end = matches!(self.marker_for(&series.data[index - 1].bar), Marker::Framed(_))
                    || matches!(self.marker_for(&series.data[index].bar), Marker::Framed(_));
                match line {
                    LineMode::None => {}
                    LineMode::Braille => self.surface.draw_line(start.x, start.y, end.x, end.y, LineType::Braille, attr),
                    LineMode::Line(line_type) | LineMode::Orthogonal(line_type, _) if start == end => {
                        // a single cell between two framed markers
                        self.surface.draw_horizontal_line(start.x, start.y, start.x, line_type, attr)
                    }
                    LineMode::Line(line_type) if framed_end && start.y != end.y => {
                        // a box-drawing staircase can start or end with a vertical step, which would run along
                        // the side of the frame instead of entering it: route it horizontally at both ends
                        self.surface.draw_orthogonal_line(
                            start.x,
                            start.y,
                            end.x,
                            end.y,
                            line_type,
                            OrthogonalDirection::HorizontalUntilMiddle,
                            attr,
                        )
                    }
                    LineMode::Line(line_type) => self.surface.draw_line(start.x, start.y, end.x, end.y, line_type, attr),
                    LineMode::Orthogonal(line_type, direction) => {
                        let direction = if framed_end && direction == OrthogonalDirection::Auto {
                            OrthogonalDirection::HorizontalUntilMiddle
                        } else {
                            direction
                        };
                        self.surface.draw_orthogonal_line(start.x, start.y, end.x, end.y, line_type, direction, attr)
                    }
                }
                // junctions on the sides of framed markers (applied after the markers are drawn)
                let series = &self.series[series_index];
                if let Some(ch) = self.junction_for(&series.data[index - 1], start, false) {
                    let point = &series.data[index - 1].bar;
                    let a = point.attr.unwrap_or(attr);
                    self.junctions.push((series_index, Point::new(start.x - 1, start.y), Character::with_attributes(ch, a)));
                }
                if let Some(ch) = self.junction_for(&series.data[index], end, true) {
                    let point = &series.data[index].bar;
                    let a = point.attr.unwrap_or(attr);
                    self.junctions.push((series_index, Point::new(end.x + 1, end.y), Character::with_attributes(ch, a)));
                }
            }
        }
        self.surface.reset_clip();
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
            let series_attr = Self::series_attr(series, series_index, series_colors);
            for (index, point) in series.data.iter().enumerate().skip(self.first_visible_point) {
                let x = self.screen_x(index);
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
                    Marker::Framed(point_type) => {
                        // the middle row is on the value
                        for (dy, row) in point_type.cells().iter().enumerate() {
                            for (dx, ch) in row.iter().enumerate() {
                                self.surface.write_char(x + dx as i32, y - 1 + dy as i32, Character::with_attributes(*ch, attr));
                            }
                        }
                    }
                }
            }
            // the junctions of this series (a later series is drawn on top of them)
            for (_, pos, ch) in self.junctions.iter().filter(|(s, _, _)| *s == series_index) {
                self.surface.write_char(pos.x, pos.y, *ch);
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
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
            return;
        }
        surface.draw_surface(0, 0, &self.surface);
        if self.plot.levels == 0 {
            return;
        }
        let clip = self.selection_clip();
        if let Some(Hover::Group(index)) = self.hovered {
            if index < self.points_count() {
                // highlight the whole column (plot area only) by changing its background
                surface.set_relative_clip(clip.left(), clip.top(), clip.right(), self.plot_bottom());
                let column = Rect::with_size(self.screen_x(index), 0, self.plot.column_width as u16, (self.plot_bottom() + 1) as u16);
                let back = theme.chart.hover.background;
                surface.transform_rect(column, |ch, _| Some(Character::new(ch.code, ch.foreground, back, ch.flags)));
            }
        }
        if let Some(point) = self.selected_point {
            if self.point_exists(point) {
                surface.set_relative_clip(clip.left(), clip.top(), clip.right(), clip.bottom());
                let mut r = self.marker_rect(point);
                r.inflate_width(1, 1, 1, 1);
                if self.flags.contains(Flags::DimPointsOnSelection) {
                    let attr = theme.chart.inactive;
                    surface.transform_rect(clip, |ch, p| {
                        if r.contains(p) {
                            None
                        } else {
                            Some(Character::with_attributes(ch.code, attr))
                        }
                    });
                }
                surface.draw_rect(r, LineType::Single, theme.chart.selection_border);
            }
        }
    }
}

impl<T> OnResize for LineChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_layout();
        self.scrollbars.resize(self.content_width(), new_size.height as u64, &self.base);
        self.update_scroll_pos_from_scrollbars();
    }
}

impl<T> OnMouseEvent for LineChart<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.clear_hover();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(pos) => {
                let hover = self.hover_at(pos.x, pos.y);
                if hover == self.hovered {
                    return EventProcessStatus::Ignored;
                }
                self.hovered = hover;
                match hover {
                    Some(hover) => self.show_hover_tooltip(hover),
                    None => self.hide_tooltip(),
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) | MouseEvent::DoubleClick(data) => {
                let point = self.point_at(data.x, data.y);
                self.select_point_by_user(point);
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(wheel) => {
                match wheel {
                    MouseWheelDirection::Left | MouseWheelDirection::Up => self.scroll_by(-1),
                    MouseWheelDirection::Right | MouseWheelDirection::Down => self.scroll_by(1),
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnKeyPressed for LineChart<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                match self.selected_point {
                    Some(p) => {
                        if p.index > 0 {
                            self.select_point_by_user(Some(PointPos::new(p.series, p.index - 1)));
                        }
                    }
                    None => self.scroll_by(-1),
                }
                EventProcessStatus::Processed
            }
            key!("Right") => {
                match self.selected_point {
                    Some(p) => {
                        let next = PointPos::new(p.series, p.index + 1);
                        if self.point_exists(next) {
                            self.select_point_by_user(Some(next));
                        }
                    }
                    None => self.scroll_by(1),
                }
                EventProcessStatus::Processed
            }
            key!("Up") | key!("Down") => {
                let Some(current) = self.selected_point else {
                    return EventProcessStatus::Ignored;
                };
                if let Some(next) = self.vertical_neighbour(current, key.value() == key!("Up")) {
                    self.select_point_by_user(Some(next));
                }
                EventProcessStatus::Processed
            }
            key!("Escape") => {
                if self.selected_point.is_none() {
                    return EventProcessStatus::Ignored;
                }
                self.select_point_by_user(None);
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.scroll_by(-self.left_scroll);
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.scroll_by(self.max_left_scroll() - self.left_scroll);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Left") => {
                // go to the previous point; if the current one is cut on the left, align it first
                let aligned = self.column_x(self.first_visible_point) - self.default_spacing as i32 == self.left_scroll;
                if aligned && self.first_visible_point > 0 {
                    self.first_visible_point -= 1;
                }
                self.align_scroll_to_first_visible_point();
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
                if self.first_visible_point + 1 < self.points_count() && self.left_scroll < self.max_left_scroll() {
                    self.first_visible_point += 1;
                    self.align_scroll_to_first_visible_point();
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

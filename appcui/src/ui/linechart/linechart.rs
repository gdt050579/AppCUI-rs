use super::{Flags, LineMode, Marker, Series};
use crate::prelude::*;

#[CustomControl(internal = true)]
/// A line chart for one or more numeric series of type `T`.
///
/// Each [`Series`] is a list of points that share the same X axis, so several lines
/// can be drawn over the same range. The X range is as long as the longest series;
/// shorter series simply end earlier. Optional behavior is controlled by [`Flags`].
///
/// Data is always added to a series by its index, so at least one series must be
/// added with [`add_series`](Self::add_series) before any value.
pub struct LineChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    series: Vec<Series<T>>,
    default_line: LineMode,
    default_marker: Marker,
    default_spacing: u8,
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
    /// [`LineMode::default`], and separated by 2 empty cells until a default is changed.
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

    // ------------------------------------------------------------------ internal

    /// Recomputes the layout and repaints the internal surface. Implemented with painting (step 3).
    fn repaint_surface(&mut self) {}
}

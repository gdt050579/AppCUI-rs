use super::{Bar, BarDrawMode, BarScale, BarSpan, Flags, YAxisLabelFormat, YAxisLabelMode};
use crate::prelude::*;
use crate::ui::components::{BarDefaults, ScrollBars};

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
    inner: &'a mut Vec<Bar<T>>,
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
        self.inner.get(index)
    }
    /// Returns a mutable reference to the bar at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar<T>> {
        self.inner.get_mut(index)
    }
    /// Appends a bar at the end of the series.
    #[inline(always)]
    pub fn add<B>(&mut self, bar: B)
    where
        B: Into<Bar<T>>,
    {
        self.inner.push(bar.into());
    }
    /// Appends several bars at the end of the series.
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.inner.extend(bars.into_iter().map(Into::into));
    }
    /// Inserts a bar at `index`. Returns `false` if `index` is greater than [`len`](Self::len).
    pub fn insert<B>(&mut self, index: usize, bar: B) -> bool
    where
        B: Into<Bar<T>>,
    {
        if index > self.inner.len() {
            return false;
        }
        self.inner.insert(index, bar.into());
        true
    }
    /// Removes the bar at `index` and returns it, or `None` if out of range.
    pub fn delete(&mut self, index: usize) -> Option<Bar<T>> {
        if index >= self.inner.len() {
            return None;
        }
        Some(self.inner.remove(index))
    }
    /// Replaces the bar at `index` and returns the previous bar, or `None` if out of range.
    pub fn set<B>(&mut self, index: usize, bar: B) -> Option<Bar<T>>
    where
        B: Into<Bar<T>>,
    {
        let slot = self.inner.get_mut(index)?;
        Some(std::mem::replace(slot, bar.into()))
    }
    /// Removes all bars from the series.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Iterates over the bars in the series.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Bar<T>> {
        self.inner.iter()
    }
    /// Iterates mutably over the bars in the series.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bar<T>> {
        self.inner.iter_mut()
    }
}

struct XAxis {
    width: u8,
    step: u8,
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
    bars: Vec<Bar<T>>,
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
                visible: true,
                show_grid: true,
            },
            defaults: BarDefaults {
                attr: CharAttribute::default(),
                thickness: 1,
                spacing: 1,
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
        self.bars.push(bar.into());
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
        self.bars.extend(bars.into_iter().map(Into::into));
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
        self.bars.get(index)
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
        if let Some(bar) = self.bars.get_mut(index) {
            f(bar);
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
    fn repaint_surface(&mut self) {
        let background = self.theme().chart.background;
        self.surface.clear(Character::with_attributes(' ', background));
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

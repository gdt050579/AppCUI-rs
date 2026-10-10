use flat_string::FlatString;

use super::{
    events::{EventData, EventType},
    Bar, BarDefaults, BarDrawMode, BarScale, BarSpan, Flags, XAxisLabelFormat, Bars,
};
use crate::{prelude::*, ui::vbarchart::XAxisLabelMode};
struct YAxis {
    width: u8,
    step: u8,
    zero: i32,
    bottom_value: f64,
    bottom_step: f64,
    visible: bool,
    show_grid: bool,
}
struct XAxis {
    label_format: XAxisLabelFormat,
    spans: Vec<BarSpan>,
}

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
/// A vertical bar chart for a numeric series of type `T`.
///
/// `VBarChart` displays one vertical bar per value. Scale, axis labels, the default
/// bar appearance, and number formatting are set through the methods on this type.
/// Optional behavior, such as scroll bars, a zero line, and dimming unselected bars,
/// is controlled by [`Flags`].
///
/// Clicking a bar selects it and raises a bar-selected event. Clicking outside any
/// bar clears the selection. Hovering a bar shows its value, and its label when one
/// is set, in a tooltip. When the chart has focus, the arrow keys scroll it horizontally.
pub struct VBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    bars: Vec<BarWithLayout<T>>,
    bars_width: u32,
    pub(super) left_scroll: i32,
    pub(super) first_visible_bar: u32,
    yaxis: YAxis,
    scale: BarScale<T>,
    number_format: FormatNumber,
    defaults: BarDefaults,
    surface: Surface,
    use_theme_colors_for_bars: bool,
    xaxis: XAxis,
    scrollbars: ScrollBars,
    hovered_bar: Option<u32>,
    selected_bar: Option<u32>,
    tooltip_text: String,
}

impl<T> VBarChart<T>
where
    T: Number + 'static,
{
    /// Creates an empty vertical bar chart.
    ///
    /// `layout` places the control. `flags` selects optional behavior; see [`Flags`].
    ///
    /// A new chart scales bars with [`BarScale::FromZero`], shows the Y axis and its
    /// grid, and formats labels with two decimals when `T` is a floating-point type
    /// or with thousands separators when `T` is an integer. Bars use the theme color,
    /// a thickness of 1, and a spacing of 1 until a default is changed.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = VBarChart::<i32>::new(
    ///     layout!("d:f"),
    ///     vbarchart::Flags::ScrollBars | vbarchart::Flags::ShowZeroLineOnYAxis,
    /// );
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
            bars: Vec::new(),
            bars_width: 0,
            left_scroll: 0,
            first_visible_bar: 0,
            yaxis: YAxis {
                width: 6,
                step: 3,
                zero: 0,
                bottom_value: 0.0,
                bottom_step: 0.0,
                visible: true,
                show_grid: true,
            },
            defaults: BarDefaults {
                attr: CharAttribute::default(),
                thickness: 1,
                spacing: 1,
                vertical: true,
                draw_mode: BarDrawMode::default(),
            },
            xaxis: XAxis {
                label_format: XAxisLabelFormat::None,
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
            hovered_bar: None,
            selected_bar: None,
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
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.add_bar(10);
    /// chart.add_bar(BarBuilder::new(20).label("Feb").build());
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
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
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
    /// Scrolls the chart horizontally so the bar at `index` is visible.
    ///
    /// When the bar fits in the plot area, the chart scrolls the minimum amount needed to show
    /// it entirely. A bar that is already fully visible leaves the scroll position unchanged.
    /// A bar wider than the plot area is scrolled until part of it is on screen.
    /// Does nothing if `index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.add_bars(1..=50);
    /// chart.ensure_visible(49);
    /// ```
    pub fn ensure_visible(&mut self, index: usize) {
        if index >= self.bars.len() {
            return;
        }
        self.update_bars_layout();
        let plot_width = (self.size().width as i32 - self.x_axis_left_margin()).max(0);
        if plot_width <= 0 {
            return;
        }
        let bar = &self.bars[index];
        let bar_left = bar.pos;
        let bar_right = bar_left + bar.bar.actual_thickness(&self.defaults) as i32;
        let view_left = self.left_scroll;
        let view_right = view_left + plot_width;
        let new_scroll = if bar_right - bar_left <= plot_width {
            if bar_left >= view_left && bar_right <= view_right {
                return;
            }
            if bar_left < view_left {
                bar_left
            } else {
                bar_right - plot_width
            }
        } else if bar_right <= view_left {
            bar_right - plot_width
        } else if bar_left >= view_right {
            bar_left
        } else {
            return;
        };
        let new_scroll = new_scroll.clamp(0, self.max_left_scroll());
        if new_scroll != self.left_scroll {
            self.left_scroll = new_scroll;
            self.after_horizontal_scroll();
        }
    }
    /// Returns an immutable reference to the bar at `index`, or `None` if out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.add_bar(3);
    /// if let Some(bar) = chart.get_bar(0) {
    ///     assert_eq!(bar.value(), 3);
    /// }
    /// ```
    #[inline(always)]
    pub fn get_bar(&self, index: usize) -> Option<&Bar<T>> {
        self.bars.get(index).map(|b| &b.bar)
    }
    /// Mutates the bar at `index`, then relayouts and repaints the chart.
    ///
    /// Does nothing if `index` is out of range.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
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
            f(&mut bar.bar);
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
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
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
    /// Sets how bar values are mapped onto the plot height.
    ///
    /// * [`BarScale::FromZero`] draws every bar from zero. The visible range includes
    ///   zero and every bar value.
    /// * [`BarScale::FromZeroMinRange`] does the same, and also expands the range so
    ///   that it covers `min` and `max`.
    /// * [`BarScale::FitData`] stretches the smallest and largest values across the
    ///   full plot height.
    /// * [`BarScale::Fixed`] uses the given `min` and `max`. Values outside that
    ///   range are drawn at the corresponding edge of the plot.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.set_bars_scale(vbarchart::BarScale::Fixed { min: 0, max: 100 });
    /// ```
    pub fn set_bars_scale(&mut self, scale: BarScale<T>) {
        self.scale = scale;
        self.repaint_surface();
    }
    /// Sets the format used for Y-axis labels, the zero-line label, and hover tooltips.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<f64>::new(layout!("d:f"), vbarchart::Flags::None);
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
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.set_default_bar_drawmode(BarDrawMode::Fill(BarFillType::Shade50));
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
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.set_default_bar_attr(CharAttribute::with_fore_color(Color::Yellow));
    /// ```
    pub fn set_default_bar_attr(&mut self, attr: CharAttribute) {
        self.defaults.attr = attr;
        self.use_theme_colors_for_bars = false;
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
    /// Sets how many characters are reserved for Y-axis labels.
    ///
    /// Values below 1 are treated as 1. This width is used only while the Y axis
    /// is visible.
    pub fn set_yaxis_width(&mut self, width: u8) {
        self.yaxis.width = width.max(1);
        self.repaint_surface();
    }
    /// Sets the distance, in rows, between horizontal grid lines and their labels.
    ///
    /// Values below 1 are treated as 1.
    pub fn set_yaxis_step(&mut self, step: u8) {
        self.yaxis.step = step.max(1);
        self.repaint_surface();
    }
    /// Sets how labels are drawn under the bars.
    ///
    /// * [`XAxisLabelMode::None`] draws no X axis.
    /// * [`XAxisLabelMode::Index`] labels each bar with `start + bar index`.
    /// * [`XAxisLabelMode::BarLabels`] uses each bar's own label and skips empty ones.
    /// * [`XAxisLabelMode::Custom`] copies the given spans. Spans are ordered by
    ///   start index, then by end index. A span that overlaps an earlier one is dropped.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = VBarChart::<i32>::new(layout!("d:f"), vbarchart::Flags::None);
    /// chart.add_bars(1..=6);
    /// let spans = [BarSpan::new(0, 3, "Q1"), BarSpan::new(3, 3, "Q2")];
    /// chart.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Custom(&spans));
    /// ```
    pub fn set_xaxis_label_mode(&mut self, xaxis: XAxisLabelMode) {
        match xaxis {
            XAxisLabelMode::None => self.xaxis.label_format = XAxisLabelFormat::None,
            XAxisLabelMode::Index (start) => self.xaxis.label_format = XAxisLabelFormat::Index (start),
            XAxisLabelMode::BarLabels => self.xaxis.label_format = XAxisLabelFormat::BarLabels,
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
    fn visible_height(&self) -> u32 {
        // one space from the top and the size of the height
        self.size().height.saturating_sub(self.xaxis.label_format.height() as u32 + 1)
    }
    #[inline(always)]
    fn x_axis_left_margin(&self) -> i32 {
        if self.yaxis.visible && self.yaxis.width > 0 {
            self.yaxis.width as i32 + 2
        } else {
            0
        }
    }
    fn update_yaxis_scale(&mut self, bottom_value: f64, top_value: f64) {
        let height = self.visible_height() as f64;
        self.yaxis.bottom_value = bottom_value;
        self.yaxis.bottom_step = if height > 0.0 {
            (bottom_value - top_value) / height * (self.yaxis.step as f64)
        } else {
            0.0
        };
    }
    fn update_bars_height_from_zero(&mut self, min: f64, max: f64) {
        let lo = min.min(0.0);
        let hi = max.max(0.0);
        let total = hi - lo;
        self.update_yaxis_scale(lo, hi);
        if total > 0.0 {
            let height = self.visible_height() as f64;
            let cells_below = (-lo / total * height).round();
            let cells_above = height - cells_below;
            self.yaxis.zero = cells_below as i32;
            for bar in self.bars.iter_mut() {
                let v = bar.bar.value.to_f64();
                let h = if v >= 0.0 {
                    if hi > 0.0 {
                        v / hi * cells_above
                    } else {
                        0.0
                    }
                } else {
                    -(v / lo * cells_below)
                };
                bar.len = h.round() as i16;
            }
        }
    }
    fn update_bars_height_fit_data(&mut self, min: f64, max: f64) {
        let height = self.visible_height() as f64;
        let range = max - min;
        self.yaxis.zero = 0;
        self.update_yaxis_scale(min, max);

        if range > 0.0 {
            for bar in self.bars.iter_mut() {
                let v = bar.bar.value.to_f64();
                let h = (v - min) / range * height;
                bar.len = h.trunc() as i16;
            }
        } else {
            let uniform = (height * 0.5).trunc() as i16;
            for bar in self.bars.iter_mut() {
                bar.len = uniform;
            }
        }
    }
    fn update_bars_height_fixed(&mut self, min: f64, max: f64) {
        let height = self.visible_height() as f64;
        let total = max - min;
        self.update_yaxis_scale(min, max);

        if total <= 0.0 {
            self.yaxis.zero = 0;
            let uniform = (height * 0.5).trunc() as i16;
            for bar in self.bars.iter_mut() {
                bar.len = uniform;
            }
            return;
        }

        let zero_cell = (0.0 - min) / total * height;
        let zero_vis = zero_cell.clamp(0.0, height); // visible baseline row
        self.yaxis.zero = zero_vis.round() as i32;

        for bar in self.bars.iter_mut() {
            let v = bar.bar.value.to_f64();
            let tip = ((v - min) / total * height).clamp(0.0, height);
            let h = tip - zero_vis;
            bar.len = h.trunc() as i16;
        }
    }
    fn update_bars_layout(&mut self) {
        if self.bars.is_empty() {
            self.bars_width = 0;
            return;
        }
        let mut x = 0;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;

        for bar in self.bars.iter_mut() {
            x += bar.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
            bar.pos = x;
            bar.len = 0;
            x += bar.bar.actual_thickness(&self.defaults) as i32;
            let value = bar.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        self.bars_width = x as u32 + self.bars[0].bar.spacing.unwrap_or(self.defaults.spacing) as u32;
        match self.scale {
            BarScale::FromZero => self.update_bars_height_from_zero(v_min, v_max),
            BarScale::FromZeroMinRange { min, max } => self.update_bars_height_from_zero(min.to_f64().min(v_min), max.to_f64().max(v_max)),
            BarScale::FitData => self.update_bars_height_fit_data(v_min, v_max),
            BarScale::Fixed { min, max } => self.update_bars_height_fixed(min.to_f64(), max.to_f64()),
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
        // mereu in ordinea asta - Y, X (X va suprascrie o parte de la Y)
        self.paint_yaxis(axis_attr, grid_attr, label_attr);
        self.paint_xaxis(axis_attr, label_attr);
        let len = self.bars.len();
        let mut start = self.first_visible_bar as usize;
        let width = self.size().width as i32;
        let plot_bottom = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        let left_margin = self.x_axis_left_margin();
        let mut layout = BarLayout { y: plot_bottom - self.yaxis.zero, ..Default::default() };
        let mut defaults = self.defaults;
        if self.use_theme_colors_for_bars {
            defaults.attr = bar_attr;
        }
        self.surface.set_relative_clip(left_margin, 0, width, plot_bottom);
        layout.surface_size = Size::new((width + 1 - left_margin) as u32, (plot_bottom + 1) as u32);
        while start < len {
            layout.x = self.bars[start].pos - self.left_scroll + left_margin;
            if layout.x >= width {
                break;
            }
            let bar = &self.bars[start];
            layout.length = bar.len;
            bar.bar.paint_vertical(&mut self.surface, &layout, &defaults);
            start += 1;
        }
        if (left_margin > 0) && (self.xaxis.label_format != XAxisLabelFormat::None) {
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
            let format = &self.number_format;
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
                // value(y) = bottom_value - bottom_step * (bottom - y) / step
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
            XAxisLabelFormat::Index (start) => self.paint_xaxis_index(start, label_attr),
            XAxisLabelFormat::BarLabels => self.paint_xaxis_bar_labels(label_attr),
            XAxisLabelFormat::Custom => self.paint_xaxis_custom(label_attr),
        }
    }
    fn print_label(&mut self, x: i32, y: i32, start_bar_index: usize, end_bar_index: usize, label: &str, attr: CharAttribute) {
        let last_bar = &self.bars[end_bar_index];
        let first_bar = &self.bars[start_bar_index];
        let width = (last_bar.bar.actual_thickness(&self.defaults) as i32) + last_bar.pos - first_bar.pos;
        let left_space = first_bar.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
        let right_space = if end_bar_index + 1 < self.bars.len() {
            self.bars[end_bar_index + 1].bar.spacing.unwrap_or(self.defaults.spacing) as i32
        } else {
            left_space
        };
        // pentru impare - 5 -> 5/2 - (1-5 & 1) = 2 - 0 = 2;
        // pentru pare - 6 => 6/2 - (1-6 & 1) = 3 - 1 = 2;
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
        let mut idx_start = self.first_visible_bar as usize;
        let mut bar_idx = (start as i64) + self.first_visible_bar as i64;
        let len = self.bars.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let bar = &self.bars[idx_start];
            let x = bar.pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if let Some(result) = INT_FORMAT.write_number(bar_idx, &mut temp) {
                self.print_label(x, y, idx_start, idx_start, result, attr);
            }
            idx_start += 1;
            bar_idx += 1;
        }
    }
    fn paint_xaxis_bar_labels(&mut self, attr: CharAttribute) {
        let mut temp_str: FlatString<254> = FlatString::new();
        let mut idx_start = self.first_visible_bar as usize;
        let len = self.bars.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let bar = &self.bars[idx_start];
            let x = bar.pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if !bar.bar.label.is_empty() {
                temp_str.set(bar.bar.label.as_str());
                self.print_label(x, y, idx_start, idx_start, temp_str.as_str(), attr);
            }
            idx_start += 1;
        }
    }
    fn paint_xaxis_custom(&mut self, attr: CharAttribute) {
        let len = self.bars.len();
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
            let bar = &self.bars[start_index];
            let x = bar.pos - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            let end_index = (span.end as usize).min(len - 1);
            let end_x = self.bars[end_index].pos - self.left_scroll + self.bars[end_index].bar.actual_thickness(&self.defaults) as i32;
            if end_x < 0 {
                continue; // not visible
            }
            let span_copy = *span; // doar 32 de octeti - e rapid
            self.print_label(x, y, start_index, end_index, span_copy.label.as_str(), attr);
        }
    }
    #[inline(always)]
    fn max_left_scroll(&self) -> i32 {
        let content_width = self.bars_width as i32 + self.x_axis_left_margin() + 1;
        (content_width - self.size().width as i32).max(0)
    }
    fn sync_horizontal_scrollbar(&mut self) {
        let sz = self.size();
        self.scrollbars
            .update(self.bars_width as u64 + self.x_axis_left_margin() as u64 + 1, sz.height as u64, sz);
        self.scrollbars.set_indexes(self.left_scroll as u64, 0);
    }
    #[inline(always)]
    fn update_first_visible_bar(&mut self) {
        self.first_visible_bar = (self.bars.partition_point(|b| b.pos < self.left_scroll) as u32).saturating_sub(1);
    }
    fn after_horizontal_scroll(&mut self) {
        self.update_first_visible_bar();
        self.sync_horizontal_scrollbar();
        self.repaint_surface();
    }
    fn align_scroll_to_first_visible_bar(&mut self) {
        if let Some(bar) = self.bars.get(self.first_visible_bar as usize) {
            self.left_scroll = bar.pos.max(0);
        }
        self.sync_horizontal_scrollbar();
        self.repaint_surface();
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.left_scroll = self.scrollbars.horizontal_index() as i32;
        // binary search - cea mai apropiata bara
        self.update_first_visible_bar();
        self.repaint_surface();
    }
    fn bar_index_at(&self, x: i32, y: i32) -> Option<u32> {
        let left_margin = self.x_axis_left_margin();
        let plot_bottom = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        if x < left_margin || y < 0 || y > plot_bottom {
            return None;
        }
        let content_x = x + self.left_scroll - left_margin;
        let idx = self.bars.partition_point(|b| b.pos <= content_x).saturating_sub(1);
        let bar = self.bars.get(idx)?;
        let thickness = bar.bar.actual_thickness(&self.defaults) as i32;
        if content_x < bar.pos || content_x >= bar.pos + thickness {
            return None;
        }
        let baseline = plot_bottom - self.yaxis.zero;
        let h = bar.len;
        let (top, bottom) = if h == 0 {
            (baseline, baseline)
        } else if h > 0 {
            (baseline + 1 - h as i32, baseline)
        } else {
            (baseline + 1, baseline + h.unsigned_abs() as i32)
        };
        if y < top || y > bottom {
            return None;
        }
        Some(idx as u32)
    }
    fn bar_rect(&self, index: usize) -> Rect {
        let bar = &self.bars[index];
        let left_margin = self.x_axis_left_margin();
        let plot_bottom = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        let x = bar.pos - self.left_scroll + left_margin;
        let thickness = bar.bar.actual_thickness(&self.defaults) as u16;
        let baseline = plot_bottom - self.yaxis.zero;
        let h = bar.len;
        if h == 0 {
            Rect::with_size(x, baseline, thickness, 1)
        } else if h > 0 {
            Rect::with_size(x, baseline + 1 - h as i32, thickness, h as u16)
        } else {
            let abs_h = h.unsigned_abs();
            Rect::with_size(x, baseline + 1, thickness, abs_h)
        }
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
    fn clamp_selected_bar(&mut self) {
        if let Some(index) = self.selected_bar {
            if (index as usize) >= self.bars.len() {
                self.selected_bar = None;
            }
        }
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
            data: ControlEventData::VBarChart(EventData {
                event_type: EventType::BarSelected(index),
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn raise_clear_selection_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::VBarChart(EventData {
                event_type: EventType::ClearSelection,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags.contains(Flags::ScrollBars)) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        } else {
            surface.draw_surface(0, 0, &self.surface);
            if let Some(index) = self.selected_bar {
                // clip to the plot area bounded by the X and Y axes (inclusive)
                let left = if self.yaxis.visible && self.yaxis.width > 0 {
                    self.yaxis.width as i32 + 1
                } else {
                    0
                };
                let bottom = self.size().height as i32 - if self.xaxis.label_format.is_none() { 1 } else { 2 };
                let right = self.size().width.saturating_sub(1) as i32;
                surface.set_relative_clip(left, 0, right, bottom);
                let mut r = self.bar_rect(index as usize);

                // option 3
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

impl<T> OnResize for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_bars_layout();
        // neaaparat dupa repaint unde se calculeaza bars_width
        self.scrollbars.resize(
            self.bars_width as u64 + self.x_axis_left_margin() as u64 + 1,
            new_size.height as u64,
            &self.base,
        );
        self.update_scroll_pos_from_scrollbars();
    }
}

impl<T> OnMouseEvent for VBarChart<T>
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
                    if self.left_scroll > 0 {
                        self.left_scroll -= 1;
                        self.after_horizontal_scroll();
                    }
                    EventProcessStatus::Processed
                }
                MouseWheelDirection::Right | MouseWheelDirection::Down => {
                    if self.left_scroll < self.max_left_scroll() {
                        self.left_scroll += 1;
                        self.after_horizontal_scroll();
                    }
                    EventProcessStatus::Processed
                }
            },
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnKeyPressed for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                if self.left_scroll > 0 {
                    self.left_scroll -= 1;
                    self.after_horizontal_scroll();
                }
                EventProcessStatus::Processed
            }
            key!("Right") => {
                if self.left_scroll < self.max_left_scroll() {
                    self.left_scroll += 1;
                    self.after_horizontal_scroll();
                }
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
                if self.first_visible_bar > 0 {
                    self.first_visible_bar -= 1;
                    self.align_scroll_to_first_visible_bar();
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
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

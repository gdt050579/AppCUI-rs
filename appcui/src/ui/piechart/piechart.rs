use super::draw_mode::{plot_area, DrawMode};
use super::events::{EventData, EventType};
use super::labels::{self, Place};
use super::selection;
use super::{Direction, DrawModeRenderer, DrawModeShape, Flags, HollowThickness, LabelContent, LabelLayout, Slice};
use crate::prelude::*;
use crate::system::RuntimeManager;
use crate::ui::common::control_event_wrapper::ControlEventData;

pub(super) const FULL: u64 = 1 << 32;

pub(super) const PERCENT_UNITS: u32 = 10_000;

pub(super) fn update_proportions(slices: &mut [Slice]) {
    let total: u128 = slices.iter().map(|s| s.value as u128).sum();
    if total == 0 {
        for s in slices.iter_mut() {
            s.start = 0;
            s.end = 0;
            s.percent = 0;
        }
        return;
    }
    // cumulative positions: end = cumulative_sum * FULL / total
    // (the last non-empty slice always ends exactly at FULL)
    let mut cumulative: u128 = 0;
    let mut pos: u64 = 0;
    for s in slices.iter_mut() {
        cumulative += s.value as u128;
        s.start = pos;
        pos = ((cumulative * FULL as u128) / total) as u64;
        s.end = pos;
    }
    // percents (largest remainder method)
    let mut percents = vec![0u32; slices.len()];
    largest_remainder(slices.iter().map(|s| s.value), total, PERCENT_UNITS, &mut percents);
    for (s, p) in slices.iter_mut().zip(percents) {
        s.percent = p as u16;
    }
}

pub(super) fn largest_remainder(values: impl Iterator<Item = u64>, total: u128, units: u32, output: &mut [u32]) {
    if total == 0 {
        output.iter_mut().for_each(|o| *o = 0);
        return;
    }
    let mut remainders: Vec<(u128, usize)> = Vec::with_capacity(output.len());
    let mut assigned: u32 = 0;
    for (index, value) in values.enumerate() {
        let scaled = value as u128 * units as u128;
        let share = (scaled / total) as u32;
        output[index] = share;
        assigned += share;
        remainders.push((scaled % total, index));
    }
    let left = (units - assigned) as usize;
    if left == 0 {
        return;
    }
    // largest remainder first, lower index first on ties
    remainders.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    for &(_, index) in remainders.iter().take(left) {
        output[index] += 1;
    }
}

#[CustomControl(overwrite = OnPaint + OnResize + OnMouseEvent + OnFocus, internal = true)]
/// A chart that shows a single series of values as parts of a whole.
///
/// Each value is a slice whose size is proportional to its share of the total.
/// Values are `u64`, so negative values cannot be added. A slice with value `0`
/// stays in the chart (and keeps its index) but takes no space.
pub struct PieChart {
    flags: Flags,
    pub(super) slices: Vec<Slice>,
    pub(super) surface: Surface,
    draw_mode: DrawMode,
    pixels: PaletteTile,
    cells: PaletteTile,
    pub(super) hovered_slice: Option<usize>,
    selected_slice: Option<usize>,
    label_layout: LabelLayout,
    label_content: LabelContent,
    label_places: Vec<Place>,
    number_format: FormatNumber,
}

impl PieChart {
    /// Creates an empty pie chart.
    ///
    /// `layout` places the control. `flags` selects optional behavior; see [`Flags`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            slices: Vec::new(),
            surface: Surface::new(1, 1),
            draw_mode: DrawMode::default(),
            pixels: PaletteTile::new(1, 1),
            cells: PaletteTile::new(1, 1),
            hovered_slice: None,
            selected_slice: None,
            label_layout: LabelLayout::None,
            label_content: LabelContent::Label,
            label_places: Vec::new(),
            number_format: FormatNumber::new(10).group(3, b','),
        }
    }

    /// Appends a slice to the chart and repaints it.
    ///
    /// `label` may be empty, in which case the slice has no label. When `attr` is
    /// `None`, the slice is drawn with an automatic color.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.add_value(50, "Rust", None);
    /// chart.add_value(30, "C++", Some(CharAttribute::with_fore_color(Color::Red)));
    /// chart.add_value(20, "", None);
    /// ```
    pub fn add_value(&mut self, value: u64, label: &str, attr: Option<CharAttribute>) {
        self.slices.push(Slice::new(value, label, attr));
        self.data_changed();
    }

    /// Removes all slices from the chart and repaints it.
    pub fn clear(&mut self) {
        self.slices.clear();
        self.data_changed();
    }

    /// Returns the number of slices in the chart.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.slices.len()
    }

    /// Returns the value of the slice at `index`, or `None` if `index` is out of range.
    #[inline(always)]
    pub fn value(&self, index: usize) -> Option<u64> {
        self.slices.get(index).map(|s| s.value)
    }

    /// Returns the label of the slice at `index`, or `None` if `index` is out of range.
    ///
    /// A slice added without a label returns `Some("")`.
    #[inline(always)]
    pub fn label(&self, index: usize) -> Option<&str> {
        self.slices.get(index).map(|s| s.label.as_str())
    }

    /// Changes the value of the slice at `index` and repaints the chart.
    /// Does nothing if `index` is out of range.
    pub fn set_value(&mut self, index: usize, value: u64) {
        if let Some(slice) = self.slices.get_mut(index) {
            if slice.value != value {
                slice.value = value;
                self.data_changed();
            }
        }
    }

    /// Returns the index of the selected slice, or `None` if no slice is selected.
    ///
    /// A slice is selected by clicking it, or with [`set_selected_slice`](Self::set_selected_slice).
    /// The selection is cleared when the chart loses the focus.
    #[inline(always)]
    pub fn selected_slice(&self) -> Option<usize> {
        self.selected_slice
    }

    /// Selects the slice at `index` and repaints the chart: a border is drawn around it and,
    /// with [`Flags::DimSlicesOnSelection`], the other slices are dimmed.
    ///
    /// Does nothing if `index` is out of range or the slice is not drawn (its value is `0`).
    /// The selection is cleared when the chart loses the focus, and when the selected slice
    /// is removed or its value becomes `0`.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::DimSlicesOnSelection);
    /// chart.add_value(50, "Rust", None);
    /// chart.add_value(30, "C++", None);
    /// chart.set_selected_slice(1);
    /// assert_eq!(chart.selected_slice(), Some(1));
    /// ```
    pub fn set_selected_slice(&mut self, index: usize) {
        let Some(slice) = self.slices.get(index) else {
            return;
        };
        if (slice.start != slice.end) && (self.selected_slice != Some(index)) {
            self.selected_slice = Some(index);
            self.repaint_surface();
        }
    }

    /// Clears the selection (if a slice is selected) and repaints the chart.
    pub fn clear_selected_slice(&mut self) {
        if self.selected_slice.is_some() {
            self.selected_slice = None;
            self.repaint_surface();
        }
    }

    /// Sets where the first slice starts and repaints the chart.
    ///
    /// `degrees` is an angle from `0` to `360`, measured clockwise from 12 o'clock: `0` (the
    /// default) starts at the top, `90` on the right, `180` at the bottom and `270` on the left.
    /// `360` is the same as `0`, and larger values are treated as `360`. The slices follow each
    /// other from there in the direction set with [`set_direction`](Self::set_direction).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// // the first slice starts at 3 o'clock
    /// chart.set_start_angle(90);
    /// ```
    pub fn set_start_angle(&mut self, degrees: u32) {
        let degrees = degrees.min(360);
        if self.draw_mode.start_angle != degrees {
            self.draw_mode.start_angle = degrees;
            self.repaint_surface();
        }
    }

    /// Sets the direction in which the slices follow each other, starting from the start angle
    /// (see [`set_start_angle`](Self::set_start_angle)), and repaints the chart.
    ///
    /// The default is [`Direction::Clockwise`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.set_direction(piechart::Direction::CounterClockwise);
    /// ```
    pub fn set_direction(&mut self, direction: Direction) {
        if self.draw_mode.direction != direction {
            self.draw_mode.direction = direction;
            self.repaint_surface();
        }
    }

    /// Sets the outline the chart is drawn in, and repaints the chart.
    ///
    /// The default is [`DrawModeShape::Circle`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.set_shape(piechart::DrawModeShape::Ellipse);
    /// ```
    pub fn set_shape(&mut self, shape: DrawModeShape) {
        self.draw_mode.shape = shape;
        self.repaint_surface();
    }

    /// Sets how the chart's pixels are turned into characters, and repaints the chart.
    ///
    /// The default is [`DrawModeRenderer::HalfBlocks`].
    pub fn set_renderer(&mut self, renderer: DrawModeRenderer) {
        self.draw_mode.renderer = renderer;
        self.repaint_surface();
    }

    /// Sets the size of the hole in the middle of the chart, and repaints the chart.
    ///
    /// The default is [`HollowThickness::None`] (no hole).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// // a ring: the hole is half the size of the circle
    /// chart.set_hollow_thickness(piechart::HollowThickness::Percentage(50));
    /// ```
    pub fn set_hollow_thickness(&mut self, hollow_thickness: HollowThickness) {
        self.draw_mode.hollow_thickness = hollow_thickness;
        self.repaint_surface();
    }

    /// Sets where the labels of the slices are drawn, and repaints the chart.
    ///
    /// The default is [`LabelLayout::None`] (no labels).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.add_value(50, "Rust", None);
    /// chart.add_value(30, "C++", None);
    /// chart.set_label_layout(piechart::LabelLayout::Outside);
    /// ```
    pub fn set_label_layout(&mut self, layout: LabelLayout) {
        self.label_layout = layout;
        self.repaint_surface();
    }

    /// Sets what the label of each slice shows, and repaints the chart.
    ///
    /// The default is [`LabelContent::Label`]. Labels are only drawn when the label
    /// layout is not [`LabelLayout::None`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.add_value(50, "Rust", None);
    /// chart.set_label_layout(piechart::LabelLayout::Inside);
    /// chart.set_label_content(piechart::LabelContent::LabelAndValue);
    /// ```
    pub fn set_label_content(&mut self, content: LabelContent) {
        self.label_content = content;
        labels::update_texts(&mut self.slices, content, &self.number_format);
        self.repaint_surface();
    }

    /// Sets how values are written in labels, and repaints the chart.
    ///
    /// The default groups digits by thousands (`1,234,567`).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut chart = PieChart::new(layout!("d:f"), piechart::Flags::None);
    /// chart.add_value(1500, "Disk", None);
    /// chart.set_label_layout(piechart::LabelLayout::Outside);
    /// chart.set_label_content(piechart::LabelContent::LabelAndValue);
    /// chart.set_number_format(FormatNumber::new(10).group(3, b',').suffix(" MB"));
    /// ```
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
        labels::update_texts(&mut self.slices, self.label_content, &self.number_format);
        self.repaint_surface();
    }

    fn data_changed(&mut self) {
        update_proportions(&mut self.slices);
        // the selected slice was removed, or takes no space any more
        if let Some(index) = self.selected_slice {
            let drawn = match self.slices.get(index) {
                Some(s) => s.start != s.end,
                None => false,
            };
            if !drawn {
                self.selected_slice = None;
            }
        }
        labels::update_texts(&mut self.slices, self.label_content, &self.number_format);
        self.repaint_surface();
    }

    fn repaint_surface(&mut self) {
        let theme = RuntimeManager::get().theme();
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', theme.chart.background));
        let size = self.surface.size();
        self.cells.resize(size.width, size.height);
        let (area, outside) = match self.label_layout {
            LabelLayout::Outside => labels::outside_area(size, &self.slices, &self.draw_mode),
            _ => (plot_area(size), false),
        };
        self.draw_mode.paint(&mut self.surface, theme, &self.slices, &mut self.pixels, &mut self.cells, area);
        update_centers(&mut self.slices, &self.cells);
        match (self.label_layout, area) {
            (LabelLayout::Inside, _) => labels::paint_inside(&self.slices, &mut self.surface, size, theme),
            (LabelLayout::Outside, Some(area)) if outside => {
                labels::paint_outside(&self.slices, &mut self.surface, &self.cells, area, &self.draw_mode, theme, &mut self.label_places)
            }
            _ => {}
        }
        if let Some(index) = self.selected_slice {
            if self.flags.contains(Flags::DimSlicesOnSelection) {
                selection::dim(&self.cells, index, &mut self.surface, theme.chart.inactive);
            }
            selection::paint_selection(&self.cells, index, &mut self.surface, theme.chart.selection_border);
        }
        self.hovered_slice = None;
    }

    // tells the event processor that the user changed the selection
    fn raise_selection_event(&mut self, index: Option<usize>) {
        let event_type = match index {
            Some(index) => EventType::SliceSelected(index),
            None => EventType::ClearSelection,
        };
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::PieChart(EventData { event_type }),
        });
    }

    // index of the slice drawn at (x, y), relative to the control
    pub(super) fn slice_index_at(&self, x: i32, y: i32) -> Option<usize> {
        if (x < 0) || (y < 0) {
            return None;
        }
        self.cells.get(x as u32, y as u32).map(|i| i as usize)
    }
}

pub(super) fn slice_at(slices: &[Slice], fraction: f64) -> usize {
    let pos = ((fraction * FULL as f64) as u64).min(FULL - 1);
    // first slice whose end is past `pos`
    let index = slices.partition_point(|s| s.end <= pos);
    index.min(slices.len() - 1)
}

// the center of a slice is the average of the cells it is drawn in (from the cell map); if
// that cell shows another slice or nothing (e.g. a slice wrapping around the hole of a ring),
// the slice's cell nearest to the average is used instead
pub(super) fn update_centers(slices: &mut [Slice], map: &PaletteTile) {
    for s in slices.iter_mut() {
        s.center = None;
        s.sum_x = 0;
        s.sum_y = 0;
        s.cells = 0;
    }
    let size = map.size();
    let (w, h) = (size.width, size.height);
    for y in 0..h {
        for x in 0..w {
            if let Some(s) = map.get(x, y).and_then(|i| slices.get_mut(i as usize)) {
                s.sum_x += x as u64;
                s.sum_y += y as u64;
                s.cells += 1;
            }
        }
    }
    for (index, s) in slices.iter_mut().enumerate() {
        if s.cells == 0 {
            continue;
        }
        // rounded average
        let ax = ((s.sum_x + s.cells / 2) / s.cells) as u32;
        let ay = ((s.sum_y + s.cells / 2) / s.cells) as u32;
        if map.get(ax, ay) == Some(index as u16) {
            s.center = Some(Point::new(ax as i32, ay as i32));
            continue;
        }
        // nearest cell of this slice; a cell is twice as tall as it is wide
        let mut best: Option<(u64, u32, u32)> = None;
        for y in 0..h {
            for x in 0..w {
                if map.get(x, y) == Some(index as u16) {
                    let dx = x.abs_diff(ax) as u64;
                    let dy = 2 * y.abs_diff(ay) as u64;
                    let d = dx * dx + dy * dy;
                    let closer = match best {
                        None => true,
                        Some((best_d, _, _)) => d < best_d,
                    };
                    if closer {
                        best = Some((d, x, y));
                    }
                }
            }
        }
        s.center = best.map(|(_, x, y)| Point::new(x as i32, y as i32));
    }
}

impl OnPaint for PieChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.is_enabled() {
            surface.draw_surface(0, 0, &self.surface);
        } else {
            // keep the outline of the chart visible: chart pixels take the inactive
            // foreground, everything else the inactive background
            let inactive = theme.chart.inactive;
            let background = theme.chart.background.background;
            let gray = |c: Color| if c == background { inactive.background } else { inactive.foreground };
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| {
                if ch.code == '▀' {
                    Some(Character::new(ch.code, gray(ch.foreground), gray(ch.background), CharFlags::None))
                } else {
                    Some(Character::with_attributes(ch.code, inactive))
                }
            });
        }
    }
}

impl OnMouseEvent for PieChart {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.hovered_slice = None;
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(pos) => {
                let index = self.slice_index_at(pos.x, pos.y);
                if index != self.hovered_slice {
                    self.hovered_slice = index;
                    if let Some(index) = index {
                        if let Some(slice) = self.slices.get(index) {
                            let anchor = slice.center.unwrap_or(*pos);
                            self.show_tooltip_on_point(slice.tooltip.as_str(), anchor.x, anchor.y);
                        }
                    } else {
                        self.hide_tooltip();
                    }
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(dt) => {
                let index = self.slice_index_at(dt.x, dt.y);
                if index != self.selected_slice {
                    self.selected_slice = index;
                    self.repaint_surface();
                    self.raise_selection_event(index);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnResize for PieChart {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.repaint_surface();
    }
}
impl OnFocus for PieChart {
    fn on_lose_focus(&mut self) {
        if self.selected_slice.is_some() {
            self.selected_slice = None;
            self.repaint_surface();
            self.raise_selection_event(None);
        }
    }
}

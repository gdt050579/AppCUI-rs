use super::{Flags, Slice};
use crate::prelude::*;

// Proportions
// -----------
// Positions along the chart are fixed-point values in `0..=FULL`, where `FULL`
// stands for the whole chart (360° for a circle, the full perimeter for a hollow
// rectangle). All sums are done in `u128`, so any number of `u64` values works.

/// The fixed-point value that represents the whole chart.
pub(super) const FULL: u64 = 1 << 32;

/// Percents are computed in hundredths of a percent, so they always add up to this value.
pub(super) const PERCENT_UNITS: u32 = 10_000;

/// Recomputes `start`, `end` and `percent` for every slice.
///
/// If the total is `0` (no slices, or all values are `0`), every slice gets
/// `start == end == 0` and `percent == 0`.
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

/// Splits `units` between `values` proportionally, so that the results add up to
/// exactly `units` (when `total > 0`).
///
/// Every value first gets the floor of its exact share. The units left over are
/// then given, one each, to the values with the largest remainders; ties go to
/// the value with the lower index. A value of `0` never receives a unit.
///
/// `total` must be the sum of `values`; `output` must have one entry per value.
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

#[CustomControl(overwrite = OnPaint + OnResize, internal = true)]
/// A chart that shows a single series of values as parts of a whole.
///
/// Each value is a slice whose size is proportional to its share of the total.
/// Values are `u64`, so negative values cannot be added. A slice with value `0`
/// stays in the chart (and keeps its index) but takes no space.
pub struct PieChart {
    flags: Flags,
    pub(super) slices: Vec<Slice>,
    surface: Surface,
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

    /// Recomputes the proportions of every slice and repaints the internal surface.
    fn data_changed(&mut self) {
        update_proportions(&mut self.slices);
        self.repaint_surface();
    }

    /// Repaints the internal surface from the current data and size.
    ///
    /// The surface is drawn by `on_paint`; it is only rebuilt when the data or the
    /// size of the control changes. Shapes are not drawn yet (step 3).
    fn repaint_surface(&mut self) {
        let background = self.theme().chart.background;
        self.surface.reset_clip();
        self.surface.clear(Character::with_attributes(' ', background));
    }
}

impl OnPaint for PieChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.is_enabled() {
            surface.draw_surface(0, 0, &self.surface);
        } else {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
        }
    }
}

impl OnResize for PieChart {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.repaint_surface();
    }
}

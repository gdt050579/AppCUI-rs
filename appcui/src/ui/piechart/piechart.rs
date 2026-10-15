use super::{Flags, Slice};
use crate::prelude::*;

#[CustomControl(internal = true)]
/// A chart that shows a single series of values as parts of a whole.
///
/// Each value is a slice whose size is proportional to its share of the total.
/// Values are `u64`, so negative values cannot be added. A slice with value `0`
/// stays in the chart (and keeps its index) but takes no space.
pub struct PieChart {
    flags: Flags,
    pub(super) slices: Vec<Slice>,
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
        }
    }

    /// Appends a slice to the chart.
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
    }

    /// Removes all slices from the chart.
    pub fn clear(&mut self) {
        self.slices.clear();
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

    /// Changes the value of the slice at `index`. Does nothing if `index` is out of range.
    pub fn set_value(&mut self, index: usize, value: u64) {
        if let Some(slice) = self.slices.get_mut(index) {
            slice.value = value;
        }
    }
}

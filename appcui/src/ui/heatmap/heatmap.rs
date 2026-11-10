use crate::prelude::*;
use super::{Flags, Cell};

#[CustomControl(overwrite = OnPaint, internal = true)]
/// A heat map control.
pub struct HeatMap {
    flags: Flags,
    cells: Vec<Option<Cell>>,
    size: Size,
}

impl HeatMap {
    /// Creates a heat map with the given layout and flags.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let heatmap = HeatMap::new(layout!("d:f"), heatmap::Flags::None);
    /// ```
    pub fn new(layout: Layout, size: Size, flags: Flags) -> Self {
        let count = size.width as usize * size.height as usize;
        let mut v = Vec::with_capacity(count.max(16));
        for _ in 0..count {
            v.push(None);
        }
        let mut me = Self {
            base: ControlBase::new(layout, true),
            flags,
            cells: v,
            size,
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }

}

impl OnPaint for HeatMap {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}

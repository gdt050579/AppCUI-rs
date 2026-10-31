use crate::prelude::*;
use super::Flags;

#[CustomControl(overwrite = OnPaint, internal = true)]
/// A heat map control.
pub struct HeatMap {
    #[allow(dead_code)]
    flags: Flags,
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
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut me = Self {
            base: ControlBase::new(layout, true),
            flags,
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }
}

impl OnPaint for HeatMap {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}

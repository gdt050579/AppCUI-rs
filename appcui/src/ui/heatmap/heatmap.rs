use super::{Cell, Flags};
use crate::prelude::*;

#[CustomControl(overwrite = OnPaint+OnResize, internal = true)]
/// A heat map control.
pub struct HeatMap {
    flags: Flags,
    cells: Vec<Option<Cell>>,
    size: Size,
    cell_size: Size,
    surface: Surface,
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
            cell_size: Size::new(2, 1),
            size,
            surface: Surface::new(1, 1),
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }
    fn update_view(&mut self) {
        self.surface.clear(Character::with_attributes(' ', self.theme().chart.background));
    }
}

impl OnPaint for HeatMap {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}

impl OnResize for HeatMap {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.update_view();
    }
}

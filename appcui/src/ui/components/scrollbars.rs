use super::scrollbars_components::HScrollBar;
use super::scrollbars_components::VScrollBar;
use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;
use crate::ui::ControlBase;

pub struct ScrollBars {
    horizontal: HScrollBar,
    vertical: VScrollBar,
    should_paint: bool,
}
impl ScrollBars {
    /// Creates a new instance of ScrollBars.
    /// The parameter `visible` indicates if the scrollbars should be visible or not.
    pub fn new(visible: bool) -> Self {
        Self {
            horizontal: HScrollBar::new(visible),
            vertical: VScrollBar::new(visible),
            should_paint: false,
        }
    }
    pub(crate) fn update(&mut self, horizontal_indexes: u64, vertical_indexes: u64, size: Size) {
        self.horizontal.update(size.width as u64, horizontal_indexes);
        self.vertical.update(size.height as u64, vertical_indexes);
    }
    /// Paints the scrollbars on the given surface with a givern theme, relative to a control
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        self.horizontal.paint(surface, theme, control);
        self.vertical.paint(surface, theme, control);
    }
    /// Processes a mouse event and returns true if the event was processed.
    /// The event is usually passed from a custom control from its implementation of the `OnMouseEvent` trait.
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        let mut r = self.horizontal.on_mouse_event(event);
        r |= self.vertical.on_mouse_event(event);
        self.should_paint = r.should_repaint();
        r.should_update()
    }

    /// Resizes the scrollbars based on the given horizontal and vertical indexes and the control size.
    /// The `horizontal_indexes` and `vertical_indexes` parameters are the maximum values for the scrollbars.
    /// The `control` parameter is the control that contains the scrollbars.
    /// This method is usually called from a custom control from its implementation of the `OnResize` trait.
    pub fn resize(&mut self, horizontal_indexes: u64, vertical_indexes: u64, control: &ControlBase) {
        let control_size = control.size();
        let left_margin = control.left_components_margin as i32;
        let top_margin = control.top_components_margin as i32;
        let w = (control_size.width as i32) - (left_margin + 1); // 2 space from right
        let h = (control_size.height as i32) - top_margin; // 1 space from bottom
        let x = left_margin;
        let y = top_margin;
        self.horizontal.recompute_position(x, w, control_size);
        self.vertical.recompute_position(y, h, control_size);
        self.update(horizontal_indexes, vertical_indexes, control_size);
    }

    /// Returns **true** if the scrollbars should be painted or false otherwise.
    pub fn should_repaint(&self) -> bool {
        self.should_paint
    }

    /// Sets the indexes (horizontal and vertical) for the scrollbars.
    /// The indexes will be clipped to the maximum values of the scrollbars.
    pub fn set_indexes(&mut self, horizontal: u64, vertical: u64) {
        self.horizontal.set_value(horizontal);
        self.vertical.set_value(vertical);
    }

    /// Returns the horizontal index of the scrollbar.
    #[inline(always)]
    pub fn horizontal_index(&self) -> u64 {
        self.horizontal.value()
    }

    /// Returns the vertical index of the scrollbar.
    #[inline(always)]
    pub fn vertical_index(&self) -> u64 {
        self.vertical.value()
    }

    /// Returns a point that represents the offset in a surface from where the content that is scrolled should be drawn.
    #[inline(always)]
    pub fn offset(&self) -> Point {
        let x = self.horizontal.value() as i32;
        let y = self.vertical.value() as i32;
        Point::new(-x, -y)
    }
}

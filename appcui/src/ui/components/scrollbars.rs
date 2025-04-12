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
    pub fn new(visible: bool) -> Self {
        Self {
            horizontal: HScrollBar::new(visible),
            vertical: VScrollBar::new(visible),
            should_paint: false,
        }
    }
    pub fn update(&mut self, horizontal_indexes: u64, vertical_indexes: u64, size: Size) {
        self.horizontal.update(size.width as u64, horizontal_indexes);
        self.vertical.update(size.height as u64, vertical_indexes);
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        self.horizontal.paint(surface, theme, control);
        self.vertical.paint(surface, theme, control);
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        let mut r = self.horizontal.on_mouse_event(event);
        r |= self.vertical.on_mouse_event(event);
        self.should_paint = r.should_repaint();
        r.should_update()
    }
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
    pub fn should_repaint(&self) -> bool {
        self.should_paint
    }
    pub fn set_indexes(&mut self, horizontal: u64, vertical: u64) {
        self.horizontal.set_value(horizontal);
        self.vertical.set_value(vertical);
    }
    #[inline(always)]
    pub fn horizontal_index(&self) -> u64 {
        self.horizontal.value()
    }
    #[inline(always)]
    pub fn vertical_index(&self) -> u64 {
        self.vertical.value()
    }
    #[inline(always)]
    pub fn offset(&self) -> Point {
        let x = self.horizontal.value() as i32;
        let y = self.vertical.value() as i32;
        Point::new(-x, -y)
    }
}

use crate::prelude::*;
use crate::ui::canvas::initialization_flags::Flags;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Canvas {
    surface: Surface,
    x: i32,
    y: i32,
    flags: Flags,
}
impl Canvas {
    pub fn new(canvas_size: Size, layout: Layout, flags: Flags) -> Self {
        let mut canvas = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            surface: Surface::new(canvas_size.width, canvas_size.height),
            x: 0,
            y: 0,
            flags,
        };
        canvas
    }
    pub fn resize_surface(&mut self, new_size: Size) {}
    #[inline(always)]
    pub fn get_surface(&mut self) -> &mut Surface {
        &mut self.surface
    }
    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.get_size();
        let surface_size = self.surface.get_size();
        self.x = if surface_size.width <= sz.width {
            0
        } else {
            x.max((sz.width as i32) - (surface_size.width as i32))
        };
        self.y = if surface_size.height <= sz.height {
            0
        } else {
            y.max((sz.height as i32) - (surface_size.height as i32))
        };
        self.x = self.x.min(0);
        self.y = self.y.min(0);
    }
}
impl OnPaint for Canvas {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(self.x, self.y, &self.surface);
    }
}
impl OnKeyPressed for Canvas {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Left") => {
                self.move_scroll_to(self.x - 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.move_scroll_to(self.x + 1, self.y);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for Canvas {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
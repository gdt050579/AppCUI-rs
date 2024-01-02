use crate::prelude::*;
use crate::ui::canvas::initialization_flags::Flags;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Canvas {
    surface: Surface,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
}
impl Canvas {
    pub fn new(canvas_size: Size, layout: Layout, flags: Flags) -> Self {
        let mut canvas = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            surface: Surface::new(canvas_size.width, canvas_size.height),
            x: 0,
            y: 0,
            background: None,
            flags,
        };
        canvas
    }
    pub fn resize_surface(&mut self, new_size: Size) {
        self.surface.resize(new_size);
    }
    #[inline(always)]
    pub fn get_drawing_surface(&mut self) -> &mut Surface {
        &mut self.surface
    }
    pub fn set_backgound(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }
    pub fn clear_background(&mut self) {
        self.background = None;
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
        if let Some(back) = self.background {
            surface.clear(back);
        }
        surface.draw_surface(self.x, self.y, &self.surface);
    }
}
impl OnKeyPressed for Canvas {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Left") => {
                self.move_scroll_to(self.x + 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.move_scroll_to(self.x - 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Up") => {
                self.move_scroll_to(self.x, self.y + 1);
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.move_scroll_to(self.x, self.y - 1);
                EventProcessStatus::Processed
            }
            key!("Alt+Left") => {
                self.move_scroll_to(0, self.y);
                EventProcessStatus::Processed
            }
            key!("Alt+Right") => {
                self.move_scroll_to(i32::MIN, self.y);
                EventProcessStatus::Processed
            }
            key!("Alt+Up") => {
                self.move_scroll_to(self.x, 0);
                EventProcessStatus::Processed
            }
            key!("Alt+Down") => {
                self.move_scroll_to(self.x, i32::MIN);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Left") => {
                self.move_scroll_to(self.x + self.get_size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
                self.move_scroll_to(self.x - self.get_size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Up") => {
                self.move_scroll_to(self.x, self.y + self.get_size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") => {
                self.move_scroll_to(self.x, self.y - self.get_size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Home") => {
                self.move_scroll_to(0, 0);
                EventProcessStatus::Processed
            }
            key!("End") => {
                self.move_scroll_to(i32::MIN, i32::MIN);
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

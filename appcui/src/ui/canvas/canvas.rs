use crate::prelude::*;
use crate::ui::canvas::initialization_flags::ScrollBarType;
use crate::ui::components::ScrollBar;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Canvas {
    surface: Surface,
    x: i32,
    y: i32,
    background: Option<Character>,
    scroll_bar_type: ScrollBarType,
    drag_point: Option<Point>,
    horizontal_scroll: ScrollBar,
    vertical_scroll: ScrollBar,
}
impl Canvas {
    pub fn new(canvas_size: Size, layout: Layout, scroll_bar_type: ScrollBarType) -> Self {
        let mut canvas = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            surface: Surface::new(canvas_size.width, canvas_size.height),
            x: 0,
            y: 0,
            background: None,
            scroll_bar_type,
            drag_point: None,
            horizontal_scroll: ScrollBar::new(0, 0, 1, false, 1),
            vertical_scroll: ScrollBar::new(0, 0, 1, true, 1),
        };
        let sz = canvas.surface.get_size();
        canvas.horizontal_scroll.set_count(sz.width as u64);
        canvas.vertical_scroll.set_count(sz.height as u64);
        canvas
    }
    pub fn resize_surface(&mut self, new_size: Size) {
        self.surface.resize(new_size);
        let mut sz = self.surface.get_size();
        self.horizontal_scroll.set_count(sz.width as u64);
        self.vertical_scroll.set_count(sz.height as u64);
        self.move_scroll_to(self.x, self.y);
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
        let sz = self
            .get_size()
            .reduce_by(if self.scroll_bar_type == ScrollBarType::Inside { 1 } else { 0 });
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
        self.horizontal_scroll.set_value((-self.x) as u64);
        self.vertical_scroll.set_value((-self.y) as u64);
    }
}
impl OnResize for Canvas {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        // reposition scroll bars
        let paint_sz = self.surface.get_size();
        let visible_size = new_size.reduce_by(if self.scroll_bar_type == ScrollBarType::Inside { 1 } else { 0 });
        self.horizontal_scroll.update_count(visible_size.width as u64, paint_sz.width as u64);
        self.vertical_scroll.update_count(visible_size.height as u64, paint_sz.height as u64);
        match self.scroll_bar_type {
            ScrollBarType::None => {
                self.horizontal_scroll.set_visible(false);
                self.vertical_scroll.set_visible(false);
            }
            ScrollBarType::Inside => {
                self.horizontal_scroll.update_position(new_size, 0, 1, false);
                self.vertical_scroll.update_position(new_size, 0, 1, false);
            }
            ScrollBarType::External => {
                self.horizontal_scroll.update_position(new_size, 0, 1, true);
                self.vertical_scroll.update_position(new_size, 0, 1, true);
            }
        }

        self.move_scroll_to(self.x, self.y);
    }
}
impl OnPaint for Canvas {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if let Some(back) = self.background {
            surface.clear(back);
        }
        if self.scroll_bar_type == ScrollBarType::Inside {
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        surface.draw_surface(self.x, self.y, &self.surface);
        match self.scroll_bar_type {
            ScrollBarType::None => {}
            ScrollBarType::Inside => {
                surface.reset_clip();
                self.vertical_scroll.paint(surface, theme, self);
                self.horizontal_scroll.paint(surface, theme, self);
            }
            ScrollBarType::External => {
                if self.has_focus() {
                    self.vertical_scroll.paint(surface, theme, self);
                    self.horizontal_scroll.paint(surface, theme, self);
                }
            }
        }
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
            key!("Shift+Left") => {
                self.move_scroll_to(0, self.y);
                EventProcessStatus::Processed
            }
            key!("Shift+Right") => {
                self.move_scroll_to(i32::MIN, self.y);
                EventProcessStatus::Processed
            }
            key!("Shift+Up") => {
                self.move_scroll_to(self.x, 0);
                EventProcessStatus::Processed
            }
            key!("Shift+Down") => {
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
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Ignored,
            MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = None;
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::None => {}
                    MouseWheelDirection::Left => self.move_scroll_to(self.x + 1, self.y),
                    MouseWheelDirection::Right => self.move_scroll_to(self.x - 1, self.y),
                    MouseWheelDirection::Up => self.move_scroll_to(self.x, self.y + 1),
                    MouseWheelDirection::Down => self.move_scroll_to(self.x, self.y - 1),
                };
                EventProcessStatus::Processed
            }
        }
    }
}

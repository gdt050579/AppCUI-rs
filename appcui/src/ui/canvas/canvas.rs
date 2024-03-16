use crate::prelude::components::ComponentsToolbar;
use crate::prelude::*;
use crate::ui::canvas::initialization_flags::Flags;
use crate::ui::components::ScrollBar;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Canvas {
    surface: Surface,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    components: ComponentsToolbar,
    horizontal_scrollbar: Handle<ScrollBar>,
    vertical_scrollbar: Handle<ScrollBar>,
}
impl Canvas {
    pub fn new(canvas_size: Size, layout: Layout, flags: Flags) -> Self {
        let mut canvas = Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags == Flags::ScrollBars {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            surface: Surface::new(canvas_size.width, canvas_size.height),
            x: 0,
            y: 0,
            flags,
            background: None,
            drag_point: None,
            components: ComponentsToolbar::with_capacity(if flags == Flags::ScrollBars { 2 } else { 0 }),
            horizontal_scrollbar: Handle::None,
            vertical_scrollbar: Handle::None,
        };
        if flags == Flags::ScrollBars {
            let sz = canvas.surface.get_size();
            canvas.horizontal_scrollbar = canvas.components.add(ScrollBar::new(sz.width as u64, false));
            canvas.vertical_scrollbar = canvas.components.add(ScrollBar::new(sz.width as u64, true));
        }
        canvas
    }
    pub fn resize_surface(&mut self, new_size: Size) {
        self.surface.resize(new_size);
        let sz = self.surface.get_size();
        if let Some(s) = self.components.get_mut(self.horizontal_scrollbar) {
            s.set_count(sz.width as u64);
        }
        if let Some(s) = self.components.get_mut(self.vertical_scrollbar) {
            s.set_count(sz.height as u64);
        }
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
        let sz = self.size();
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
        if let Some(s) = self.components.get_mut(self.horizontal_scrollbar) {
            s.set_index((-self.x) as u64);
        }
        if let Some(s) = self.components.get_mut(self.vertical_scrollbar) {
            s.set_index((-self.y) as u64);
        }
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        if let (Some(horiz), Some(vert)) = (
            self.components.get(self.horizontal_scrollbar),
            self.components.get(self.vertical_scrollbar),
        ) {
            let h = -(horiz.get_index() as i32);
            let v = -(vert.get_index() as i32);
            self.move_scroll_to(h, v);
        }
    }
}
impl OnResize for Canvas {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.components.on_resize(&self.base);
        let paint_sz = self.surface.get_size();
        if let Some(s) = self.components.get_mut(self.horizontal_scrollbar) {
            s.update_count(new_size.width as u64, paint_sz.width as u64)
        }
        if let Some(s) = self.components.get_mut(self.vertical_scrollbar) {
            s.update_count(new_size.height as u64, paint_sz.height as u64);
        }
        self.move_scroll_to(self.x, self.y);
    }
}
impl OnPaint for Canvas {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.components.paint(surface, theme, self);
            surface.reduce_clip_by(0,0,1,1);
        }
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
                self.move_scroll_to(self.x + self.size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Right") => {
                self.move_scroll_to(self.x - self.size().width as i32, self.y);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Up") | key!("PageUp")=> {
                self.move_scroll_to(self.x, self.y + self.size().height as i32);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Down") | key!("PageDown")=> {
                self.move_scroll_to(self.x, self.y - self.size().height as i32);
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
        let res = self.components.on_mouse_event(event);
        if res.should_update() {
            self.update_scroll_pos_from_scrollbars();
        }
        if res.should_pass_to_control() == false {
            if res.should_repaint() {
                return EventProcessStatus::Processed;
            } else {
                return EventProcessStatus::Ignored;
            }
        }
        let response = match event {
            MouseEvent::Enter => EventProcessStatus::Ignored,
            MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                if (self.flags == Flags::ScrollBars) && (self.has_focus()) {
                    let sz = self.size();
                    if (data.x == sz.width as i32) || (data.y == sz.height as i32) {
                        return EventProcessStatus::Ignored;
                    }
                }
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
        };
        // if one of the components require a repaint, than we should repaint even if the canvas required us to ignore the event
        if res.should_repaint() {
            return EventProcessStatus::Processed;
        } else {
            return response;
        }
    }
}


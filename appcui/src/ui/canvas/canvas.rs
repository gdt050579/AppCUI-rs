use crate::prelude::*;
use crate::ui::canvas::initialization_flags::Flags;

use self::components::ScrollBars;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct Canvas {
    surface: Surface,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars
}
impl Canvas {
    /// Creates a new canvas with the specified size, layout and flags.
    /// The flags can be a combination of the following values:
    /// * `Flags::ScrollBars` - if set, the canvas will have horizontal and vertical scrollbars
    /// 
    /// The parameter `canvas_size` is the size of the canvas (in characters), while the layout is the layout of the control.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut canvas = Canvas::new(Size::new(100, 100), Layout::new("x:1,y:1,w:30,h:10"), canvas::Flags::ScrollBars); 
    /// ```
    pub fn new(canvas_size: Size, layout: Layout, flags: Flags) -> Self {
        Self {
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
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars)
        }
    }


    /// Resizes the inner surface of the canvas. This will also update the scrollbars if they are present.
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut canvas = Canvas::new(Size::new(100, 100), Layout::new("x:1,y:1,w:30,h:10"), canvas::Flags::ScrollBars);
    /// canvas.resize_surface(Size::new(200, 200));
    /// ```
    pub fn resize_surface(&mut self, new_size: Size) {
        self.surface.resize(new_size);
        let sz = self.surface.size();
        self.scrollbars.update(sz.width as u64, sz.height as u64, self.size());
        self.move_scroll_to(self.x, self.y);
    }

    /// Returns a mutable reference to the inner surface of the canvas.
    #[inline(always)]
    pub fn get_drawing_surface(&mut self) -> &mut Surface {
        &mut self.surface
    }

    /// Sets the background of the canvas to the specified character.
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut canvas = Canvas::new(Size::new(100, 100), Layout::new("x:1,y:1,w:30,h:10"), canvas::Flags::ScrollBars);
    /// canvas.set_background(Character::new('*', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn set_background(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }

    /// Clears the background character of the canvas. It esentially resets it to transparent foreground and backgroud colors
    pub fn clear_background(&mut self) {
        self.background = None;
    }

    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.size();
        let surface_size = self.surface.size();
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
        self.scrollbars.set_indexes((-self.x) as u64, (-self.y) as u64);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h,v);
    }
}
impl OnResize for Canvas {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let paint_sz = self.surface.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64,&self.base);
        self.move_scroll_to(self.x, self.y);
    }
}
impl OnPaint for Canvas {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
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
        match key.value() {
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
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
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
        if self.scrollbars.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}


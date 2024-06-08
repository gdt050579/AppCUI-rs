use self::layout::Dimension;

use super::ResizeBehavior;
use super::SplitterPanel;
use crate::prelude::*;
use crate::ui::layout::Coordonate;

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    None,
    OverSeparator,
    OverLeftButton,
    OverRightButton,
    ClickedOnLeftButton,
    ClickedOnRightButton,
    Dragging,
}

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize, internal = true)]
pub struct VSplitter {
    left: Handle<SplitterPanel>,
    right: Handle<SplitterPanel>,
    min_left: Dimension,
    min_right: Dimension,
    pos: Coordonate,
    preserve_pos: i32,
    resize_behavior: ResizeBehavior,
    state: State,
}
impl VSplitter {
    pub fn new<T>(pos: T, layout: Layout, resize_behavior: ResizeBehavior) -> Self
    where
        Coordonate: From<T>,
    {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            left: Handle::None,
            right: Handle::None,
            pos: pos.into(),
            min_left: Dimension::Percentage(0),
            min_right: Dimension::Percentage(0),
            state: State::None,
            resize_behavior,
            preserve_pos: 0,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.left = obj.add_child(SplitterPanel::new());
        obj.right = obj.add_child(SplitterPanel::new());
        obj
    }

    /// Adds a new control to the specified panel of the splitter (left or right)
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut vs = VSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveRightPanelSize);
    /// vs.add(vsplitter::Panel::Left,button!("PressMe,x:1,y:1,w:12"));
    /// vs.add(vsplitter::Panel::Right,button!("PressMe,x:1,y:1,w:12"));   
    /// ```
    #[inline(always)]
    pub fn add<T>(&mut self, panel: vsplitter::Panel, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        let h = if panel == vsplitter::Panel::Left { self.left } else { self.right };
        let cm = RuntimeManager::get().get_controls_mut();
        if let Some(panel) = cm.get_mut(h.cast()) {
            panel.base_mut().add_child(control)
        } else {
            Handle::None
        }
    }
    pub fn set_min_width<T>(&mut self, panel: vsplitter::Panel, min_size: T)
    where
        Dimension: From<T>,
    {
        match panel {
            vsplitter::Panel::Left => self.min_left = min_size.into(),
            vsplitter::Panel::Right => self.min_right = min_size.into(),
        }
    }

    pub fn set_position<T>(&mut self, pos: T)
    where
        Coordonate: From<T>,
    {
        // force type conversion
        self.pos = pos.into();
        // update the position of the splitter
        self.update_position(self.pos, true);
    }
    fn update_position(&mut self, pos: Coordonate, upadate_preserve_position: bool) {
        let w = self.size().width as u16;
        let mut abs_value = pos.absolute(w);
        let min_left_margin = self.min_left.absolute(w);
        let min_right_margin = self.min_right.absolute(w);
        if abs_value >= (w as i32 - min_right_margin as i32) {
            abs_value = w as i32 - min_right_margin as i32 - 1;
        }
        abs_value = abs_value.max(min_left_margin as i32);
        match self.resize_behavior {
            ResizeBehavior::PreserveAspectRatio => self.pos.update_with_absolute_value(abs_value as i16, w),
            ResizeBehavior::PreserveLeftPanelSize | ResizeBehavior::PreserveRightPanelSize => {
                // if the position is preserverd, there is no need to keep the percentage
                self.pos = Coordonate::Absolute(abs_value as i16);
            }
        };
        self.update_panel_sizes(self.size());
        if upadate_preserve_position {
            match self.resize_behavior {
                ResizeBehavior::PreserveLeftPanelSize => {
                    self.preserve_pos = self.pos.absolute(w);
                }
                ResizeBehavior::PreserveRightPanelSize => {
                    self.preserve_pos = w as i32 - self.pos.absolute(w);
                }
                _ => {}
            }
        }
    }
    fn update_panel_sizes(&mut self, new_size: Size) {
        let w = self.pos.absolute(new_size.width as u16).max(0) as u16;
        let h = new_size.height as u16;
        let h1 = self.left;
        let h2 = self.right;
        let rm = RuntimeManager::get();
        if let Some(p1) = rm.get_control_mut(h1) {
            p1.set_position(0, 0);
            if w > 1 {
                p1.set_size(w, h);
                p1.set_visible(true);
            } else {
                p1.set_size(0, h);
                p1.set_visible(false);
            }
        }
        if let Some(p2) = rm.get_control_mut(h2) {
            p2.set_position(w as i32 + 1, 0);
            if (w as i32 + 1) < new_size.width as i32 {
                p2.set_size(new_size.width as u16 - w - 1, h);
                p2.set_visible(true);
            } else {
                p2.set_size(0, h);
                p2.set_visible(false);
            }
        }
    }
    fn mouse_to_state(&self, x: i32, y: i32, clicked: bool) -> State {
        let sz = self.size();
        let pos = self.pos.absolute(sz.width as u16);
        if x != pos {
            State::None
        } else if clicked {
            match y {
                1 => State::ClickedOnLeftButton,
                2 => State::ClickedOnRightButton,
                _ => State::Dragging,
            }
        } else {
            match y {
                1 => State::OverLeftButton,
                2 => State::OverRightButton,
                _ => State::OverSeparator,
            }
        }
    }
}
impl OnPaint for VSplitter {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let (col_line, col_b1, col_b2) = if !self.is_enabled() {
            (theme.lines.inactive, theme.symbol.inactive, theme.symbol.inactive)
        } else {
            match self.state {
                State::OverSeparator => (theme.lines.hovered, theme.symbol.arrows, theme.symbol.arrows),
                State::OverLeftButton => (theme.lines.normal, theme.symbol.hovered, theme.symbol.arrows),
                State::OverRightButton => (theme.lines.normal, theme.symbol.arrows, theme.symbol.hovered),
                State::ClickedOnLeftButton => (theme.lines.normal, theme.symbol.pressed, theme.symbol.arrows),
                State::ClickedOnRightButton => (theme.lines.normal, theme.symbol.arrows, theme.symbol.pressed),
                State::Dragging => (theme.lines.pressed_or_selectd, theme.symbol.arrows, theme.symbol.arrows),
                State::None => (theme.lines.normal, theme.symbol.arrows, theme.symbol.arrows),
            }
        };
        let sz = self.size();
        let x = self.pos.absolute(sz.width as u16);
        surface.draw_vertical_line_with_size(x, 0, sz.height, LineType::Single, col_line);
        surface.write_char(x, 1, Character::with_attributes(SpecialChar::TriangleLeft, col_b1));
        surface.write_char(x, 2, Character::with_attributes(SpecialChar::TriangleRight, col_b2));
    }
}
impl OnKeyPressed for VSplitter {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Alt+Left") => {
                let sz = self.size();
                if sz.width > 0 {
                    self.update_position(Coordonate::Absolute((self.pos.absolute(sz.width as u16) - 1) as i16), true);
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Right") => {
                let sz = self.size();
                if sz.width > 0 {
                    self.update_position(Coordonate::Absolute((self.pos.absolute(sz.width as u16) + 1) as i16), true);
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Left") => {
                self.update_position(Coordonate::Absolute(0), true);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Right") => {
                self.update_position(Coordonate::Absolute(self.size().width as i16), true);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for VSplitter {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.state = State::None;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let new_state = self.mouse_to_state(point.x, point.y, false);
                if new_state != self.state {
                    self.state = new_state;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Pressed(evn) => {
                let new_state = self.mouse_to_state(evn.x, evn.y, true);
                if new_state != self.state {
                    self.state = new_state;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Released(evn) => {
                let processed = match self.state {
                    State::ClickedOnLeftButton => {
                        self.update_position(Coordonate::Absolute(0), true);
                        true
                    }
                    State::ClickedOnRightButton => {
                        self.update_position(Coordonate::Absolute(self.size().width as i16), true);
                        true
                    }
                    _ => false,
                };
                let new_state = self.mouse_to_state(evn.x, evn.y, false);
                if (new_state != self.state) || processed {
                    self.state = new_state;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Drag(evn) => {
                if self.state == State::Dragging {
                    self.update_position(Coordonate::Absolute(evn.x as i16), true);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
impl OnResize for VSplitter {
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        let previous_width = old_size.width as i32;
        // recompute the position of the splitter
        match self.resize_behavior {
            ResizeBehavior::PreserveAspectRatio => {
                if (previous_width > 0) && (self.pos.is_absolute()) {
                    let ratio = self.pos.absolute(old_size.width as u16) as f32 / previous_width as f32;
                    let new_pos = (new_size.width as f32 * ratio) as i32;
                    self.update_position(Coordonate::Absolute(new_pos as i16), false);
                } else {
                    // first time (initialization) or already a percentage
                    self.update_panel_sizes(new_size);
                }
            }
            ResizeBehavior::PreserveLeftPanelSize => {
                if previous_width == 0 {
                    // first resize (initialize the splitter preserved position)
                    self.preserve_pos = self.pos.absolute(new_size.width as u16);
                    self.set_position(self.preserve_pos);
                } else {
                    self.update_position(Coordonate::Absolute(self.preserve_pos as i16), false);
                }
            }
            ResizeBehavior::PreserveRightPanelSize => {
                if previous_width == 0 {
                    // first resize (initialize the splitter preserved position)
                    self.preserve_pos = (new_size.width as i32 - self.pos.absolute(new_size.width as u16)).max(0);
                    let new_pos = (new_size.width as i32 - self.preserve_pos).max(0);
                    self.set_position(new_pos);
                } else {
                    let new_pos = (new_size.width as i32 - self.preserve_pos).max(0);
                    self.update_position(Coordonate::Absolute(new_pos as i16), false);
                }
            }
        }
    }
}

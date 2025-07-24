use self::layout::Dimension;

use super::ResizeBehavior;
use super::SplitterPanel;
use crate::prelude::*;
use crate::ui::layout::Coordinate;

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    None,
    OverSeparator,
    OverTopButton,
    OverBottomButton,
    ClickedOnTopButton,
    ClickedOnBottomButton,
    Dragging,
}

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize, internal = true)]
pub struct HSplitter {
    top: Handle<SplitterPanel>,
    bottom: Handle<SplitterPanel>,
    min_left: Dimension,
    min_right: Dimension,
    pos: Coordinate,
    preserve_pos: i32,
    resize_behavior: ResizeBehavior,
    state: State,
}
impl HSplitter {
    /// Creates a new Horizontal Splitter control with the specified position, layout and resize behavior
    /// The position can be a percentage (e.g. a float value) or an absolute value (e.g. an unsigned value)
    /// The resize behavior can be one of the following values:
    /// * `ResizeBehavior::PreserveAspectRatio` - the aspect ratio of the panels is preserved when resizing the control
    /// * `ResizeBehavior::PreserveTopPanelSize` - the size of the top panel is preserved when resizing the control
    /// * `ResizeBehavior::PreserveBottomPanelSize` - the size of the bottom panel is preserved when resizing the control
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut vs = HSplitter::new(0.5,layout!("d:f"),hsplitter::ResizeBehavior::PreserveTopPanelSize);
    /// vs.add(hsplitter::Panel::Top,button!("PressMe,x:1,y:1,w:12"));
    /// vs.add(hsplitter::Panel::Bottom,button!("PressMe,x:1,y:1,w:12"));
    /// ```
    pub fn new<T>(pos: T, layout: Layout, resize_behavior: ResizeBehavior) -> Self
    where
        Coordinate: From<T>,
    {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            top: Handle::None,
            bottom: Handle::None,
            pos: pos.into(),
            min_left: Dimension::Absolute(0),
            min_right: Dimension::Absolute(0),
            state: State::None,
            resize_behavior,
            preserve_pos: 0,
        };
        obj.set_size_bounds(1, 3, u16::MAX, u16::MAX);
        obj.top = obj.add_child(SplitterPanel::new());
        obj.bottom = obj.add_child(SplitterPanel::new());
        obj
    }

    /// Adds a new control to the specified panel of the splitter (top or bottom)
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut vs = HSplitter::new(0.5,layout!("d:f"),hsplitter::ResizeBehavior::PreserveTopPanelSize);
    /// vs.add(hsplitter::Panel::Top,button!("PressMe,x:1,y:1,w:12"));
    /// vs.add(hsplitter::Panel::Bottom,button!("PressMe,x:1,y:1,w:12"));   
    /// ```
    #[inline(always)]
    pub fn add<T>(&mut self, panel: hsplitter::Panel, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        let h = if panel == hsplitter::Panel::Top { self.top } else { self.bottom };
        let cm = RuntimeManager::get().get_controls_mut();
        if let Some(panel) = cm.get_mut(h.cast()) {
            panel.base_mut().add_child(control)
        } else {
            Handle::None
        }
    }

    /// Sets the minimum height for the top or bottom panel
    /// The value can be a percentage (e.g. a float value) or an absolute value (e.g. an unsigned value)
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut vs = HSplitter::new(0.5,layout!("d:f"),hsplitter::ResizeBehavior::PreserveTopPanelSize);
    /// vs.add(hsplitter::Panel::Top,button!("PressMe,x:1,y:1,w:12"));
    /// vs.add(hsplitter::Panel::Bottom,button!("PressMe,x:1,y:1,w:12"));
    /// // minim 2 chars from Top
    /// vs.set_min_height(hsplitter::Panel::Top,2);
    /// // minim 20% from Bottom
    /// vs.set_min_height(hsplitter::Panel::Bottom,0.2);
    /// ```
    pub fn set_min_height<T>(&mut self, panel: hsplitter::Panel, min_size: T)
    where
        Dimension: From<T>,
    {
        match panel {
            hsplitter::Panel::Top => self.min_left = min_size.into(),
            hsplitter::Panel::Bottom => self.min_right = min_size.into(),
        }
    }

    /// Returns the absolute position of the splitter (in characters)
    #[inline(always)]
    pub fn position(&self) -> i32 {
        self.pos.absolute(self.size().height.saturating_sub(1) as u16)
    }

    /// Sets the position of the splitter. The value can be a percentage (e.g. a float value) or an absolute value (e.g. an unsigned value)
    pub fn set_position<T>(&mut self, pos: T)
    where
        Coordinate: From<T>,
    {
        // force type conversion
        self.pos = pos.into();
        // update the position of the splitter
        self.update_position(self.pos, true);
    }
    fn update_position(&mut self, pos: Coordinate, upadate_preserve_position: bool) {
        let bottom_most = self.size().height.saturating_sub(1) as u16;
        let mut abs_value = pos.absolute(bottom_most);
        let min_top_margin = self.min_left.absolute(bottom_most);
        let min_bottom_margin = self.min_right.absolute(bottom_most);
        if abs_value > (bottom_most as i32 - min_bottom_margin as i32) {
            abs_value = bottom_most as i32 - min_bottom_margin as i32;
        }
        abs_value = abs_value.max(min_top_margin as i32);
        match self.resize_behavior {
            ResizeBehavior::PreserveAspectRatio => {
                self.pos.update_with_absolute_value(abs_value as i16, bottom_most);
            }
            ResizeBehavior::PreserveTopPanelSize | ResizeBehavior::PreserveBottomPanelSize => {
                // if the position is preserverd, there is no need to keep the percentage
                self.pos = Coordinate::Absolute(abs_value);
            }
        };
        self.update_panel_sizes(self.size());
        if upadate_preserve_position {
            match self.resize_behavior {
                ResizeBehavior::PreserveTopPanelSize => {
                    self.preserve_pos = self.pos.absolute(bottom_most);
                }
                ResizeBehavior::PreserveBottomPanelSize => {
                    self.preserve_pos = bottom_most as i32 - self.pos.absolute(bottom_most);
                }
                _ => {}
            }
        }
    }
    fn update_panel_sizes(&mut self, new_size: Size) {
        let splitter_pos = self.pos.absolute(new_size.height.saturating_sub(1) as u16).max(0) as u16;
        let w = new_size.width as u16;
        let h1 = self.top;
        let h2 = self.bottom;
        let rm = RuntimeManager::get();
        if let Some(p1) = rm.get_control_mut(h1) {
            p1.set_position(0, 0);
            if splitter_pos > 0 {
                p1.set_size(w, splitter_pos);
                p1.set_visible(true);
            } else {
                p1.set_size(w, 0);
                p1.set_visible(false);
            }
        }
        if let Some(p2) = rm.get_control_mut(h2) {
            p2.set_position(0, splitter_pos as i32 + 1);
            if (splitter_pos as i32) + 1 < (new_size.height as i32) {
                p2.set_size(w, new_size.height as u16 - splitter_pos - 1);
                p2.set_visible(true);
            } else {
                p2.set_size(w, 0);
                p2.set_visible(false);
            }
        }
    }
    fn mouse_to_state(&self, x: i32, y: i32, clicked: bool) -> State {
        let sz = self.size();
        let pos = self.pos.absolute(sz.height.saturating_sub(1) as u16);
        if y != pos {
            State::None
        } else if clicked {
            match x {
                1 => State::ClickedOnTopButton,
                2 => State::ClickedOnBottomButton,
                _ => State::Dragging,
            }
        } else {
            match x {
                1 => State::OverTopButton,
                2 => State::OverBottomButton,
                _ => State::OverSeparator,
            }
        }
    }
}
impl OnPaint for HSplitter {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let (col_line, col_b1, col_b2) = if !self.is_enabled() {
            (theme.lines.inactive, theme.symbol.inactive, theme.symbol.inactive)
        } else {
            match self.state {
                State::OverSeparator => (theme.lines.hovered, theme.symbol.arrows, theme.symbol.arrows),
                State::OverTopButton => (theme.lines.normal, theme.symbol.hovered, theme.symbol.arrows),
                State::OverBottomButton => (theme.lines.normal, theme.symbol.arrows, theme.symbol.hovered),
                State::ClickedOnTopButton => (theme.lines.normal, theme.symbol.pressed, theme.symbol.arrows),
                State::ClickedOnBottomButton => (theme.lines.normal, theme.symbol.arrows, theme.symbol.pressed),
                State::Dragging => (theme.lines.pressed_or_selectd, theme.symbol.arrows, theme.symbol.arrows),
                State::None => (theme.lines.normal, theme.symbol.arrows, theme.symbol.arrows),
            }
        };
        let sz = self.size();
        let y = self.pos.absolute(sz.height.saturating_sub(1) as u16);
        surface.draw_horizontal_line_with_size(0, y, sz.width, LineType::Single, col_line);
        surface.write_char(1, y, Character::with_attributes(SpecialChar::TriangleUp, col_b1));
        surface.write_char(2, y, Character::with_attributes(SpecialChar::TriangleDown, col_b2));
    }
}
impl OnKeyPressed for HSplitter {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Alt+UP") => {
                let sz = self.size();
                if sz.height > 0 {
                    self.update_position(Coordinate::Absolute(self.pos.absolute(sz.height.saturating_sub(1) as u16) - 1), true);
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Down") => {
                let sz = self.size();
                if sz.height > 0 {
                    self.update_position(Coordinate::Absolute(self.pos.absolute(sz.height.saturating_sub(1) as u16) + 1), true);
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Up") => {
                self.update_position(Coordinate::Absolute(0), true);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Down") => {
                self.update_position(Coordinate::Absolute(self.size().height.saturating_sub(1) as i32), true);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for HSplitter {
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
                    State::ClickedOnTopButton => {
                        self.update_position(Coordinate::Absolute(0), true);
                        true
                    }
                    State::ClickedOnBottomButton => {
                        self.update_position(Coordinate::Absolute(self.size().height.saturating_sub(1) as i32), true);
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
                    self.update_position(Coordinate::Absolute(evn.y), true);
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
impl OnResize for HSplitter {
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        let previous_width = old_size.height as i32;
        // recompute the position of the splitter
        match self.resize_behavior {
            ResizeBehavior::PreserveAspectRatio => {
                if (previous_width > 0) && (self.pos.is_absolute()) {
                    let ratio = self.pos.absolute(old_size.height.saturating_sub(1) as u16) as f32 / previous_width as f32;
                    let new_pos = (new_size.height as f32 * ratio) as i32;
                    self.update_position(Coordinate::Absolute(new_pos), false);
                } else {
                    // first time (initialization) or already a percentage
                    self.update_panel_sizes(new_size);
                }
            }
            ResizeBehavior::PreserveTopPanelSize => {
                if previous_width == 0 {
                    // first resize (initialize the splitter preserved position)
                    self.preserve_pos = self.pos.absolute(new_size.height.saturating_sub(1) as u16);
                    self.set_position(self.preserve_pos);
                } else {
                    self.update_position(Coordinate::Absolute(self.preserve_pos), false);
                }
            }
            ResizeBehavior::PreserveBottomPanelSize => {
                if previous_width == 0 {
                    // first resize (initialize the splitter preserved position)
                    self.preserve_pos =
                        (new_size.height.saturating_sub(1) as i32 - self.pos.absolute(new_size.height.saturating_sub(1) as u16)).max(0);
                    let new_pos = (new_size.height.saturating_sub(1) as i32 - self.preserve_pos).max(0);
                    self.set_position(new_pos);
                } else {
                    let new_pos = (new_size.height.saturating_sub(1) as i32 - self.preserve_pos).max(0);
                    self.update_position(Coordinate::Absolute(new_pos), false);
                }
            }
        }
    }
}

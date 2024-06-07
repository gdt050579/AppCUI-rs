use self::layout::Dimension;

use super::Flags;
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

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize + OnFocus, internal = true)]
pub struct VSplitter {
    left: Handle<SplitterPanel>,
    right: Handle<SplitterPanel>,
    min_left: Dimension,
    min_right: Dimension,
    pos: Coordonate,
    flags: Flags,
    state: State,
}
impl VSplitter {
    pub fn new<T>(pos: T, layout: Layout, flags: Flags) -> Self
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
            flags,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.left = obj.add_child(SplitterPanel::new());
        obj.right = obj.add_child(SplitterPanel::new());
        obj
    }
    pub fn set_left_min_size<T>(&mut self, min_size: T)
    where
        Dimension: From<T>,
    {
        self.min_left = min_size.into();
    }
    pub fn set_right_min_size<T>(&mut self, min_size: T)
    where
        Dimension: From<T>,
    {
        self.min_right = min_size.into();
    }
    pub fn set_position<T>(&mut self, pos: T)
    where
        Coordonate: From<T>,
    {
        // force type conversion
        self.pos = pos.into();
        // update the position of the splitter
        self.update_position(self.pos);
    }
    fn update_position(&mut self, pos: Coordonate) {
        let w = self.size().width as u16;
        let mut abs_value = pos.absolute(w);
        let min_left_margin = self.min_left.absolute(w);
        let min_right_margin = self.min_right.absolute(w);
        if abs_value >= (w as i32 - min_right_margin as i32) {
            abs_value = w as i32 - min_right_margin as i32 - 1;
        }
        abs_value = abs_value.max(min_left_margin as i32);
        self.pos.update_with_absolute_value(abs_value as i16, w);
        self.update_panel_sizes(self.size());
    }
    fn update_panel_sizes(&mut self, new_size: Size) {
        let w = self.pos.absolute(new_size.width as u16).max(0) as u16;
        let h = new_size.height as u16;
        let h1 = self.left;
        let h2 = self.right;
        let rm = RuntimeManager::get();
        if let Some(p1) = rm.get_control_mut(h1) {
            p1.set_position(0, 0);
            if w > 2 {
                p1.set_size(w - 1, h);
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
        } else {
            if clicked {
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
        let x = self.pos.absolute(sz.width as u16) as i32;
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
                    self.update_position(Coordonate::Absolute((self.pos.absolute(sz.width as u16) - 1) as i16));
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Right") => {
                let sz = self.size();
                if sz.width > 0 {
                    self.update_position(Coordonate::Absolute((self.pos.absolute(sz.width as u16) + 1) as i16));
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Left") => {
                self.update_position(Coordonate::Absolute(0));
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Shift+Right") => {
                self.update_position(Coordonate::Absolute(self.size().width as i16));
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
                        self.update_position(Coordonate::Absolute(0));
                        true
                    }
                    State::ClickedOnRightButton => {
                        self.update_position(Coordonate::Absolute(self.size().width as i16));
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
                    self.update_position(Coordonate::Absolute(evn.x as i16));
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
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        // recompute the position of the splitter

        // set the size of panels
        self.update_panel_sizes(new_size);
    }
}
impl OnFocus for VSplitter {
    fn on_focus(&mut self) {}

    fn on_lose_focus(&mut self) {}
}

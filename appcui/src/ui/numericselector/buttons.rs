use crate::graphics::*;
use crate::prelude::MouseEvent;
use crate::system::*;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ButtonState {
    Normal,
    Hovered,
    Inactive,
    Pressed,
}

impl ButtonState {
    #[inline(always)]
    fn color(&self, theme: &Theme, is_enable: bool) -> CharAttribute {
        if is_enable {
            match self {
                ButtonState::Normal => theme.button.regular.text.normal,
                ButtonState::Hovered => theme.button.regular.text.hovered,
                ButtonState::Inactive => theme.button.regular.text.inactive,
                ButtonState::Pressed => theme.button.regular.text.pressed_or_selectd,
            }
        } else {
            theme.button.regular.text.inactive
        }
    }
    #[inline(always)]
    fn clear(&mut self) {
        if *self != ButtonState::Inactive {
            *self = ButtonState::Normal;
        }
    }
    #[inline(always)]
    fn set(&mut self, new_state: ButtonState) -> bool {
        if *self != ButtonState::Inactive && *self != new_state {
            *self = new_state;
            return true;
        }
        false
    }
    #[inline(always)]
    fn is_accesible(&self) -> bool {
        *self != ButtonState::Inactive
    }
}
pub(super) struct ButtonResponse {
    pub(super) repaint: bool,
    pub(super) forward_to_control: bool,
    pub(super) click_on_add: bool,
    pub(super) click_on_sub: bool,
}
pub(super) struct Buttons {
    sub: ButtonState,
    add: ButtonState,
    width: u16,
}
impl Buttons {
    pub(super) fn new() -> Self {
        Self {
            sub: ButtonState::Normal,
            add: ButtonState::Normal,
            width: 0,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, is_enable: bool) {
        let sub_attr = self.sub.color(theme, is_enable);
        let add_attr = self.add.color(theme, is_enable);
        surface.write_string(0, 0, " - ", sub_attr, false);
        surface.write_string(self.width as i32 - 3, 0, " + ", add_attr, false);
    }
    #[inline(always)]
    pub(super) fn update_width(&mut self, width: u16) {
        self.width = width;
    }
    #[inline(always)]
    fn is_on_sub(&self, x: i32, y: i32) -> bool {
        ((0..3).contains(&x) && (y == 0)) && self.sub.is_accesible()
    }
    #[inline(always)]
    fn is_on_add(&self, x: i32, y: i32) -> bool {
        ((x >= (self.width as i32 - 3)) && (x < (self.width as i32)) && (y == 0)) && self.add.is_accesible()
    }
    #[inline(always)]
    fn process_mouse_event(&mut self, x: i32, y: i32, new_state: ButtonState, click: bool) -> ButtonResponse {
        let mut repaint = false;
        let on_sub = self.is_on_sub(x, y);
        let on_add = self.is_on_add(x, y);
        if on_sub {
            repaint |= self.sub.set(new_state);
        } else {
            repaint |= self.sub.set(ButtonState::Normal);
        }
        if on_add {
            repaint |= self.add.set(new_state);
        } else {
            repaint |= self.add.set(ButtonState::Normal);
        }
        ButtonResponse {
            repaint,
            forward_to_control: (!on_sub) && (!on_add),
            click_on_add: on_add & click,
            click_on_sub: on_sub & click,
        }
    }
    pub(super) fn disable_buttons(&mut self, disable_sub: bool, disable_add: bool) {
        if disable_sub {
            self.sub = ButtonState::Inactive;
        } else if self.sub == ButtonState::Inactive {
            self.sub = ButtonState::Normal;
        }
        if disable_add {
            self.add = ButtonState::Inactive;
        } else if self.add == ButtonState::Inactive {
            self.add = ButtonState::Normal;
        }   
    }
    pub(super) fn on_mouse_event(&mut self, event: &MouseEvent) -> ButtonResponse {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.sub.clear();
                self.add.clear();
                ButtonResponse {
                    repaint: true,
                    forward_to_control: true,
                    click_on_add: false,
                    click_on_sub: false,
                }
            }
            MouseEvent::Over(point) => self.process_mouse_event(point.x, point.y, ButtonState::Hovered, false),
            MouseEvent::Pressed(data) => self.process_mouse_event(data.x, data.y, ButtonState::Pressed, true),
            MouseEvent::Released(data) => self.process_mouse_event(data.x, data.y, ButtonState::Hovered, false),
            MouseEvent::DoubleClick(data) => self.process_mouse_event(data.x, data.y, ButtonState::Hovered, true),
            MouseEvent::Drag(_) | MouseEvent::Wheel(_) => ButtonResponse {
                repaint: false,
                forward_to_control: true,
                click_on_add: false,
                click_on_sub: false,
            },
        }
    }
}

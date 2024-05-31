use crate::graphics::*;
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
                ButtonState::Normal => theme.button.text.normal,
                ButtonState::Hovered => theme.button.text.hovered,
                ButtonState::Inactive => theme.button.text.inactive,
                ButtonState::Pressed => theme.button.text.pressed_or_selectd,
            }
        } else {
            theme.button.text.inactive
        }
    }
    #[inline(always)]
    fn update_state(&mut self, check: bool, expected_value: ButtonState) -> bool {
        if (check) && (*self != expected_value) {
            *self = expected_value;
            return true;
        }
        if (!check) && (*self == expected_value) {
            *self = ButtonState::Normal;
            return true;
        }
        false
    }
    #[inline(always)]
    fn is_accesible(&self) -> bool {
        !matches!(self, ButtonState::Inactive)
    }
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
        surface.write_string(self.width as i32 - 4, 0, " + ", add_attr, false);
    }
    #[inline(always)]   
    pub(super) fn update_width(&mut self, width: u16) {
        self.width = width;
    }
}

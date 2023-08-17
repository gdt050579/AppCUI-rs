use crate::{graphics::CharAttribute, system::MenuTheme};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(super) enum MenuButtonState {
    Normal,
    Hovered,
    Pressed,
}
impl MenuButtonState {
    #[inline(always)]
    pub(super) fn get_color(&self, inactive: bool, color: &MenuTheme) -> CharAttribute {
        if inactive {
            color.text.inactive
        } else {
            match self {
                MenuButtonState::Normal => color.text.normal,
                MenuButtonState::Hovered => color.text.hovered,
                MenuButtonState::Pressed => color.text.pressed_or_selectd,
            }
        }
    }
}

use crate::graphics::CharAttribute;
use crate::system::Theme;

pub(super) enum ItemStatus {
    Current,
    Hovered,
    Inactive,
    Normal,
}

impl ItemStatus {
    pub(super) fn text_attribute(&self, theme: &Theme) -> CharAttribute {
        match self {
            ItemStatus::Current => theme.menu.text.pressed_or_selectd,
            ItemStatus::Hovered => theme.menu.text.hovered,
            ItemStatus::Inactive => theme.menu.text.inactive,
            ItemStatus::Normal => theme.menu.text.normal,
        }
    }
    pub(super) fn text_attribute_for_toggle(&self, theme: &Theme, selected: bool) -> CharAttribute {
        match self {
            ItemStatus::Current => theme.menu.text.pressed_or_selectd,
            ItemStatus::Hovered => {
                if selected {
                    theme.menu.text.pressed_or_selectd
                } else {
                    theme.menu.text.hovered
                }
            }
            ItemStatus::Inactive => theme.menu.text.inactive,
            ItemStatus::Normal => {
                if selected {
                    theme.menu.text.pressed_or_selectd
                } else {
                    theme.menu.text.normal
                }
            }
        }
    }
    pub(super) fn hotkey_attribute(&self, theme: &Theme) -> CharAttribute {
        match self {
            ItemStatus::Current => theme.menu.hotkey.pressed_or_selectd,
            ItemStatus::Hovered => theme.menu.hotkey.hovered,
            ItemStatus::Inactive => theme.menu.hotkey.inactive,
            ItemStatus::Normal => theme.menu.hotkey.normal,
        }
    }
    #[inline(always)]
    pub(super) fn is_hover_or_current(&self) -> bool {
        matches!(self, ItemStatus::Current | ItemStatus::Hovered)
    }
}

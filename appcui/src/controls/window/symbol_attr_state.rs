use crate::{system::Theme, graphics::CharAttribute};

use super::DecoratorPaintData;

#[derive(PartialEq)]
pub(super) enum SymbolAttrState {
    Hovered,
    Normal,
    Pressed,
    Inactive,
}
impl SymbolAttrState {
    pub(super) fn new(paint_data: &DecoratorPaintData) -> Self {
        if paint_data.current {
            if paint_data.is_current_item_pressed {
                SymbolAttrState::Pressed
            } else {
                // showChecked = ((Members->Focused) && (btn->IsChecked()));
                SymbolAttrState::Hovered
            }
        } else {
            if paint_data.focused {
                // showChecked = btn->IsChecked();
                SymbolAttrState::Normal
            } else {
                SymbolAttrState::Inactive
            }
        }
    }
    #[inline(always)]
    pub(super) fn get_attr(&self, theme: &Theme, default_attr: CharAttribute) -> CharAttribute {
        match self {
            SymbolAttrState::Hovered => theme.symbol.hovered,
            SymbolAttrState::Normal => default_attr,
            SymbolAttrState::Pressed => theme.symbol.pressed,
            SymbolAttrState::Inactive => theme.symbol.inactive,
        }
    }
    #[inline(always)]
    pub(super) fn get_button_attr(&self, theme: &Theme) -> CharAttribute {
        match self {
            SymbolAttrState::Hovered => theme.button.text.hovered,
            SymbolAttrState::Normal => theme.text.normal,
            SymbolAttrState::Pressed => theme.button.text.pressed_or_selectd,
            SymbolAttrState::Inactive => theme.text.inactive,
        }
    }
    #[inline(always)]
    pub(super) fn get_hotkey_attr(&self, theme: &Theme) -> CharAttribute {
        match self {
            SymbolAttrState::Hovered => theme.button.text.hovered,
            SymbolAttrState::Normal => theme.text.hot_key,
            SymbolAttrState::Pressed => theme.button.text.pressed_or_selectd,
            SymbolAttrState::Inactive => theme.text.inactive,
        }
    }
}

use crate::{
    graphics::{Character, Surface},
    system::{Handle, HandleSupport, Theme},
};

use super::{
    item_base::ItemBase, CloseButton, HotKey, Label, MaximizeRestoreButton, PaintData,
    ResizeCorner, Tag, Button, CheckBox
};

pub(crate) enum ToolBarItem {
    Label(Label),
    HotKey(HotKey),
    Tag(Tag),
    CloseButton(CloseButton),
    MaximizeRestoreButton(MaximizeRestoreButton),
    ResizeCorner(ResizeCorner),
    Button(Button),
    CheckBox(CheckBox)
}
impl ToolBarItem {
    pub(crate) fn get_base(&self) -> &ItemBase {
        match self {
            ToolBarItem::Label(item) => &item.base,
            ToolBarItem::HotKey(item) => &item.base,
            ToolBarItem::Tag(item) => &item.base,
            ToolBarItem::CloseButton(item) => &item.base,
            ToolBarItem::MaximizeRestoreButton(item) => &item.base,
            ToolBarItem::ResizeCorner(item) => &item.base,
            ToolBarItem::Button(item) => &item.base,
            ToolBarItem::CheckBox(item) => &item.base,
        }
    }
    pub(crate) fn get_base_mut(&mut self) -> &mut ItemBase {
        match self {
            ToolBarItem::Label(item) => &mut item.base,
            ToolBarItem::HotKey(item) => &mut item.base,
            ToolBarItem::Tag(item) => &mut item.base,
            ToolBarItem::CloseButton(item) => &mut item.base,
            ToolBarItem::MaximizeRestoreButton(item) => &mut item.base,
            ToolBarItem::ResizeCorner(item) => &mut item.base,
            ToolBarItem::Button(item) => &mut item.base,
            ToolBarItem::CheckBox(item) => &mut item.base,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let base = self.get_base();
        if !base.can_be_drawn() {
            return;
        }

        let from_left = base.get_gravity().is_on_left_side();
        match self {
            ToolBarItem::Label(item) => item.paint(surface, theme, data),
            ToolBarItem::HotKey(item) => item.paint(surface, theme, data),
            ToolBarItem::Tag(item) => item.paint(surface, theme, data),
            ToolBarItem::CloseButton(item) => item.paint(surface, theme, data),
            ToolBarItem::MaximizeRestoreButton(item) => item.paint(surface, theme, data),
            ToolBarItem::ResizeCorner(item) => item.paint(surface, theme, data),
            ToolBarItem::Button(item) => item.paint(surface, theme, data),
            ToolBarItem::CheckBox(item) => item.paint(surface, theme, data),
        };
        // separators
        if base.is_part_of_group() {
            if base.has_left_group_marker() {
                surface.write_char(
                    base.get_x() - 1,
                    base.get_y(),
                    Character::with_attributes('[', data.sep_attr),
                );
            } else if from_left {
                surface.write_char(
                    base.get_x() - 1,
                    base.get_y(),
                    Character::with_attributes('|', data.sep_attr),
                );
            }
            if base.has_right_group_marker() {
                surface.write_char(
                    base.get_x() + base.get_width(),
                    base.get_y(),
                    Character::with_attributes(']', data.sep_attr),
                );
            } else if !from_left {
                surface.write_char(
                    base.get_x() + base.get_width(),
                    base.get_y(),
                    Character::with_attributes('|', data.sep_attr),
                );
            }
        }
    }
}
impl HandleSupport<ToolBarItem> for ToolBarItem {
    fn get_handle(&self) -> Handle<ToolBarItem> {
        match self {
            ToolBarItem::Label(item) => item.handle.cast(),
            ToolBarItem::HotKey(item) => item.handle.cast(),
            ToolBarItem::Tag(item) => item.handle.cast(),
            ToolBarItem::CloseButton(item) => item.handle.cast(),
            ToolBarItem::MaximizeRestoreButton(item) => item.handle.cast(),
            ToolBarItem::ResizeCorner(item) => item.handle.cast(),
            ToolBarItem::Button(item) => item.handle.cast(),
            ToolBarItem::CheckBox(item) => item.handle.cast(),
        }
    }

    fn set_handle(&mut self, handle: Handle<ToolBarItem>) {
        match self {
            ToolBarItem::Label(item) => item.handle = handle.cast(),
            ToolBarItem::HotKey(item) => item.handle = handle.cast(),
            ToolBarItem::Tag(item) => item.handle = handle.cast(),
            ToolBarItem::CloseButton(item) => item.handle = handle.cast(),
            ToolBarItem::MaximizeRestoreButton(item) => item.handle = handle.cast(),
            ToolBarItem::ResizeCorner(item) => item.handle = handle.cast(),
            ToolBarItem::Button(item) => item.handle = handle.cast(),
            ToolBarItem::CheckBox(item) => item.handle = handle.cast(),
        }
    }
}

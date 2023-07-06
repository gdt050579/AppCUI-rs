use crate::{system::{Handle, HandleSupport, Theme}, graphics::{Surface, Character}};

use super::{item_base::ItemBase, Label, PaintData, HotKey, Tag};

pub(super) enum ToolBarItem {
    Label(Label),
    HotKey(HotKey),
    Tag(Tag),
}
impl ToolBarItem {
    pub(super) fn get_base(&self) -> &ItemBase {
        match self {
            ToolBarItem::Label(item) => &item.base,
            ToolBarItem::HotKey(item) => &item.base,
            ToolBarItem::Tag(item) => &item.base,
        }
    }
    pub(super) fn get_base_mut(&mut self) -> &mut ItemBase {
        match self {
            ToolBarItem::Label(item) => &mut item.base,
            ToolBarItem::HotKey(item) => &mut item.base,
            ToolBarItem::Tag(item) => &mut item.base,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let pos = self.get_base();
        if (pos.is_visible() == false) || (pos.is_hidden()) {
            return;
        }

        let from_left = pos.get_gravity().is_on_left_side();
        match self {
            ToolBarItem::Label(item) => item.paint(surface, theme, data),
            ToolBarItem::HotKey(item) => item.paint(surface, theme, data),
            ToolBarItem::Tag(item) => item.paint(surface, theme, data),
        };
        // separators
        if pos.is_part_of_group() {
            if pos.has_left_group_marker() {
                surface.write_char(
                    pos.get_x() - 1,
                    pos.get_y(),
                    Character::with_attributes('[', data.sep_attr),
                );
            } else if from_left {
                surface.write_char(
                    pos.get_x() - 1,
                    pos.get_y(),
                    Character::with_attributes('|', data.sep_attr),
                );
            }
            if pos.has_right_group_marker() {
                surface.write_char(
                    pos.get_x() + pos.get_width(),
                    pos.get_y(),
                    Character::with_attributes(']', data.sep_attr),
                );
            } else if !from_left {
                surface.write_char(
                    pos.get_x() + pos.get_width(),
                    pos.get_y(),
                    Character::with_attributes('|', data.sep_attr),
                );
            }
        }

    }
}
impl HandleSupport for ToolBarItem {
    fn get_handle(&self) -> Handle {
        match self {
            ToolBarItem::Label(item) => item.handle,
            ToolBarItem::HotKey(item) => item.handle,
            ToolBarItem::Tag(item) => item.handle,
        }
    }

    fn set_handle(&mut self, handle: Handle) {
        match self {
            ToolBarItem::Label(item) => item.handle = handle,
            ToolBarItem::HotKey(item) => item.handle = handle,
            ToolBarItem::Tag(item) => item.handle = handle,
        }
    }
}

use crate::{system::{Handle, HandleSupport, Theme}, graphics::{Surface, Character}};

use super::{position::Position, Label, PaintData, ToolbarItemLayout, HotKey};

pub(super) enum ToolBarItem {
    Label(Label),
    HotKey(HotKey),
}
impl ToolBarItem {
    pub(super) fn get_position(&self) -> &Position {
        match self {
            ToolBarItem::Label(item) => &item.position,
            ToolBarItem::HotKey(item) => &item.position,
        }
    }
    pub(super) fn get_position_mut(&mut self) -> &mut Position {
        match self {
            ToolBarItem::Label(item) => &mut item.position,
            ToolBarItem::HotKey(item) => &mut item.position,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let pos = self.get_position();
        if (pos.is_visible() == false) || (pos.is_hidden()) {
            return;
        }

        let from_left = match pos.get_layout() {
            ToolbarItemLayout::TopLeft | ToolbarItemLayout::BottomLeft => true,
            _ => false,
        };
        match self {
            ToolBarItem::Label(item) => item.paint(surface, theme, data),
            ToolBarItem::HotKey(item) => item.paint(surface, theme, data),
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
        }
    }

    fn set_handle(&mut self, handle: Handle) {
        match self {
            ToolBarItem::Label(item) => item.handle = handle,
            ToolBarItem::HotKey(item) => item.handle = handle,
        }
    }
}

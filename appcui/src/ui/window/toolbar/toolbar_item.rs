use crate::{
    graphics::{Character, Surface},
    input::Key,
    system::{Handle, HandleSupport, Theme},
};

use super::{item_base::ItemBase, Button, CheckBox, CloseButton, HotKey, Label, MaximizeRestoreButton, PaintData, ResizeCorner, SingleChoice, Tag};

pub(crate) enum ToolBarItem {
    Label(Label),
    HotKey(HotKey),
    Tag(Tag),
    CloseButton(CloseButton),
    MaximizeRestoreButton(MaximizeRestoreButton),
    ResizeCorner(ResizeCorner),
    Button(Button),
    CheckBox(CheckBox),
    SingleChoice(SingleChoice),
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
            ToolBarItem::SingleChoice(item) => &item.base,
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
            ToolBarItem::SingleChoice(item) => &mut item.base,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let base = self.get_base();
        if !base.can_be_drawn() {
            return;
        }
        match self {
            ToolBarItem::Label(item) => item.paint(surface, theme, data),
            ToolBarItem::HotKey(item) => item.paint(surface, theme, data),
            ToolBarItem::Tag(item) => item.paint(surface, theme, data),
            ToolBarItem::CloseButton(item) => item.paint(surface, theme, data),
            ToolBarItem::MaximizeRestoreButton(item) => item.paint(surface, theme, data),
            ToolBarItem::ResizeCorner(item) => item.paint(surface, theme, data),
            ToolBarItem::Button(item) => item.paint(surface, theme, data),
            ToolBarItem::CheckBox(item) => item.paint(surface, theme, data),
            ToolBarItem::SingleChoice(item) => item.paint(surface, theme, data),
        };
        // separators
        if base.supports_markers() {
            if base.has_left_group_marker() {
                surface.write_char(base.get_left() - 1, base.get_y(), Character::with_attributes('[', data.sep_attr));
            }
            if base.has_left_separator() {
                surface.write_char(base.get_left() - 1, base.get_y(), Character::with_attributes('|', data.sep_attr));
            }
            if base.has_right_separator() {
                surface.write_char(base.get_right(), base.get_y(), Character::with_attributes('|', data.sep_attr));
            }
            if base.has_right_group_marker() {
                surface.write_char(base.get_right(), base.get_y(), Character::with_attributes(']', data.sep_attr));
            }
        }
    }
    pub(super) fn hotkey(&self) -> Key {
        match self {
            ToolBarItem::Label(_) => Key::None,
            ToolBarItem::HotKey(_) => Key::None,
            ToolBarItem::Tag(_) => Key::None,
            ToolBarItem::CloseButton(_) => Key::None,
            ToolBarItem::MaximizeRestoreButton(_) => Key::None,
            ToolBarItem::ResizeCorner(_) => Key::None,
            ToolBarItem::Button(item) => item.caption.hotkey(),
            ToolBarItem::CheckBox(item) => item.caption.hotkey(),
            ToolBarItem::SingleChoice(item) => item.caption.hotkey(),
        }
    }
    #[inline(always)]
    pub(super) fn is_resize_corner(&self) -> bool {
        matches!(self, ToolBarItem::ResizeCorner(_))
    }
}
impl HandleSupport<ToolBarItem> for ToolBarItem {
    fn get_handle(&self) -> Handle<ToolBarItem> {
        self.get_base().get_handle().cast()
    }

    fn set_handle(&mut self, handle: Handle<ToolBarItem>) {
        self.get_base_mut().set_handle(handle.cast())
    }
}

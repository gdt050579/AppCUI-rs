use crate::{
    ui::common::{traits::EventProcessStatus, UIElement},
    graphics::{Character, Size, Surface, TextAlignament, TextFormat},
    input::{Key, KeyCode, KeyModifier},
    system::{RuntimeManager, Theme, Handle},
    utils::{Strategy, VectorIndex},
};

use super::{MenuBarItem, MenuHandle};

pub struct MenuBar {
    items: Vec<MenuBarItem>,
    x: i32,
    y: i32,
    width: u32,
    opened_item: VectorIndex,
    hovered_item: VectorIndex,
    count: usize,
    receiver_control_handle: Handle<UIElement>,
}

impl MenuBar {
    pub(crate) fn new(width: u32) -> Self {
        Self {
            items: Vec::with_capacity(8),
            x: 0,
            y: 0,
            count:0,
            width,
            opened_item: VectorIndex::Invalid,
            hovered_item: VectorIndex::Invalid,
            receiver_control_handle: Handle::None,
        }
    }
    pub (crate) fn update_positions(&mut self) {
        let mut x = 0;
        let mut idx = 0usize;
        for item in &mut self.items {
            if idx>=self.count { break; }
            item.x = x;
            x += 2 + (item.caption.get_chars_count() as i32);
            idx += 1;
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let x = x - self.x;
        let y = y - self.y;

        if y != 0 {
            return None;
        }
        for (index, item) in self.items.iter().enumerate() {
            if index>=self.count { break; }
            if (x >= item.x) && (x < (item.x + 2 + (item.caption.get_chars_count() as i32))) {
                return Some(index);
            }
        }
        return None;
    }
    pub(crate) fn set_position(&mut self, x: i32, y: i32, width: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.update_positions();
    }
    pub fn add(&mut self, handle: MenuHandle) {
        if self.receiver_control_handle.is_none() {
            return;
        }
        if let Some(menu) = RuntimeManager::get().get_menu(handle) {
            if self.count<self.items.len() {
                // overwrite an existing item
                self.items[self.count].set(handle, self.receiver_control_handle, &menu.caption);
            } else {
                self.items.push(MenuBarItem::new(handle,self.receiver_control_handle, &menu.caption));
            }
            self.count+=1;
        }
    }

    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        if let Some(idx) = self.mouse_position_to_index(x, y) {
            self.open(VectorIndex::from(idx));
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> EventProcessStatus {
        if let Some(idx) = self.mouse_position_to_index(x, y) {
            if self.hovered_item.index() != idx {
                self.hovered_item = VectorIndex::from(idx);
                // if MenuBar is already opened, moving a mouse over another menu will implicetely open that menu
                if self.opened_item.is_valid() {
                    self.open(self.hovered_item);
                }
                return EventProcessStatus::Processed;
            } else {
                // the same index (and a valid one)
                return EventProcessStatus::Ignored;
            }
        } else {
            if self.hovered_item.is_valid() {
                self.hovered_item = VectorIndex::Invalid;
                return EventProcessStatus::Processed;
            }
        }
        return EventProcessStatus::Ignored;
    }

    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<UIElement>) {
        self.receiver_control_handle = handle;
    }
    pub(crate) fn clear(&mut self) {
        self.count = 0;
        self.receiver_control_handle = Handle::None;
    }
    pub(crate) fn close(&mut self) {
        self.opened_item = VectorIndex::Invalid;
        self.hovered_item = VectorIndex::Invalid;
    }
    fn open(&mut self, index: VectorIndex) {
        self.opened_item = index;
        if index.in_range(self.count) {
            RuntimeManager::get().show_menu(
                self.items[index.index()].handle,
                self.items[index.index()].receiver_control_handle,
                self.x + self.items[index.index()].x,
                self.y + 1,
                Size::new(0, 0),
            )
        }
    }
    #[inline(always)]
    fn is_opened(&self) -> bool {
        return self.opened_item.is_valid();
    }
    pub(crate) fn on_key_event(&mut self, key: Key) -> EventProcessStatus {
        if self.is_opened() {
            if (key.code == KeyCode::Left) && (key.modifier == KeyModifier::None) {
                let mut idx = self.opened_item;
                idx.sub(1, self.count, Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                return EventProcessStatus::Processed;
            }
            if (key.code == KeyCode::Right) && (key.modifier == KeyModifier::None) {
                let mut idx = self.opened_item;
                idx.add(1, self.count, Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                return EventProcessStatus::Processed;
            }
        } else {
            for (index, item) in self.items.iter().enumerate() {
                if index>=self.count { break; }
                if item.caption.get_hotkey() == key {
                    self.open(VectorIndex::from(index));
                    return EventProcessStatus::Processed;
                }
            }
        }
        // check recursivelly if a shortcut key was not pressed
        for item in &self.items {
            // if (this->Items[tr]->Mnu.ProcessShortcutKey(keyCode))
            // {
            //     Close();
            //     return true;
            // }
        }

        // nothing to process
        return EventProcessStatus::Ignored;
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line_with_size(
            self.x,
            self.y,
            self.width,
            Character::with_attributes(' ', theme.menu.text.normal),
        );
        let mut format =
            TextFormat::single_line(0, self.y, theme.menu.text.normal, TextAlignament::Left);
        let open_idx = self.opened_item.index();
        let hover_idx = self.hovered_item.index();
        for (index, item) in self.items.iter().enumerate() {
            if index>=self.count {
                break;
            }
            format.x = self.x + item.x + 1;
            format.hotkey_pos = item.caption.get_hotkey_pos();
            format.chars_count = Some(item.caption.get_chars_count() as u16);
            format.char_attr = match () {
                _ if index == open_idx => theme.menu.text.pressed_or_selectd,
                _ if index == hover_idx => theme.menu.text.hovered,
                _ => theme.menu.text.normal,
            };
            if item.caption.has_hotkey() {
                format.hotkey_attr = Some(match () {
                    _ if index == open_idx => theme.menu.hotkey.pressed_or_selectd,
                    _ if index == hover_idx => theme.menu.hotkey.hovered,
                    _ => theme.menu.hotkey.normal,
                });
            }
            if (index == open_idx) || (index == hover_idx) {
                surface.fill_horizontal_line(
                    format.x - 1,
                    format.y,
                    format.x + item.caption.get_chars_count() as i32,
                    Character::with_attributes(' ', format.char_attr),
                );
            }
            surface.write_text(item.caption.get_text(), &format);
        }
    }
}

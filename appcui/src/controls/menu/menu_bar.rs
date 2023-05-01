use crate::{
    controls::events::EventProcessStatus,
    graphics::{Character, Surface, TextAlignament, TextFormat, Size},
    input::{Key, KeyCode, KeyModifier},
    system::{RuntimeManager, Theme},
    utils::{Caption, Strategy, VectorIndex},
};

use super::{Menu, MenuBarItem, MenuHandle};

pub(crate) struct MenuBar {
    items: Vec<MenuBarItem>,
    x: i32,
    y: i32,
    width: u32,
    opened_item: VectorIndex,
    hovered_item: VectorIndex,
}

impl MenuBar {
    pub(crate) fn new(width: u32) -> Self {
        Self {
            items: Vec::with_capacity(4),
            x: 0,
            y: 0,
            width,
            opened_item: VectorIndex::Invalid,
            hovered_item: VectorIndex::Invalid,
        }
    }
    fn update_positions(&mut self) {
        let mut x = 0;
        for item in &mut self.items {
            item.x = x;
            x += 2 + (item.caption.get_chars_count() as i32);
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let x = x - self.x;
        let y = y - self.y;

        if y != 0 {
            return None;
        }
        for (index, item) in self.items.iter().enumerate() {
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
    pub(crate) fn add(&mut self, menu: Menu, caption: Caption) -> MenuHandle {
        let menus = RuntimeManager::get().get_menus();
        let h = menus.add(menu);
        self.items.push(MenuBarItem {
            caption,
            x: 0,
            handle: h,
        });
        self.update_positions();
        return h;
    }
    pub(crate) fn get_menu(&self, handle: MenuHandle) -> Option<&mut Menu> {
        RuntimeManager::get().get_menus().get_mut(handle)
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
                return EventProcessStatus::Cancel;
            }
        } else {
            if self.hovered_item.is_valid() {
                self.hovered_item = VectorIndex::Invalid;
                return EventProcessStatus::Update;
            }
        }
        return EventProcessStatus::Ignored;
    }

    fn close(&mut self) {
        self.opened_item = VectorIndex::Invalid;
        self.hovered_item = VectorIndex::Invalid;
    }
    fn open(&mut self, index: VectorIndex) {
        self.opened_item = index;
        if index.in_range(self.items.len()) {
            RuntimeManager::get().show_menu(
                self.items[index.index()].handle,
                self.x + self.items[index.index()].x,
                self.y + 1,
                Size::new(0,0),
            )
            // Items[menuIndex]->Mnu.Show(this->Parent, this->X + Items[menuIndex]->X, this->Y + 1);
            // // set the owner
            // ((MenuContext*) (Items[menuIndex]->Mnu.Context))->Owner = this;
        }
    }
    #[inline(always)]
    fn is_opened(&self) -> bool {
        return self.opened_item.is_valid();
    }
    fn on_key_event(&mut self, key: Key) -> EventProcessStatus {
        if self.is_opened() {
            if (key.code == KeyCode::Left) && (key.modifier == KeyModifier::None) {
                let mut idx = self.opened_item;
                idx.sub(1, self.items.len(), Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                return EventProcessStatus::Processed;
            }
            if (key.code == KeyCode::Right) && (key.modifier == KeyModifier::None) {
                let mut idx = self.opened_item;
                idx.add(1, self.items.len(), Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                return EventProcessStatus::Processed;
            }
        } else {
            for (index, item) in self.items.iter().enumerate() {
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
            surface.write_text(item.caption.get_text(), &format);
            //GDT: spaces around the text are missing (to be fixed)
        }
    }
}

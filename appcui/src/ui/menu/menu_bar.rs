use crate::{
    graphics::{Character, Surface, TextAlignment},
    input::{Key, KeyCode, KeyModifier},
    prelude::TextFormatBuilder,
    system::{Handle, RuntimeManager, Theme},
    ui::common::traits::EventProcessStatus,
    utils::{Strategy, VectorIndex},
};

use super::{Menu, MenuBarItem_old};

/// A menu bar that displays a horizontal list of menus at the top of a window or desktop.
///
/// The menu bar is responsible for displaying and managing the top-level menus 
/// in an application. It handles user interactions, keyboard shortcuts, and visual 
/// presentation of menus.
pub struct MenuBar {
    items: Vec<MenuBarItem_old>,
    x: i32,
    y: i32,
    width: u32,
    opened_item: VectorIndex,
    hovered_item: VectorIndex,
    count: usize,
    receiver_control_handle: Handle<()>,
}

impl MenuBar {
    pub(crate) fn new(width: u32) -> Self {
        Self {
            items: Vec::with_capacity(8),
            x: 0,
            y: 0,
            count: 0,
            width,
            opened_item: VectorIndex::Invalid,
            hovered_item: VectorIndex::Invalid,
            receiver_control_handle: Handle::None,
        }
    }
    pub(crate) fn update_positions(&mut self) {
        // sort the data first
        self.items.sort_by_key(|i| i.order);
        let mut x = 0;
        for (idx, item) in self.items.iter_mut().enumerate() {
            if idx >= self.count {
                break;
            }
            item.x = x;
            x += 2 + (item.caption.chars_count() as i32);
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let x = x - self.x;
        let y = y - self.y;

        if y != 0 {
            return None;
        }
        for (index, item) in self.items.iter().enumerate() {
            if index >= self.count {
                break;
            }
            if (x >= item.x) && (x < (item.x + 2 + (item.caption.chars_count() as i32))) {
                return Some(index);
            }
        }
        None
    }
    pub(crate) fn set_position(&mut self, x: i32, y: i32, width: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.update_positions();
    }
    /// Adds a menu to the menu bar.
    ///
    /// This method adds the specified menu to the menu bar, making it accessible
    /// to the user. Menus are displayed in the order they are added.
    ///
    /// # Parameters
    /// * `handle` - A handle to the menu to add.
    /// * `order` - The order of the menu in the menu bar (lower values are displayed to the left)
    pub fn add(&mut self, handle: Handle<Menu>, order: u8) {
        if self.receiver_control_handle.is_none() {
            return;
        }
        if let Some(menu) = RuntimeManager::get().get_menu(handle) {
            if self.count < self.items.len() {
                // overwrite an existing item
                self.items[self.count].set(handle, self.receiver_control_handle, &menu.caption);
                self.items[self.count].set_order(order);
            } else {
                self.items.push(MenuBarItem_old::new(handle, self.receiver_control_handle, &menu.caption, order));
            }
            self.count += 1;
        }
    }

    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        if let Some(idx) = self.mouse_position_to_index(x, y) {
            self.open(VectorIndex::from(idx));
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
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
        } else if self.hovered_item.is_valid() {
            self.hovered_item = VectorIndex::Invalid;
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }

    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
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
                None,
            )
        }
    }
    #[inline(always)]
    pub(crate) fn is_opened(&self) -> bool {
        self.opened_item.is_valid()
    }
    fn process_shortcut(&mut self, key: Key) -> bool {
        for (index, item) in self.items.iter().enumerate() {
            if index >= self.count {
                break;
            }
            if item.caption.hotkey() == key {
                self.open(VectorIndex::from(index));
                return true;
            }
        }
        false
    }
    fn process_key(&mut self, key: Key) -> EventProcessStatus {
        match key.code {
            KeyCode::Left => {
                let mut idx = self.opened_item;
                idx.sub(1, self.count, Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                EventProcessStatus::Processed
            }
            KeyCode::Right => {
                let mut idx = self.opened_item;
                idx.add(1, self.count, Strategy::Rotate);
                if idx.is_valid() {
                    self.open(idx);
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
    pub(crate) fn on_key_event(&mut self, key: Key, menu_is_opened: bool) -> EventProcessStatus {
        if menu_is_opened {
            if key.modifier.is_empty() {
                return self.process_key(key);
            }
            if key.modifier == KeyModifier::Alt {
                // check if a shortcut was pressed
                if self.process_shortcut(key) {
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        } else {
            if key.modifier == KeyModifier::Alt {
                // check if a shortcut was pressed
                if self.process_shortcut(key) {
                    return EventProcessStatus::Processed;
                }
            }
            // else check all shortcuts
            let menus = RuntimeManager::get().get_menus();
            for (index, item) in self.items.iter().enumerate() {
                if index >= self.count {
                    break;
                }
                if let Some(menu) = menus.get_mut(item.handle) {
                    if menu.process_shortcut(key, item.receiver_control_handle) {
                        return EventProcessStatus::Processed;
                    }
                }
            }

            // nothing to process
            EventProcessStatus::Ignored
        }
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line_with_size(self.x, self.y, self.width, Character::with_attributes(' ', theme.menu.text.normal));
        let mut format = TextFormatBuilder::new()
            .position(0, self.y)
            .attribute(theme.menu.text.normal)
            .align(TextAlignment::Left)
            .build();
        let open_idx = self.opened_item.index();
        let hover_idx = self.hovered_item.index();
        for (index, item) in self.items.iter().enumerate() {
            if index >= self.count {
                break;
            }
            format.x = self.x + item.x + 1;
            format.set_chars_count(item.caption.chars_count() as u16);
            format.char_attr = match () {
                _ if index == open_idx => theme.menu.text.pressed_or_selectd,
                _ if index == hover_idx => theme.menu.text.hovered,
                _ => theme.menu.text.normal,
            };
            format.set_hotkey_from_caption(
                match () {
                    _ if index == open_idx => theme.menu.hotkey.pressed_or_selectd,
                    _ if index == hover_idx => theme.menu.hotkey.hovered,
                    _ => theme.menu.hotkey.normal,
                },
                &item.caption,
            );
            if (index == open_idx) || (index == hover_idx) {
                surface.fill_horizontal_line(
                    format.x - 1,
                    format.y,
                    format.x + item.caption.chars_count() as i32,
                    Character::with_attributes(' ', format.char_attr),
                );
            }
            surface.write_text(item.caption.text(), &format);
        }
    }
}

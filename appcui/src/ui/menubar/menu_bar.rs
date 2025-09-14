use std::usize;

use super::{MenuBarItem, MenuBarItemWrapper};
use crate::graphics::*;
use crate::input::Key;
use crate::system::Handle;
use crate::system::Theme;
use crate::ui::common::traits::EventProcessStatus;
use crate::ui::menubar::ItemStatus;
use crate::utils::HandleManager;

#[derive(Copy, Clone)]
struct MenuBarItemPos {
    idx: u32,
    x: i16,
    width: u8,
    order: u8,
}
pub struct MenuBar {
    manager: HandleManager<MenuBarItemWrapper>,
    shown_items: Vec<MenuBarItemPos>,
    receiver_control_handle: Handle<()>,
    width: u32,
    hovered_item_index: Option<usize>,
    current_item_index: Option<usize>,
}
impl MenuBar {
    pub(crate) fn new(width: u32) -> Self {
        Self {
            manager: HandleManager::with_capacity(16),
            shown_items: Vec::with_capacity(64),
            receiver_control_handle: Handle::None,
            width,
            hovered_item_index: None,
            current_item_index: None,
        }
    }
    pub fn add<T>(&mut self, item: T) -> Handle<T>
    where
        T: MenuBarItem,
    {
        self.manager.add(item.into_menuibartem()).cast()
    }
    pub fn get<T>(&self, menubaritem_hamdle: Handle<T>) -> Option<&T>
    where
        T: MenuBarItem,
    {
        let ref_item = self.manager.get(menubaritem_hamdle.cast())?;
        match ref_item {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub fn get_mut<T>(&mut self, menubaritem_hamdle: Handle<T>) -> Option<&mut T>
    where
        T: MenuBarItem,
    {
        let ref_item = self.manager.get_mut(menubaritem_hamdle.cast())?;
        match ref_item {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(crate) fn clear(&mut self) {
        self.shown_items.clear();
        self.receiver_control_handle = Handle::None;
    }
    pub(crate) fn close(&mut self) {
        todo!()
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if y != 0 {
            return None;
        }
        for (index, item) in self.shown_items.iter().enumerate() {
            let start = item.x as i32;
            let end = start + item.width as i32;
            if (x >= start) && (x < end) {
                return Some(index);
            }
        }
        None
    }
    pub(crate) fn update_positions(&mut self) {
        // sort the data first
        self.shown_items.sort_by_key(|i| i.order);
        let mut x = 0;
        for item in &mut self.shown_items {
            let idx = item.idx as usize;
            if let Some(obj) = self.manager.element_mut(idx) {
                obj.base_mut().set_x(x);
                let w = obj.base().width();
                item.x = x as i16;
                item.width = w;
                x += w as i32;
            }
        }
    }
    #[inline(always)]
    pub(crate) fn is_opened(&self) -> bool {
        self.current_item_index.is_some()
    }
    fn open(&mut self, index: usize) {
        if index < self.shown_items.len() {
            self.current_item_index = Some(index);
            let idx = self.shown_items[index].idx as usize;
            if let Some(elem) = self.manager.element_mut(idx) {
                elem.activate();
            }
        }
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line_with_size(0, 0, self.width, Character::with_attributes(' ', theme.menu.text.normal));
        let hover_index = self.hovered_item_index.unwrap_or(usize::MAX);
        let current_index = self.current_item_index.unwrap_or(usize::MAX);
        for (index, item) in self.shown_items.iter().enumerate() {
            if let Some(elem) = self.manager.element(item.idx as usize) {
                let status = if index == current_index {
                    ItemStatus::Current
                } else if index == hover_index {
                    ItemStatus::Hovered
                } else {
                    ItemStatus::Normal
                };
                elem.paint(surface, theme, status);
            }
        }
    }
    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        todo!()
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> EventProcessStatus {
        let new_hover_pos = self.mouse_position_to_index(x, y);
        if new_hover_pos == self.hovered_item_index {
            return EventProcessStatus::Ignored;
        }
        self.hovered_item_index = new_hover_pos;
        if let Some(idx) = new_hover_pos {
            if self.current_item_index.is_some() {
                self.open(idx);
            }
        }
        EventProcessStatus::Processed
    }
    pub(crate) fn on_key_event(&mut self, key: Key, menu_is_opened: bool) -> EventProcessStatus {
        todo!()
    }
    pub(crate) fn set_position(&mut self, x: i32, y: i32, width: u32) {
        todo!()
    }
    pub fn show<T>(&mut self, handle: Handle<T>)
    where
        T: MenuBarItem,
    {
        if self.receiver_control_handle.is_none() {
            return;
        }
        if let Some(item) = self.manager.get_mut(handle.cast()) {
            item.set_receiver_control_handle(self.receiver_control_handle);
            let base = item.base();
            self.shown_items.push(MenuBarItemPos {
                idx: handle.index() as u32,
                x: base.x() as i16,
                width: base.width(),
                order: base.order(),
            });
        }
    }
}

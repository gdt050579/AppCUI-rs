use super::{MenuBarItem, MenuBarItemWrapper};
use crate::graphics::Surface;
use crate::input::Key;
use crate::system::Handle;
use crate::system::Theme;
use crate::ui::common::traits::EventProcessStatus;
use crate::utils::HandleManager;

pub struct MenuBar {
    manager: HandleManager<MenuBarItemWrapper>,
    visible_indexes: Vec<(u32,u8)>,
    receiver_control_handle: Handle<()>,
}
impl MenuBar {
    pub(crate) fn new(width: u32) -> Self {
        Self {
            manager: HandleManager::with_capacity(16),
            visible_indexes: Vec::with_capacity(64),
            receiver_control_handle: Handle::None,
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
        self.visible_indexes.clear();
        self.receiver_control_handle = Handle::None;
    }
    pub(crate) fn close(&mut self) {
        todo!()
    }
    pub(crate) fn update_positions(&mut self) {
        // sort the data first
        self.visible_indexes.sort_by_key(|i| i.1);
        let mut x = 0;
        for vis in &self.visible_indexes {
            let idx = vis.0 as usize;
            if let Some(obj) = self.manager.element_mut(idx) {
                obj.base_mut().set_x(x);
                x += 2 + obj.base().width() as i32;
            }
        }
    }
    #[inline(always)]
    pub(crate) fn is_opened(&self) -> bool {
        todo!()
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        todo!()
    }
    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        todo!()
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> EventProcessStatus {
        todo!()
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
            let o = item.base().order();
            self.visible_indexes.push((handle.index() as u32,o));
        }

    }
}

use super::{ItemBase, ItemStatus, MenuBarItem, MenuBarItemWrapper, MenuBarPosition};
use crate::graphics::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::menu::Menu;

pub struct MenuEntry {
    handle: Handle<Menu>,
    receiver_control_handle: Handle<()>,
    pub(super) base: ItemBase,
}

impl MenuEntry {
    pub fn new(menu: Menu, order: u8, pos: MenuBarPosition) -> Self {
        let w = (menu.caption().chars_count().max(1) + 2).min(u8::MAX as usize) as u8;
        let h = RuntimeManager::get().add_menu(menu);
        Self {
            handle: h,
            receiver_control_handle: Handle::None,
            base: ItemBase::new(w, order, pos, true),
        }
    }
    pub fn with_handle(handle: Handle<Menu>, order: u8, pos: MenuBarPosition) -> Self {
        todo!()
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        if status.is_hover_or_current() {
            surface.fill_horizontal_line_with_size(
                self.base.x(),
                0,
                (self.base.width() as u32),
                Character::with_attributes(' ', status.text_attribute(theme)),
            );
        }
        if let Some(menu) = RuntimeManager::get().get_menu(self.handle) {
            let c = menu.caption();
            let mut format = TextFormatBuilder::new()
                .position(self.base.x() + 1, 0)
                .attribute(status.text_attribute(theme))
                .align(TextAlignment::Left)
                .chars_count(c.chars_count() as u16)
                .build();
            format.set_hotkey_from_caption(status.hotkey_attribute(theme), &c);
            surface.write_text(c.text(), &format);
        }
    }
    pub(super) fn on_activate(&mut self) {
        RuntimeManager::get().show_menu(self.handle, self.receiver_control_handle, self.base.x(), 1, None)
    }
}

impl MenuBarItem for MenuEntry {
    fn into_menuibartem(self) -> MenuBarItemWrapper {
        MenuBarItemWrapper::MenuEntry(self)
    }
}

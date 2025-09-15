use super::{ItemBase, ItemStatus, MenuBarItem, MenuBarItemWrapper, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, MenuHandleManager, RuntimeManager, Theme};
use crate::ui::menu::Menu;
use crate::utils::Caption;

pub struct MenuButton {
    handle: Handle<Menu>,
    receiver_control_handle: Handle<()>,
    caption: Caption,
    pub(super) base: ItemBase,
}

impl MenuButton {
    pub fn new(name: &str, menu: Menu, order: u8, pos: Side) -> Self {
        let h = RuntimeManager::get().add_menu(menu);
        Self::with_handle(name, h, order, pos)
    }
    pub fn with_handle(name: &str, handle: Handle<Menu>, order: u8, pos: Side) -> Self {
        let c = Caption::new(name, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = (c.chars_count().max(1) + 2).min(u8::MAX as usize) as u8;
        Self {
            handle: handle,
            receiver_control_handle: Handle::None,
            caption: c,
            base: ItemBase::new(w, order, pos, true),
        }
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        if status.is_hover_or_current() {
            surface.fill_horizontal_line_with_size(
                self.base.x(),
                0,
                self.base.width() as u32,
                Character::with_attributes(' ', status.text_attribute(theme)),
            );
        }
        let mut format = TextFormatBuilder::new()
            .position(self.base.x() + 1, 0)
            .attribute(status.text_attribute(theme))
            .align(TextAlignment::Left)
            .chars_count(self.caption.chars_count() as u16)
            .build();
        format.set_hotkey_from_caption(status.hotkey_attribute(theme), &self.caption);
        surface.write_text(self.caption.text(), &format);
    }
    pub(super) fn on_activate(&mut self) {
        RuntimeManager::get().show_menu(self.handle, self.receiver_control_handle, self.base.x(), 1, None)
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key, menus: &mut MenuHandleManager) -> bool {
        if (self.receiver_control_handle.is_none()) || (self.handle.is_none()) {
            false
        } else if let Some(menu) = menus.get_mut(self.handle) {
            menu.process_shortcut(key, self.receiver_control_handle)
        } else {
            false
        }
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        self.caption.hotkey()
    }
}

impl MenuBarItem for MenuButton {
    fn into_menuibartem(self) -> MenuBarItemWrapper {
        MenuBarItemWrapper::MenuEntry(self)
    }
}

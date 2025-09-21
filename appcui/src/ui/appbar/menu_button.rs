use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, MenuHandleManager, RuntimeManager, Theme};
use crate::ui::menu::Menu;
use crate::utils::Caption;

/// A menu button in the app bar. A menu button is a button that opens a menu when clicked.
/// 
/// # Examples
/// 
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// let menu = Menu::new();
/// // add items to the menu
/// let menu_button = appbar::MenuButton::new("MenuButton", menu, 0, appbar::Side::Left);
/// ```
pub struct MenuButton {
    handle: Handle<Menu>,
    receiver_control_handle: Handle<()>,
    caption: Caption,
    pub(super) base: ItemBase,
}

impl MenuButton {
    /// Creates a new menu button with the specified caption, menu, order and position.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the menu button.
    /// * `menu` - The menu to open when the button is clicked.
    /// * `order` - The order of the menu button (a number that determines the order of the menu button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the menu button (`Left` or `Right`)
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let menu = Menu::new();
    /// // add items to the menu
    /// let menu_button = appbar::MenuButton::new("MenuButton", menu, 0, appbar::Side::Left);
    /// ```
    pub fn new(caption: &str, menu: Menu, order: u8, pos: Side) -> Self {
        let h = RuntimeManager::get().add_menu(menu);
        Self::with_handle(caption, h, order, pos)
    }

    /// Creates a new menu button with the specified caption, menu handle, order and position.
    /// This is usually used when the menu was already registered into the framework (via `register_menu(...)` method from a control).
    /// This method is usually good when you want to keep both the handle to the menu and the handle to the menu button.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the menu button.
    /// * `handle` - The handle of the menu to open when the button is clicked.
    /// * `order` - The order of the menu button (a number that determines the order of the menu button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the menu button (`Left` or `Right`)
    pub fn with_handle(caption: &str, handle: Handle<Menu>, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = (c.chars_count().max(1) + 2).min(u8::MAX as usize) as u8;
        Self {
            handle,
            receiver_control_handle: Handle::None,
            caption: c,
            base: ItemBase::new(w, order, pos, true),
        }
    }

    /// Returns **true** if the menu button is enabled, **false** otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    /// Enables or disables the menu button.
    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    /// Returns the caption of the menu button.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }

    /// Sets the caption of the menu button. If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption = Caption::new(text, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = (self.caption.chars_count().max(1) + 2).min(u8::MAX as usize) as u8;
        self.base.set_width(w);
        self.base.refresh();
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
        RuntimeManager::get().show_menu(
            self.handle,
            self.receiver_control_handle,
            self.base.x(),
            1,
            self.base.width() as u32,
            None,
        )
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

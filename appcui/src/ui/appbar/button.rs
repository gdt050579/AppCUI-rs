use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar::events::AppBarEvent;
use crate::ui::appbar::events::ButtonClickEvent;
use crate::utils::Caption;

/// A button in the app bar.
/// 
/// # Examples
/// 
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// let button = appbar::Button::new("Button", 0, appbar::Side::Left);
/// ```
pub struct Button {
    receiver_control_handle: Handle<()>,
    caption: Caption,
    tooltip: String,
    pub(super) base: ItemBase,
}

impl Button {
    /// Creates a new button with the specified caption, order and position.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the button. If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    /// * `order` - The order of the button (a number that determines the order of the button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the button (`Left` or `Right`)
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let button = appbar::Button::new("Button", 0, appbar::Side::Left);
    /// ```
    pub fn new(caption: &str, order: u8, pos: Side) -> Self {
        Self::with_tooltip(caption, "", order, pos)
    }
    /// Creates a new button with the specified caption, tooltip, order and position.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the button. If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    /// * `tooltip` - The tooltip associated with the button (it will be displayed when the mouse is over the button)
    /// * `order` - The order of the button (a number that determines the order of the button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the button (`Left` or `Right`)
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let button = appbar::Button::with_tooltip("Button", "Tooltip", 0, appbar::Side::Left);
    /// ```
    pub fn with_tooltip(caption: &str, tooltip: &str, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = c.chars_count() as u8;
        Self {
            receiver_control_handle: Handle::None,
            caption: c,
            tooltip: if tooltip.is_empty() { String::new() } else { tooltip.to_string() },
            base: ItemBase::new(w, order, pos, true),
        }
    }

    /// Returns **true** if the button is enabled, **false** otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    /// Enables or disables the button.
    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    /// Returns the caption of the button.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }

    /// Sets the caption of the button. If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = self.caption.chars_count() as u8;
        self.base.set_width(w);
        self.base.refresh();
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        let mut format = TextFormatBuilder::new()
            .position(self.base.x(), 0)
            .attribute(status.text_attribute(theme))
            .align(TextAlignment::Left)
            .chars_count(self.caption.chars_count() as u16)
            .build();
        format.set_hotkey_from_caption(status.hotkey_attribute(theme), &self.caption);
        surface.write_text(self.caption.text(), &format);
    }
    pub(super) fn on_execute(&self) {
        RuntimeManager::get().set_appbar_event(AppBarEvent::ButtonClick(ButtonClickEvent {
            button_handle: self.base.handle().cast(),
            control_receiver_handle: self.receiver_control_handle,
        }));
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        self.caption.hotkey()
    }

    /// Returns the tooltip of the button.
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    /// Sets the tooltip of the button.
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str) {
        if self.tooltip != text {
            self.tooltip.clear();
            self.tooltip.push_str(text);
            self.base.refresh();
        }
    }
}

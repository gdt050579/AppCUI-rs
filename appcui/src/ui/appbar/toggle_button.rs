use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar::events::AppBarEvent;
use crate::ui::appbar::events::ToggleButtonStatusChangedEvent;
use crate::utils::Caption;

/// A toggle button in the app bar. A toggle button is a button that toggles between two states.
/// 
/// # Examples
/// 
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// let toggle_button = appbar::ToggleButton::new("ToggleButton", 
///                                        false, 
///                                        0, 
///                                        appbar::Side::Left);
/// ```
pub struct ToggleButton {
    receiver_control_handle: Handle<()>,
    caption: Caption,
    tooltip: String,
    selected: bool,
    pub(super) base: ItemBase,
}

impl ToggleButton {
    /// Creates a new toggle button with the specified caption, selected state, order and position.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the toggle button.
    /// * `selected` - The initial selected state.
    /// * `order` - The order of the toggle button (a number that determines the order of the toggle button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the toggle button (`Left` or `Right`)
    /// 
    /// **Remark:** If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let toggle_button = appbar::ToggleButton::new("ToggleButton", 
    ///                                        false, 
    ///                                        0, 
    ///                                        appbar::Side::Left);
    /// ```
    pub fn new(caption: &str, selected: bool, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = c.chars_count() as u8;
        Self {
            receiver_control_handle: Handle::None,
            caption: c,
            tooltip: String::new(),
            base: ItemBase::new(w, order, pos, true),
            selected,
        }
    }

    /// Creates a new toggle button with the specified caption, tooltip, selected state, order and position.
    /// 
    /// # Parameters
    /// 
    /// * `caption` - The caption of the toggle button.
    /// * `tooltip` - The tooltip of the toggle button.
    /// * `selected` - The initial selected state.
    /// * `order` - The order of the toggle button (a number that determines the order of the toggle button in the app bar - lower numbers are displayed first from either **left** or **right** depending on the **pos** parameter)
    /// * `pos` - The position of the toggle button (`Left` or `Right`)
    /// 
    /// **Remark:** If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let toggle_button = appbar::ToggleButton::with_tooltip(
    ///             "ToggleButton", 
    ///             "Tooltip for toggle button", 
    ///             false, 
    ///             0, 
    ///             appbar::Side::Left);
    /// ```
    pub fn with_tooltip(caption: &str, tooltip: &str, selected: bool, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = c.chars_count() as u8;
        Self {
            receiver_control_handle: Handle::None,
            caption: c,
            tooltip: tooltip.to_string(),
            base: ItemBase::new(w, order, pos, true),
            selected,
        }
    }

    /// Returns **true** if the toggle button is enabled, **false** otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    /// Enables or disables the toggle button.
    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    /// Returns **true** if the toggle button is selected, **false** otherwise.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Sets the selected state of the toggle button.
    #[inline(always)]
    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    /// Returns the caption of the toggle button.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }

    /// Sets the caption of the toggle button. If the caption contains the `&` character, the next character (if it is a letter or number) will be set as a hot-key for the button. For example, `"&Save"` will set the hot-key to `Alt+S`.
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
        let status = ItemStatus::toggle_status(status, self.selected);
        let mut format = TextFormatBuilder::new()
            .position(self.base.x(), 0)
            .attribute(status.text_attribute(theme))
            .align(TextAlignment::Left)
            .chars_count(self.caption.chars_count() as u16)
            .build();
        format.set_hotkey_from_caption(status.hotkey_attribute(theme), &self.caption);
        surface.write_text(self.caption.text(), &format);
    }
    pub(super) fn on_execute(&mut self) {
        self.selected = !self.selected;
        RuntimeManager::get().set_appbar_event(AppBarEvent::ToggleButtonStatusChanged(ToggleButtonStatusChangedEvent {
            button_handle: self.base.handle().cast(),
            control_receiver_handle: self.receiver_control_handle,
            state: self.selected,
        }));
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        self.caption.hotkey()
    }

    /// Returns the tooltip of the toggle button.
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    /// Sets the tooltip of the toggle button.
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str) {
        if self.tooltip != text {
            self.tooltip.clear();
            self.tooltip.push_str(text);
            self.base.refresh();
        }
    }
}

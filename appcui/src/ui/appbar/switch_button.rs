//! A two-state button for the app bar.
//!
//! [`SwitchButton`] shows one caption when it is selected and another when it
//! is not. An optional [`SwitchButtonSymbol`] is drawn beside the text.
//! Activating the button toggles the state and raises
//! [`AppBarEvents::on_switchbutton_state_changed`](super::events::AppBarEvents::on_switchbutton_state_changed).

use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar::events::AppBarEvent;
use crate::ui::appbar::events::SwitchButtonStatusChangedEvent;
use crate::utils::Caption;

/// Symbol drawn to the left of a [`SwitchButton`] caption.
///
/// [`None`](Self::None) adds no width. The other variants reserve two cells.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SwitchButtonSymbol {
    /// No symbol is drawn.
    None,
    /// `'✓'` when the button is selected, otherwise a space.
    CheckMark,
    /// `'🗹'` when the button is selected, otherwise `'☐'`.
    CheckBox,
}

impl SwitchButtonSymbol {
    fn char(&self, selected: bool) -> Option<char> {
        match self {
            SwitchButtonSymbol::None => None,
            SwitchButtonSymbol::CheckMark => Some(if selected { SpecialChar::CheckMark.into() } else { ' ' }),
            SwitchButtonSymbol::CheckBox => Some(if selected { '🗹' } else { '☐' }),
        }
    }
    fn width(&self) -> i32 {
        match self {
            SwitchButtonSymbol::None => 0,
            _ => 2,
        }
    }
}

/// An app bar button that switches between two captions.
///
/// The selected caption is shown while the button is selected, and the
/// unselected caption otherwise. Items are laid out by `order` (lower first)
/// from [`Side::Left`] or [`Side::Right`].
///
/// A caption may mark a hotkey with `&` before a letter or digit. `"&Save"`
/// uses `Alt+S`. Only the caption of the current state provides the hotkey.
/// Clicking the button or pressing that hotkey toggles the state and raises
/// [`AppBarEvents::on_switchbutton_state_changed`](super::events::AppBarEvents::on_switchbutton_state_changed).
///
/// # Example
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// let switch_button = appbar::SwitchButton::new("State-1", "State-2", false, 0, appbar::Side::Left);
/// ```
pub struct SwitchButton {
    receiver_control_handle: Handle<()>,
    selected_caption: Caption,
    unselected_caption: Caption,
    tooltip: String,
    selected: bool,
    symbol: SwitchButtonSymbol,
    pub(super) base: ItemBase,
}

impl SwitchButton {
    /// Creates a switch button with no symbol and an empty tooltip.
    ///
    /// `selected` is the initial state. `order` places the button among items
    /// on `pos` (lower values come first). See [`SwitchButton`] for captions and hotkeys.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let switch_button = appbar::SwitchButton::new("State-1", "State-2", false, 0, appbar::Side::Left);
    /// ```
    pub fn new(selected_caption: &str, unselected_caption: &str, selected: bool, order: u8, pos: Side) -> Self {
        Self::with_tooltip(selected_caption, unselected_caption, SwitchButtonSymbol::None, "", selected, order, pos)
    }

    /// Creates a switch button with `symbol` and an empty tooltip.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let switch_button = appbar::SwitchButton::with_symbol(
    ///     "State-1",
    ///     "State-2",
    ///     appbar::SwitchButtonSymbol::CheckBox,
    ///     false,
    ///     0,
    ///     appbar::Side::Left,
    /// );
    /// ```
    pub fn with_symbol(selected_caption: &str, unselected_caption: &str, symbol: SwitchButtonSymbol, selected: bool, order: u8, pos: Side) -> Self {
        Self::with_tooltip(selected_caption, unselected_caption, symbol, "", selected, order, pos)
    }

    /// Creates a switch button with `symbol` and `tooltip`.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let switch_button = appbar::SwitchButton::with_tooltip(
    ///     "State-1",
    ///     "State-2",
    ///     appbar::SwitchButtonSymbol::CheckBox,
    ///     "Tooltip for switch button",
    ///     false,
    ///     0,
    ///     appbar::Side::Left,
    /// );
    /// ```
    pub fn with_tooltip(
        selected_caption: &str,
        unselected_caption: &str,
        symbol: SwitchButtonSymbol,
        tooltip: &str,
        selected: bool,
        order: u8,
        pos: Side,
    ) -> Self {
        let sc = Caption::new(selected_caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let unsc = Caption::new(unselected_caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = sc.chars_count().max(unsc.chars_count()) + symbol.width() as usize;
        Self {
            receiver_control_handle: Handle::None,
            selected_caption: sc,
            unselected_caption: unsc,
            tooltip: if tooltip.is_empty() { String::new() } else { tooltip.to_string() },
            selected,
            symbol,
            base: ItemBase::new(w as u8, order, pos, true),
        }
    }

    /// Returns `true` when the button can be activated.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    /// Enables or disables the button.
    ///
    /// A disabled button is drawn in the inactive color and does not toggle
    /// when clicked or when its hotkey is pressed.
    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    /// Returns `true` when the selected caption is showing.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Sets the selected state without raising a status-changed event.
    #[inline(always)]
    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        let attr = status.text_attribute(theme);
        let caption = if self.selected {
            &self.selected_caption
        } else {
            &self.unselected_caption
        };
        let w = self.symbol.width();
        // clear the entire space (some states might be smaller)
        surface.fill_horizontal_line_with_size(self.base.x(), 0, self.base.width() as u32, Character::with_attributes(' ', attr));
        if let Some(ch) = self.symbol.char(self.selected) {
            surface.write_char(self.base.x(), 0, Character::with_attributes(ch, attr));
        }
        let mut format = TextFormatBuilder::new()
            .position(self.base.x() + w, 0)
            .attribute(attr)
            .align(TextAlignment::Left)
            .chars_count(caption.chars_count() as u16)
            .build();
        format.set_hotkey_from_caption(status.hotkey_attribute(theme), caption);
        surface.write_text(caption.text(), &format);
    }
    pub(super) fn on_execute(&mut self) {
        self.selected = !self.selected;
        RuntimeManager::get().set_appbar_event(AppBarEvent::SwitchButtonStatusChanged(SwitchButtonStatusChangedEvent {
            button_handle: self.base.handle().cast(),
            control_receiver_handle: self.receiver_control_handle,
            state: self.selected,
        }));
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        if self.selected {
            self.selected_caption.hotkey()
        } else {
            self.unselected_caption.hotkey()
        }
    }

    /// Returns the tooltip, or an empty string when none is set.
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    /// Sets the tooltip shown when the pointer rests on the button.
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str) {
        if self.tooltip != text {
            self.tooltip.clear();
            self.tooltip.push_str(text);
            self.base.refresh();
        }
    }
}

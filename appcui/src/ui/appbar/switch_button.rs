use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar::events::AppBarEvent;
use crate::ui::appbar::events::SwitchButtonStatusChangedEvent;
use crate::utils::Caption;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SwitchButtonSymbol {
    None,
    CheckMark,
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
    pub fn new(selected_caption: &str, unselected_caption: &str, selected: bool, order: u8, pos: Side) -> Self {
        Self::with_tooltip(selected_caption, unselected_caption, SwitchButtonSymbol::None, "", selected, order, pos)
    }
    pub fn with_symbol(selected_caption: &str, unselected_caption: &str, symbol: SwitchButtonSymbol, selected: bool, order: u8, pos: Side) -> Self {
        Self::with_tooltip(selected_caption, unselected_caption, symbol, "", selected, order, pos)
    }
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

    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }
    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
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
        if let Some(ch) = self.symbol.char(self.selected) {
            // clear symbol space
            surface.fill_horizontal_line(self.base.x(), 0, self.base.x() + w, Character::with_attributes(' ', attr));
            surface.write_char(self.base.x(), 0, Character::with_attributes(ch, attr));
        }
        let mut format = TextFormatBuilder::new()
            .position(self.base.x() + w, 0)
            .attribute(attr)
            .align(TextAlignment::Left)
            .chars_count(caption.chars_count() as u16)
            .build();
        format.set_hotkey_from_caption(status.hotkey_attribute(theme), &caption);
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
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str) {
        if self.tooltip != text {
            self.tooltip.clear();
            self.tooltip.push_str(text);
            self.base.refresh();
        }
    }
}

use super::{ItemBase, ItemStatus, Side};
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar::events::AppBarEvent;
use crate::ui::appbar::events::ButtonClickEvent;
use crate::utils::Caption;

pub struct Button {
    receiver_control_handle: Handle<()>,
    caption: Caption,
    tooltip: String,
    pub(super) base: ItemBase,
}

impl Button {
    pub fn new(caption: &str, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = c.chars_count() as u8;
        Self {
            receiver_control_handle: Handle::None,
            caption: c,
            tooltip: String::new(),
            base: ItemBase::new(w, order, pos, true),
        }
    }
    pub fn with_tooltip(caption: &str, tooltip: &str, order: u8, pos: Side) -> Self {
        let c = Caption::new(caption, crate::utils::ExtractHotKeyMethod::AltPlusKey);
        let w = c.chars_count() as u8;
        Self {
            receiver_control_handle: Handle::None,
            caption: c,
            tooltip: tooltip.to_string(),
            base: ItemBase::new(w, order, pos, true),
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
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
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
    pub(super) fn on_execute(&mut self) {
        RuntimeManager::get().set_appbar_event(AppBarEvent::ButtonClick(ButtonClickEvent {
            button_handle: self.base.handle().cast(),
            control_receiver_handle: self.receiver_control_handle,
        }));
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key) -> bool {
        if self.receiver_control_handle.is_none() {
            false
        } else {
            key == self.caption.hotkey()
        }
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        self.caption.hotkey()
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

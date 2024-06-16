use super::ProcessEventResult;
use crate::graphics::*;
use crate::input::MouseEvent;
use crate::prelude::HandleSupport;
use crate::system::*;
use crate::ui::ControlBase;

#[allow(private_interfaces)]
pub(crate) trait Component {
    fn into_toolbar(self) -> ComponentToolbarItem;
}

pub(super) enum ComponentToolbarItem {
    ScrollBar(super::ScrollBar),
    SearchBar(super::SearchBar),
}

impl ComponentToolbarItem {
    #[inline(always)]
    pub(super) fn is_vertical(&self) -> bool {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.is_vertical(),
            ComponentToolbarItem::SearchBar(_) => false,
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.paint(surface, theme, control),
            ComponentToolbarItem::SearchBar(item) => item.paint(surface, theme, control),
        }
    }
    #[inline(always)]
    pub(super) fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.on_mouse_event(event),
            ComponentToolbarItem::SearchBar(item) => item.on_mouse_event(event),
        }
    }
    #[inline(always)]
    pub(super) fn recompute_pos(&mut self, pos: i32, available_size: i32, control_size: Size) -> i32 {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.recompute_position(pos, available_size, control_size),
            ComponentToolbarItem::SearchBar(item) => item.recompute_position(pos, available_size, control_size),
        }
    }
}

impl HandleSupport<ComponentToolbarItem> for ComponentToolbarItem {
    fn handle(&self) -> Handle<ComponentToolbarItem> {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.handle.cast(),
            ComponentToolbarItem::SearchBar(item) => item.handle.cast(),
        }
    }

    fn set_handle(&mut self, handle: Handle<ComponentToolbarItem>) {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.handle = handle.cast(),
            ComponentToolbarItem::SearchBar(item) => item.handle = handle.cast(),
        }
    }
}

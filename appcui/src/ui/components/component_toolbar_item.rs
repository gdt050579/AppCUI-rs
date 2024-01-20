use crate::graphics::Surface;
use crate::input::MouseEvent;
use crate::prelude::HandleSupport;
use crate::system::*;
use crate::ui::ControlBase;
use super::ProcessEventResult;

pub trait Component {}

pub(super) enum ComponentToolbarItem {
    ScrollBar(super::ScrollBar),
}

impl ComponentToolbarItem {
    #[inline(always)]
    pub(super) fn is_vertical(&self) -> bool {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.is_vertical(),
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.paint(surface, theme, control),
        }
    }
    #[inline(always)]
    pub(super) fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.on_mouse_event(event),
        }
    }
}

impl HandleSupport<ComponentToolbarItem> for ComponentToolbarItem {
    fn get_handle(&self) -> Handle<ComponentToolbarItem> {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.handle.cast(),
        }
    }

    fn set_handle(&mut self, handle: Handle<ComponentToolbarItem>) {
        match self {
            ComponentToolbarItem::ScrollBar(item) => item.handle = handle.cast(),
        }
    }
}

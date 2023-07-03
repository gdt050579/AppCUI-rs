use crate::{system::{Handle, HandleSupport, Theme}, graphics::Surface};

use super::{position::Position, Label, PaintData};

pub(super) enum ToolBarItem {
    Label(Label),
}
impl ToolBarItem {
    pub(super) fn get_position(&self) -> &Position {
        match self {
            ToolBarItem::Label(item) => &item.position,
        }
    }
    pub(super) fn get_position_mut(&mut self) -> &mut Position {
        match self {
            ToolBarItem::Label(item) => &mut item.position,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {}
}
impl HandleSupport for ToolBarItem {
    fn get_handle(&self) -> Handle {
        match self {
            ToolBarItem::Label(item) => item.handle,
        }
    }

    fn set_handle(&mut self, handle: Handle) {
        match self {
            ToolBarItem::Label(item) => item.handle = handle,
        }
    }
}

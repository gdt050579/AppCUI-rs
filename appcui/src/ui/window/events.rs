use crate::{system::Handle, ui::common::traits::EventProcessStatus, graphics::Rect};
use super::toolbar::Button;

// Window events always go to the same window that triggers them --> we don't need a handle as
// we already have &mut self
pub trait WindowEvents {
    fn on_activate(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_close(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {
        
    }
}
pub trait ToolbarEvents {
    fn on_button_clicked(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum EventData {
    OnActivate,
    OnClose,
}

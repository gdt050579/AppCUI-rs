use super::toolbar::{Button,CheckBox};
use crate::{graphics::Rect, system::Handle, ui::common::traits::EventProcessStatus};

// Window events always go to the same window that triggers them --> we don't need a handle as
// we already have &mut self
pub trait WindowEvents {
    fn on_close(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    // don't need to change anything --> since layout has been change, repaint wil be force automatically
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {}
    fn on_activate(&mut self) {}
    fn on_deactivate(&mut self) {}
}
pub trait ToolBarEvents {
    fn on_button_clicked(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_checkbox_clicked(&mut self, _handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum EventData {
    OnActivate,
    OnClose,
}

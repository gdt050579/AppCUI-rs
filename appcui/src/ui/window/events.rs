use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait WindowEvents {
    fn on_activate(&mut self, _window_handle: Handle) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_close(&mut self, _window_handle: Handle) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait ToolbarEvents {
    fn on_button_clicked(&mut self, _item_handle: Handle) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum EventData {
    OnActivate,
    OnClose,
}

use crate::{system::Handle, ui::common::{traits::EventProcessStatus, UIElement}};

pub trait WindowEvents {
    fn on_activate(&mut self, _window_handle: Handle<UIElement>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_close(&mut self, _window_handle: Handle<UIElement>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait ToolbarEvents {
    fn on_button_clicked(&mut self, _item_handle: Handle<UIElement>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum EventData {
    OnActivate,
    OnClose,
}

use crate::{system::Handle, ui::common::{traits::EventProcessStatus, UIElement}};

pub trait ButtonEvents {
    fn on_pressed(&mut self, _button_handle: Handle<UIElement>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;

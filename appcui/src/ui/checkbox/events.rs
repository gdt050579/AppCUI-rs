use crate::{system::Handle, ui::common::{traits::EventProcessStatus, UIElement}};

pub trait CheckBoxEvents {
    fn on_status_changed(&mut self, _checbox_handle: Handle<UIElement>, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    pub (crate) checked: bool
}

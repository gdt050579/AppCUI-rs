use super::ListBox;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ListBoxEvents {
    fn on_something(&mut self, _handle: Handle<ListBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
}
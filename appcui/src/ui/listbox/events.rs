use super::ListBox;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ListBoxEvents {
    fn on_current_item_changed(&mut self, _handle: Handle<ListBox>, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_checked(&mut self, _handle: Handle<ListBox>, _index: usize, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) enum ListBoxEventTypes {
    CurrentItemChanged,
    ItemChecked,
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: ListBoxEventTypes,
    pub(crate) index: usize,
    pub(crate) checked: bool,
}
use super::ListBox;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::ListBox`].
///
/// Implement this on a window to react when the current item or a check box changes.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait ListBoxEvents {
    /// Called when the highlighted item changes to `index`.
    fn on_current_item_changed(&mut self, _handle: Handle<ListBox>, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the check box on the item at `index` is toggled to `checked`.
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
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::ComboBox;

/// Events from a [`struct@super::ComboBox`].
///
/// Implement this on a window to react when the selected item changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait ComboBoxEvents {
    /// Called when a different item is selected.
    fn on_selection_changed(&mut self, _handle: Handle<ComboBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {}

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::CheckBox;

/// Events from a [`struct@super::CheckBox`].
///
/// Implement this on a window to react when the checked state changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait CheckBoxEvents {
    /// Called when the check box is toggled. `checked` is the new state.
    fn on_status_changed(&mut self, _handle: Handle<CheckBox>, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    pub (crate) checked: bool
}

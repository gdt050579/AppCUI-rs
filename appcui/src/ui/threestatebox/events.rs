use super::{ThreeStateBox, State};
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::ThreeStateBox`].
///
/// Implement this on a window to react when the tri-state value changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait ThreeStateBoxEvents {
    /// Called when the state changes to `state` (checked, unchecked, or unknown).
    fn on_status_changed(&mut self, _handle: Handle<ThreeStateBox>, _state: State) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) state: State,
}

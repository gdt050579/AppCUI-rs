use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::ToggleButton;

/// Events from a [`struct@super::ToggleButton`].
///
/// Implement this on a window to react when the pressed/selected state changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait ToggleButtonEvents {
    /// Called when the toggle button is toggled. `selected` is the new pressed state.
    fn on_selection_changed(&mut self, _handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) status: bool,
}
//! Events emitted by a [`struct@super::RadioBox`].
//!
//! Implement [`RadioBoxEvents`] on a window (or other parent) to react when this
//! radio box becomes selected.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::RadioBox;

/// Events from a [`struct@super::RadioBox`].
///
/// Implement this on a window to react when this radio box becomes selected.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait RadioBoxEvents {
    /// Called when this radio box is selected (others in the group are cleared).
    fn on_selected(&mut self, _handle: Handle<RadioBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {}

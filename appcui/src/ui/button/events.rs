//! Events emitted by a [`struct@super::Button`].
//!
//! Implement [`ButtonEvents`] on a window (or other parent) to react when the
//! button is pressed.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Button;

/// Events from a [`struct@super::Button`].
///
/// Implement this on a window to react when the button is pressed.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait ButtonEvents {
    /// Called when the button is pressed (mouse click or keyboard activate).
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;

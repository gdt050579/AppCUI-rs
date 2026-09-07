use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::PathFinder;

/// Events from a [`struct@super::PathFinder`].
///
/// Implement this on a window to react when the path text changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait PathFinderEvents {
    /// Called when the current path is updated (typed or navigated).
    fn on_path_updated(&mut self, _handle: Handle<PathFinder>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;


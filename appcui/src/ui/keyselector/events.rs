use super::KeySelector;
use crate::{input::Key, system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::KeySelector`].
///
/// Implement this on a window to react when the recorded key combination changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait KeySelectorEvents {
    /// Called when the key changes from `old_key` to `new_key`.
    fn on_key_changed(&mut self, _handle: Handle<KeySelector>, _new_key: Key, _old_key: Key) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_key: Key,
    pub(crate) old_key: Key,
}

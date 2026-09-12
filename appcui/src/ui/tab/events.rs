//! Events emitted by a [`struct@super::Tab`].
//!
//! Implement [`TabEvents`] on a window (or other parent) to react when the
//! selected page changes.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Tab;

/// Events from a [`struct@super::Tab`].
///
/// Implement this on a window to react when the selected page changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait TabEvents {
    /// Called when the selected page changes from `old_tabl_index` to `new_tab_index`.
    fn on_tab_changed(&mut self, _handle: Handle<Tab>, _new_tab_index: u32, _old_tabl_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_tab_index: u32,
    pub(crate) old_tab_index: u32,
}

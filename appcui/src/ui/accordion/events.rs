use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Accordion;

/// Events from an [`struct@super::Accordion`].
///
/// Implement this on a window to react when the expanded panel changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait AccordionEvents {
    /// Called when the expanded panel changes from `old_panel_index` to `new_panel_index`.
    fn on_panel_changed(&mut self, _handle: Handle<Accordion>, _new_panel_index: u32, _old_panel_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_panel_index: u32,
    pub(crate) old_panel_index: u32,
}

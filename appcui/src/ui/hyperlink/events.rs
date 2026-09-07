use super::HyperLink;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::HyperLink`].
///
/// Implement this on a window to react when the user activates the link.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait HyperLinkEvents {
    /// Called when the hyperlink is opened (clicked or activated).
    fn on_open(&mut self, _handle: Handle<HyperLink>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;

//! Events emitted by a [`struct@crate::ui::Desktop`].
//!
//! Implement [`DesktopEvents`] on your desktop type to run startup logic,
//! intercept close, and observe how many windows are open.

use crate::ui::common::traits::*;
/// Events from a [`struct@crate::ui::Desktop`].
///
/// Implement this on your desktop type to run startup logic, intercept close,
/// and observe how many windows are open.
pub trait DesktopEvents {
    /// Called once when the desktop starts (application launch).
    fn on_start(&mut self) { }
    /// Called when the user tries to close the desktop. Return [`ActionRequest::Deny`] to cancel.
    fn on_close(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
    /// Called whenever the number of open windows changes to `count`.
    fn on_update_window_count(&mut self, _count: usize) {

    }
}
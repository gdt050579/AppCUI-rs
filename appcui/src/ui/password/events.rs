//! Events emitted by a [`struct@super::Password`].
//!
//! Implement [`PasswordEvents`] on a window (or other parent) to react when the
//! user confirms or cancels the field.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Password;

/// Events from a [`struct@super::Password`].
///
/// Implement this on a window to react when the user confirms or cancels the field.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait PasswordEvents {
    /// Called when the user confirms the password (typically Enter).
    fn on_accept(&mut self, _handle: Handle<Password>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user cancels (typically Escape).
    fn on_cancel(&mut self, _handle: Handle<Password>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) accept: bool
}

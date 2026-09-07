//! Events emitted by a [`struct@super::CommandBar`].
//!
//! Implement [`GenericCommandBarEvents`] on a window or desktop to populate the
//! command bar and handle activated commands.

use crate::system::Handle;
use super::CommandBar;

#[derive(Copy,Clone)]
pub(crate) struct CommandBarEvent {
    pub (crate) command_id: u32,
    pub (crate) control_receiver_handle: Handle<()> ,
}
/// Events from a [`struct@super::CommandBar`].
///
/// Implement this on a window or desktop to populate the command bar and handle
/// commands. Default methods do nothing.
pub trait GenericCommandBarEvents {
    /// Called when the command bar should be rebuilt. Add commands with `commandbar.set(...)`.
    fn on_update_commandbar(&self, _commandbar: &mut CommandBar) {}
    /// Called when the user activates the command with identifier `command_id`.
    fn on_event(&mut self, _command_id: u32) {}
}

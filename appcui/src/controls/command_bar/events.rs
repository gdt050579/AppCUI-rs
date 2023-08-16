use crate::system::Handle;
use super::CommandBar;

pub(crate) struct CommandBarEvent {
    command_id: u32,
    control_receiver_handle: Handle,
}
pub trait CommandBarEvents {
    fn on_update_commandbar(&self, _commandbar: &mut CommandBar) {}
    fn on_event(&mut self, _command_id: u32) {}
}

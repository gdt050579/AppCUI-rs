use super::CommandBar;
use crate::system::Handle;

#[derive(Copy, Clone)]
pub(crate) struct CommandBarEvent {
    pub(crate) command_id: u32,
    pub(crate) control_receiver_handle: Handle<()>,
}
pub trait GenericCommandBarEvents {
    fn on_update_commandbar(&self, _commandbar: &mut CommandBar) {}
    fn on_event(&mut self, _command_id: u32) {}
}

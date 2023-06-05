use crate::system::Handle;


#[derive(Copy, Clone)]
pub(crate) struct CommandBarEvent {
    pub(crate) command_id: u32,
    pub(crate) control_receiver_handle: Handle,
}
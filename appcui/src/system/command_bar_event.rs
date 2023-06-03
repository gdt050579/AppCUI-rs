use super::Handle;

#[derive(Copy, Clone)]
pub(super) struct CommandBarEvent {
    pub(super) command_id: u32,
    pub(super) control_receiver_handle: Handle,
}
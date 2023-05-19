use crate::system::Handle;

#[derive(Copy,Clone)]
pub struct CheckedStatusChangedEvent
{
    pub handle: Handle,
    pub checked: bool,
}
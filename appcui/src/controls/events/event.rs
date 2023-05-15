use crate::system::Handle;

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum Event {
    CheckedStatusChanged,
    WindowClose(Handle),
    ButtonClicked(Handle),
    Command(u32),
}
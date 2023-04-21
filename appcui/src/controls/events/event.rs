#[repr(u8)]
#[derive(Copy,Clone)]
pub enum Event {
    CheckedStatusChanged,
    WindowClose,
}
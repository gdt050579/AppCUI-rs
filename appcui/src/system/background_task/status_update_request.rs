#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum StatusUpdateRequest {
    None,
    Pause,
    Close
}
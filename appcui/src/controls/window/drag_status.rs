#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) enum DragStatus {
    None,
    Move,
    Resize,
}

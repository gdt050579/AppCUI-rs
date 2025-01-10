#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub(super) enum DragStatus {
    #[default]
    None,
    Move,
    Resize,
}

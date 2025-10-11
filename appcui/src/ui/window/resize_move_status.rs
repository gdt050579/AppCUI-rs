#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub(super) enum ResizeMoveStatus {
    #[default]
    None,
    MoveByMouse,
    ResizeByMouse,
    ResizeMoveViaKeyboard, 
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(super) enum MenuButtonState {
    Normal,
    Hovered,
    Pressed,
}

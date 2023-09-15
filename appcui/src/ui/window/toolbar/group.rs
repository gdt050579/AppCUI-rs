#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GroupPosition {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[derive(Clone, Copy)]
pub struct Group {
    pub(super) pos: GroupPosition,
    pub(super) id: u8,
}
impl Default for Group {
    fn default() -> Self {
        Self {
            pos: GroupPosition::TopLeft,
            id: 255,
        }
    }
}

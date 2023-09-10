#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GroupPosition {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

impl GroupPosition {
    pub (crate) fn is_on_left_side(&self)->bool {
        match self {
            GroupPosition::TopLeft => true,
            GroupPosition::BottomLeft => true,
            GroupPosition::TopRight => false,
            GroupPosition::BottomRight => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Group {
    pub(super) pos: GroupPosition,
    pub(super) id: u8
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Gravity {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

impl Gravity {
    pub (crate) fn is_on_left_side(&self)->bool {
        match self {
            Gravity::TopLeft => true,
            Gravity::BottomLeft => true,
            Gravity::TopRight => false,
            Gravity::BottomRight => false,
        }
    }
}
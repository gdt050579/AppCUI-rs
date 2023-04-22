use super::ControlLayout;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct AbsoluteLayout {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}
impl AbsoluteLayout {
    #[inline]
    pub(super) fn new(x: i32, y: i32, width: u16, height: u16) -> Self {
        AbsoluteLayout {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout) {
        control_layout.resize(self.width, self.height);
        control_layout.set_position(self.x, self.y);
    }
}

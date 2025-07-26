use super::should_not_use;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Error;
use super::Layout;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftTopRightAnchorsLayout {
    pub left: Coordinate16,
    pub top: Coordinate16,
    pub right: Coordinate16,
    pub height: Dimension16,
}

impl LeftTopRightAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::LeftTopRightAnchorsUsedWithXY);
        should_not_use!(params.y, Error::LeftTopRightAnchorsUsedWithXY);
        should_not_use!(params.width, Error::LeftTopRightAnchorsUsedWithWidth);
        should_not_use!(params.pivot, Error::LeftTopRightAnchorsUsedWithPivot);

        Ok(LeftTopRightAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
        })
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        let left = self.left.absolute(parent_width);
        let right = self.right.absolute(parent_width);
        let top = self.top.absolute(parent_height);
        control_layout.resize(
            ((parent_width as i32) - (left + right)).clamp(1, 0xFFFF) as u16,
            self.height.absolute(parent_height),
        );
        control_layout.set_position(left, top);
    }
}

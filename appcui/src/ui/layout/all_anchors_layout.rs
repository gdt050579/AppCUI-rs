use super::should_not_use;
use super::ControlLayout;
use super::Coordinate16;
use super::Layout;
use super::Error;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct AllAnchorsLayout {
    pub left: Coordinate16,
    pub top: Coordinate16,
    pub right: Coordinate16,
    pub bottom: Coordinate16,
}

impl AllAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::AllAnchorsParameterUsedWithXY);
        should_not_use!(params.y, Error::AllAnchorsParameterUsedWithXY);
        should_not_use!(params.height, Error::AllAnchorsParameterUsedWithSize);
        should_not_use!(params.width, Error::AllAnchorsParameterUsedWithSize);
        should_not_use!(params.pivot, Error::AllAnchorsParameterUsedWithPivot);

        Ok(AllAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
        })
    }
    #[inline]
    pub(super) fn update_control_layout(
        &self,
        control_layout: &mut ControlLayout,
        parent_width: u16,
        parent_height: u16,
    ) {
        let left = self.left.absolute(parent_width);
        let top = self.top.absolute(parent_height);
        let right = self.right.absolute(parent_width);
        let bottom = self.bottom.absolute(parent_height);
        control_layout.resize(
            ((parent_width as i32) - (left + right)).clamp(1, 0xFFFF) as u16,
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        control_layout.set_position(left, top);
    }
}

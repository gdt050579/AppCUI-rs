use super::should_not_use;
use super::ControlLayout;
use super::Coordonate16;
use super::LayoutParameters;
use super::Dimension;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftTopRightAnchorsLayout {
    pub left: Coordonate16,
    pub top: Coordonate16,
    pub right: Coordonate16,
    pub height: Dimension,
}

impl LeftTopRightAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When (left,top,right) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (left,top,right) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.width,
            "When (left,top,right) parameters are used together, 'width' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (left,top,right) parameters are used together, 'align' parameter can not be used"
        );

        LeftTopRightAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            height: params.height.unwrap_or(Dimension::Absolute(1)),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(
        &self,
        control_layout: &mut ControlLayout,
        parent_width: u16,
        parent_height: u16,
    ) {
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

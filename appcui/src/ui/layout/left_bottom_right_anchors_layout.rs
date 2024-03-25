use super::should_not_use;
use super::ControlLayout;
use super::Coordonate;
use super::LayoutParameters;
use super::Size;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftBottomRightAnchorsLayout {
    pub left: Coordonate,
    pub bottom: Coordonate,
    pub right: Coordonate,
    pub height: Size,
}

impl LeftBottomRightAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When (left,bottom,right) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (left,bottom,right) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.width,
            "When (left,bottom,right) parameters are used together, 'width' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (left,bottom,right) parameters are used together, 'align' parameter can not be used"
        );

        LeftBottomRightAnchorsLayout {
            left: params.a_left.unwrap(),
            bottom: params.a_bottom.unwrap(),
            right: params.a_right.unwrap(),
            height: params.height.unwrap_or(Size::Absolute(1)),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(
        &self,
        control_layout: &mut ControlLayout,
        parent_width: u16,
        parent_height: u16,
    ) {
        let left = self.left.as_absolute_coordonate(parent_width);
        let right = self.right.as_absolute_coordonate(parent_width);
        let bottom = self.bottom.as_absolute_coordonate(parent_height);
        control_layout.resize(
            ((parent_width as i32) - (left + right)).clamp(1, 0xFFFF) as u16,
            self.height.as_absolute_size(parent_height),
        );
        control_layout.set_position(
            left,
            (parent_height as i32) - (bottom + (control_layout.get_height() as i32)),
        );
    }
}

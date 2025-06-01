use super::should_not_use;
use super::ControlLayout;
use super::Coordonate16;
use super::LayoutParameters;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct AllAnchorsLayout {
    pub left: Coordonate16,
    pub top: Coordonate16,
    pub right: Coordonate16,
    pub bottom: Coordonate16,
}

impl AllAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When (left,top,right,bottom) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (left,top,right,bottom) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.height,
            "When (left,top,right,bottom) parameters are used together, 'height' parameter can not be used"
        );
        should_not_use!(
            params.width,
            "When (left,top,right,bottom) parameters are used together, 'widyj' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (left,top,right,bottom) parameters are used together, 'align' parameter can not be used"
        );

        AllAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
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

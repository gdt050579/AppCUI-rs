use super::should_not_use;
use super::ControlLayout;
use super::Coordonate;
use super::LayoutParameters;
use super::Dimension;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopRightBottomAnchorsLayout {
    pub top: Coordonate,
    pub right: Coordonate,
    pub bottom: Coordonate,
    pub width: Dimension,
}

impl TopRightBottomAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When (top,right,bottom) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (top,right,bottom) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.height,
            "When (top,right,bottom) parameters are used together, 'height' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (top,right,bottom) parameters are used together, 'align' parameter can not be used"
        );

        TopRightBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
            width: params.width.unwrap_or(Dimension::Absolute(1)),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(
        &self,
        control_layout: &mut ControlLayout,
        parent_width: u16,
        parent_height: u16,
    ) {
        let top = self.top.absolute(parent_height);
        let right = self.right.absolute(parent_width);
        let bottom = self.bottom.absolute(parent_height);
        control_layout.resize(
            self.width.absolute(parent_width),
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        control_layout.set_position(
            (parent_width as i32) - (right + (control_layout.get_width() as i32)),
            top,
        );
    }
}

use super::should_not_use;
use super::ControlLayout;
use super::Coordonate;
use super::LayoutParameters;
use super::Size;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopLeftBottomAnchorsLayout {
    pub top: Coordonate,
    pub left: Coordonate,
    pub bottom: Coordonate,
    pub width: Size,
}

impl TopLeftBottomAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When (top,left,bottom) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (top,left,bottom) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.height,
            "When (top,left,bottom) parameters are used together, 'height' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (top,left,bottom) parameters are used together, 'align' parameter can not be used"
        );

        TopLeftBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            left: params.a_left.unwrap(),
            bottom: params.a_bottom.unwrap(),
            width: params.width.unwrap_or(Size::Absolute(1)),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(
        &self,
        control_layout: &mut ControlLayout,
        parent_width: u16,
        parent_height: u16,
    ) {
        let top = self.top.to_absolute_coordonate(parent_height);
        let left = self.left.to_absolute_coordonate(parent_width);
        let bottom = self.bottom.to_absolute_coordonate(parent_height);
        control_layout.resize(
            self.width.to_absolute_size(parent_width),
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        control_layout.set_position(left, top);

    }
}

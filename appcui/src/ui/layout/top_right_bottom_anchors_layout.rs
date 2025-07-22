use super::should_not_use;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Error;
use super::Layout;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopRightBottomAnchorsLayout {
    pub top: Coordinate16,
    pub right: Coordinate16,
    pub bottom: Coordinate16,
    pub width: Dimension16,
}

impl TopRightBottomAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::TopRightBottomAnchorsUsedWithXY);
        should_not_use!(params.y, Error::TopRightBottomAnchorsUsedWithXY);
        should_not_use!(params.height, Error::TopRightBottomAnchorsUsedWithHeight);
        should_not_use!(params.pivot, Error::TopRightBottomAnchorsUsedWithPivot);

        Ok(TopRightBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
        })
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        let top = self.top.absolute(parent_height);
        let right = self.right.absolute(parent_width);
        let bottom = self.bottom.absolute(parent_height);
        control_layout.resize(
            self.width.absolute(parent_width),
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        control_layout.set_position((parent_width as i32) - (right + (control_layout.get_width() as i32)), top);
    }
}

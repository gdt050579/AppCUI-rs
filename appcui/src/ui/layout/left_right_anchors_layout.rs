use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Error;
use super::Layout;
use super::Pivot;
use super::{should_not_use, should_use};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftRightAnchorsLayout {
    pub left: Coordinate16,
    pub right: Coordinate16,
    pub y: Coordinate16,
    pub height: Dimension16,
    pub pivot: Pivot,
}

impl LeftRightAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::LeftRightAnchorsUsedWithX);
        should_not_use!(params.width, Error::LeftRightAnchorsUsedWithWidth);
        should_use!(params.pivot, Error::LeftRightAnchorsUsedWithoutPivot);
        should_use!(params.y, Error::LeftRightAnchorsUsedWithoutY);

        Ok(LeftRightAnchorsLayout {
            left: params.a_left.unwrap(),
            right: params.a_right.unwrap(),
            y: params.y.unwrap(),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
            pivot: params.pivot.unwrap(),
        })
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        let left = self.left.absolute(parent_width);
        let right = self.right.absolute(parent_width);
        let y = self.y.absolute(parent_height);
        control_layout.resize(
            ((parent_width as i32) - (left + right)).clamp(0, 0xFFFF) as u16,
            self.height.absolute(parent_height),
        );
        let new_h = control_layout.get_height() as i32;
        let new_w = control_layout.get_width() as i32;
        let l = left;
        let r = (parent_width as i32).saturating_sub(right);
        let (new_x, new_y) = match self.pivot {
            Pivot::TopLeft => (l, y),
            Pivot::TopRight => (r - new_w, y),
            Pivot::TopCenter => ((l + r - new_w) / 2, y),
            Pivot::BottomLeft => (l, y - new_h),
            Pivot::BottomRight => (r - new_w, y - new_h),
            Pivot::BottomCenter => ((l + r - new_w) / 2, y - new_h),
            Pivot::CenterLeft => (l, y - new_h / 2),
            Pivot::CenterRight => (r - new_w, y - new_h / 2),
            Pivot::Center => ((l + r - new_w) / 2, y - new_h / 2),
        };
        control_layout.set_position(new_x, new_y);

        // match self.pivot {
        //     Pivot::TopCenter => control_layout.set_position(left, y),
        //     Pivot::BottomCenter => control_layout.set_position(left, y - (control_layout.get_height() as i32)),
        //     Pivot::Center => control_layout.set_position(left, y - ((control_layout.get_height() / 2) as i32)),
        //     _ => unreachable!("This code should not be reached --> internal error"),
        // }
    }
}

use super::{should_not_use, should_use};
use super::Pivot;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Layout;
use super::Error;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopBottomAnchorsLayout {
    pub top: Coordinate16,
    pub bottom: Coordinate16,
    pub x: Coordinate16,
    pub width: Dimension16,
    pub pivot: Pivot,
}

impl TopBottomAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.y, Error::TopBottomAnchorsUsedWithY);
        should_not_use!(params.height, Error::TopBottomAnchorsUsedWithHeight);
        should_use!(params.x, Error::TopBottomAnchorsUsedWithoutX);
        should_use!(params.pivot, Error::TopBottomAnchorsUsedWithoutPivot);

        Ok(TopBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            bottom: params.a_bottom.unwrap(),
            x: params.x.unwrap(),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            pivot: params.pivot.unwrap(),
        })
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        let top = self.top.absolute(parent_height);
        let bottom = self.bottom.absolute(parent_height);
        let x = self.x.absolute(parent_width);
        control_layout.resize(
            self.width.absolute(parent_width),
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        let new_h = control_layout.height() as i32;
        let new_w = control_layout.width() as i32;
        let t = top;
        let b = (parent_height as i32).saturating_sub(bottom);

        let (new_x, new_y) = match self.pivot {
            Pivot::TopLeft => (x, t),
            Pivot::CenterLeft => (x, (t + b - new_h) / 2),
            Pivot::BottomLeft => (x, b - new_h),

            Pivot::TopRight => (x - new_w, t),
            Pivot::CenterRight => (x - new_w, (t + b - new_h) / 2),
            Pivot::BottomRight => (x - new_w, t - new_h),

            Pivot::TopCenter => (x- new_w / 2, t),
            Pivot::BottomCenter => (x- new_w / 2, b - new_h),
            Pivot::Center => (x- new_w / 2, (t + b - new_h) / 2),
        };
        control_layout.set_position(new_x, new_y);

    }
}

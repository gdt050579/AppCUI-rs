use super::should_not_use;
use super::Pivot;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Layout;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopBottomAnchorsLayout {
    pub top: Coordinate16,
    pub bottom: Coordinate16,
    pub x: Coordinate16,
    pub width: Dimension16,
    pub pivot: Pivot,
}

impl TopBottomAnchorsLayout {
    pub(super) fn new(params: &Layout) -> Self {
        should_not_use!(params.y, "When (top,bottom) parameters are used together, 'Y' parameter can not be used");
        should_not_use!(params.height,"When (top,bottom) parameters are used toghere, ('height' or 'h') parameters can not be used as the width is deduced from bottom-top difference");

        if let Some(pivot) = params.pivot {
            match pivot {
                Pivot::CenterLeft | Pivot::Center | Pivot::CenterRight => {}
                _ => panic!("When (top,bottom) are provided, only Left(l), Center(c) and Right(r) pivot values are allowed !"),
            }
        }

        TopBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            bottom: params.a_bottom.unwrap(),
            x: params.x.unwrap_or(Coordinate16::Absolute(0)),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            pivot: params.pivot.unwrap_or(Pivot::Center),
        }
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
        match self.pivot {
            Pivot::CenterLeft => control_layout.set_position(x, top),
            Pivot::CenterRight => control_layout.set_position(x - (control_layout.get_width() as i32), top),
            Pivot::Center => control_layout.set_position(x - ((control_layout.get_width() / 2) as i32), top),
            _ => unreachable!("This code should not be reached --> internal error"),
        }
    }
}

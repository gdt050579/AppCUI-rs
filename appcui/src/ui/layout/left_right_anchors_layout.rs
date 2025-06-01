use super::should_not_use;
use super::Alignament;
use super::ControlLayout;
use super::Coordonate16;
use super::Dimension16;
use super::LayoutParameters;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftRightAnchorsLayout {
    pub left: Coordonate16,
    pub right: Coordonate16,
    pub y: Coordonate16,
    pub height: Dimension16,
    pub align: Alignament,
}

impl LeftRightAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(params.x, "When (left,right) parameters are used together, 'X' parameter can not be used");
        should_not_use!(params.width,"When (left,right) parameters are used toghere, ('width' or 'w') parameters can not be used as the width is deduced from left-right difference");

        if let Some(align) = params.align {
            match align {
                Alignament::Top | Alignament::Center | Alignament::Bottom => {}
                _ => panic!("When (left,right) are provided, only Top(t), Center(c) and Bottom(b) alignament values are allowed !"),
            }
        }

        LeftRightAnchorsLayout {
            left: params.a_left.unwrap(),
            right: params.a_right.unwrap(),
            y: params.y.unwrap_or(Coordonate16::Absolute(0)),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
            align: params.align.unwrap_or(Alignament::Center),
        }
    }
    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        let left = self.left.absolute(parent_width);
        let right = self.right.absolute(parent_width);
        let y = self.y.absolute(parent_height);
        control_layout.resize(
            ((parent_width as i32) - (left + right)).clamp(1, 0xFFFF) as u16,
            self.height.absolute(parent_height),
        );
        match self.align {
            Alignament::Top => control_layout.set_position(left, y),
            Alignament::Bottom => control_layout.set_position(left, y - (control_layout.get_height() as i32)),
            Alignament::Center => control_layout.set_position(left, y - ((control_layout.get_height() / 2) as i32)),
            _ => unreachable!("This code should not be reached --> internal error"),
        }
    }
}

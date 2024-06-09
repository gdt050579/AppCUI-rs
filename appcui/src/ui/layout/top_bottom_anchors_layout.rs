use super::should_not_use;
use super::Alignament;
use super::ControlLayout;
use super::Coordonate16;
use super::LayoutParameters;
use super::Dimension16;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopBottomAnchorsLayout {
    pub top: Coordonate16,
    pub bottom: Coordonate16,
    pub x: Coordonate16,
    pub width: Dimension16,
    pub align: Alignament,
}

impl TopBottomAnchorsLayout {
    pub(super) fn new(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.y,
            "When (top,bottom) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(params.height,"When (top,bottom) parameters are used toghere, ('height' or 'h') parameters can not be used as the width is deduced from bottom-top difference");

        if let Some(align) = params.align {
            match align {
                Alignament::Left|Alignament::Center|Alignament::Right => {},
                _ => panic!("When (top,bottom) are provided, only Left(l), Center(c) and Right(r) alignament values are allowed !")
            }
        }

        TopBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            bottom: params.a_bottom.unwrap(),
            x: params.x.unwrap_or(Coordonate16::Absolute(0)),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            align: params.align.unwrap_or(Alignament::Center),
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
        let bottom = self.bottom.absolute(parent_height);
        let x = self.x.absolute(parent_width);
        control_layout.resize(
            self.width.absolute(parent_width),
            ((parent_height as i32) - (top + bottom)).clamp(1, 0xFFFF) as u16,
        );
        match self.align {
            Alignament::Left => control_layout.set_position(x, top),
            Alignament::Right => {
                control_layout.set_position(x - (control_layout.get_width() as i32), top)
            }
            Alignament::Center => {
                control_layout.set_position(x - ((control_layout.get_width() / 2) as i32), top)
            }
            _ => unreachable!("This code should not be reached --> internal error"),
        }
    }
}

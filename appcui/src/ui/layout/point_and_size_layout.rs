use super::should_not_use;
use super::Alignment;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Dock;
use super::LayoutParameters;
use super::Pivot;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct PointAndSizeLayout {
    pub x: Coordinate16,
    pub y: Coordinate16,
    pub width: Dimension16,
    pub height: Dimension16,
    pub align: Alignment,
    pub anchor: Alignment,
}
impl PointAndSizeLayout {
    #[inline]
    pub(super) fn new_docked(params: &LayoutParameters) -> Self {
        should_not_use!(params.x, "When ('dock' or 'd') parameter is used,'x' parameter can not be used !");
        should_not_use!(params.y, "When ('dock' or 'd') parameter is used,'y' parameter can not be used !");
        should_not_use!(
            params.a_top,
            "When ('dock' or 'd') parameter is used,('top' or 't') parameters can not be used !"
        );
        should_not_use!(
            params.a_bottom,
            "When ('dock' or 'd') parameter is used,('bottom' or 'b') parameters can not be used !"
        );
        should_not_use!(
            params.a_left,
            "When ('dock' or 'd') parameter is used,('left' or 'l') parameters can not be used !"
        );
        should_not_use!(
            params.a_right,
            "When ('dock' or 'd') parameter is used,('right' or 'r') parameters can not be used !"
        );
        should_not_use!(
            params.pivot,
            "When ('dock' or 'd') parameter is used,('align' or 'a') parameters can not be used !"
        );

        match params.dock.unwrap() {
            Dock::Left => {
                should_not_use!(
                    params.height,
                    "When ('dock' or 'd') parameter is used with the value 'Left', the 'height' parameter can not be used !"
                );
                PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Absolute(0),
                    width: params.width.unwrap_or(Dimension16::Percentage(10000)),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopLeft,
                    anchor: Alignment::TopLeft,
                }
            }
            Dock::Right => {
                should_not_use!(
                    params.height,
                    "When ('dock' or 'd') parameter is used with the value 'Right', the 'height' parameter can not be used !"
                );
                PointAndSizeLayout {
                    x: Coordinate16::Percentage(10000),
                    y: Coordinate16::Absolute(0),
                    width: params.width.unwrap_or(Dimension16::Percentage(10000)),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopRight,
                    anchor: Alignment::TopRight,
                }
            },
            Dock::Top => todo!(),
            Dock::Bottom => todo!(),
            Dock::Fill => {
                should_not_use!(
                    params.height,
                    "When ('dock' or 'd') parameter is used with the value 'Fill', the 'height' parameter can not be used !"
                );
                should_not_use!(
                    params.height,
                    "When ('dock' or 'd') parameter is used with the value 'Fill', the 'width' parameter can not be used !"
                );
                PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Absolute(0),
                    width: Dimension16::Percentage(10000),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopLeft,
                    anchor: Alignment::TopLeft,
                }
            }
        }
    }

    pub(super) fn new_aligned(params: &LayoutParameters) -> Self {
        should_not_use!(params.x, "When ('align' or 'a') parameter is used,'x' parameter can not be used !");
        should_not_use!(params.y, "When ('align' or 'a') parameter is used,'y' parameter can not be used !");
        should_not_use!(
            params.a_top,
            "When ('align' or 'a') parameter is used,('top' or 't') parameters can not be used !"
        );
        should_not_use!(
            params.a_bottom,
            "When ('align' or 'a') parameter is used,('bottom' or 'b') parameters can not be used !"
        );
        should_not_use!(
            params.a_left,
            "When ('align' or 'a') parameter is used,('left' or 'l') parameters can not be used !"
        );
        should_not_use!(
            params.a_right,
            "When ('align' or 'a') parameter is used,('right' or 'r') parameters can not be used !"
        );
        should_not_use!(
            params.dock,
            "When ('align' or 'a') parameter is used,('dock' or 'd') parameters can not be used !"
        );
        should_not_use!(
            params.pivot,
            "When ('align' or 'a') parameter is used,('pivot' or 'p') parameters can not be used !"
        );
        PointAndSizeLayout {
            x: Coordinate16::Absolute(0),
            y: Coordinate16::Absolute(0),
            width: params.width.unwrap_or(Dimension16::Percentage(10000)),
            height: params.height.unwrap_or(Dimension16::Percentage(10000)),
            align: params.align.unwrap(),
            anchor: params.align.unwrap(),
        }
    }

    #[inline]
    pub(super) fn new_xy_width_height(params: &LayoutParameters) -> Self {
        // it is assume that DOCK|D is not set (as it was process early in ProcessAlignmentedLayout)
        // if X and Y are set --> Left, Right, Top and Bottom should not be set
        should_not_use!(
            params.a_left,
            "When (x,y) parameters are used, ('left' or 'l') parameter can not be used !"
        );
        should_not_use!(
            params.a_right,
            "When (x,y) parameters are used, ('right' or 'r') parameter can not be used !"
        );
        should_not_use!(params.a_top, "When (x,y) parameters are used, ('top' or 't') parameter can not be used !");
        should_not_use!(
            params.a_bottom,
            "When (x,y) parameters are used, ('bottom' or 'b') parameter can not be used !"
        );

        let a = match params.pivot.unwrap_or(Pivot::TopLeft) {
            Pivot::TopLeft => Alignment::TopLeft,
            Pivot::Top => Alignment::TopCenter,
            Pivot::TopRight => Alignment::TopRight,
            Pivot::Right => Alignment::CenterRight,
            Pivot::BottomRight => Alignment::BottomRight,
            Pivot::Bottom => Alignment::BottomCenter,
            Pivot::BottomLeft => Alignment::BottomLeft,
            Pivot::Left => Alignment::CenterLeft,
            Pivot::Center => Alignment::Center,
        };
        PointAndSizeLayout {
            x: params.x.unwrap(),
            y: params.y.unwrap(),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
            align: a,
            anchor: Alignment::TopLeft,
        }
    }

    #[inline]
    pub(super) fn new_corner_anchor(params: &LayoutParameters, anchor: Alignment) -> Self {
        should_not_use!(
            params.x,
            "When a corner anchor is being use (top,left,righ,bottom), 'x' can bot be used !"
        );
        should_not_use!(
            params.y,
            "When a corner anchor is being use (top,left,righ,bottom), 'y' can bot be used !"
        );

        PointAndSizeLayout {
            x: match anchor {
                Alignment::TopLeft | Alignment::BottomLeft => params.a_left.unwrap(),
                Alignment::TopRight | Alignment::BottomRight => params.a_right.unwrap(),
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            y: match anchor {
                Alignment::TopLeft | Alignment::TopRight => params.a_top.unwrap(),
                Alignment::BottomLeft | Alignment::BottomRight => params.a_bottom.unwrap(),
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
            align: anchor,
            anchor,
        }
    }

    #[inline]
    pub(super) fn update_control_layout(&self, control_layout: &mut ControlLayout, parent_width: u16, parent_height: u16) {
        control_layout.resize(self.width.absolute(parent_width), self.height.absolute(parent_height));
        let mut x = self.x.absolute(parent_width);
        let mut y = self.y.absolute(parent_height);

        // compute (x,y) based on anchor
        match self.anchor {
            Alignment::TopLeft => {}
            Alignment::TopCenter => x = (parent_width / 2) as i32,
            Alignment::TopRight => x = (parent_width as i32) - x,
            Alignment::CenterRight => {
                x = (parent_width as i32) - x;
                y = (parent_height / 2) as i32;
            }
            Alignment::BottomRight => {
                x = (parent_width as i32) - x;
                y = (parent_height as i32) - y;
            }
            Alignment::BottomCenter => {
                x = (parent_width / 2) as i32;
                y = (parent_height as i32) - y;
            }
            Alignment::BottomLeft => y = (parent_height as i32) - y,
            Alignment::CenterLeft => y = (parent_height / 2) as i32,
            Alignment::Center => {
                x = (parent_width / 2) as i32;
                y = (parent_height / 2) as i32;
            }
        }
        // align (x,y) from the current position based on Width/Height
        match self.align {
            Alignment::TopLeft => {}
            Alignment::TopCenter => x -= (control_layout.get_width() / 2) as i32,
            Alignment::TopRight => x -= control_layout.get_width() as i32,
            Alignment::CenterRight => {
                x -= control_layout.get_width() as i32;
                y -= (control_layout.get_height() / 2) as i32;
            }
            Alignment::BottomRight => {
                x -= control_layout.get_width() as i32;
                y -= control_layout.get_height() as i32;
            }
            Alignment::BottomCenter => {
                x -= (control_layout.get_width() / 2) as i32;
                y -= control_layout.get_height() as i32;
            }
            Alignment::BottomLeft => y -= control_layout.get_height() as i32,
            Alignment::CenterLeft => y -= (control_layout.get_height() / 2) as i32,
            Alignment::Center => {
                x -= (control_layout.get_width() / 2) as i32;
                y -= (control_layout.get_height() / 2) as i32;
            }
        }
        // set new position
        control_layout.set_position(x, y);
    }
}

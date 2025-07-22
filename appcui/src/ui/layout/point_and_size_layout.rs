use super::should_not_use;
use super::Alignment;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Dock;
use super::Error;
use super::Layout;
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
    pub(super) fn new_docked(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::XYParameterUsedWithDock);
        should_not_use!(params.y, Error::XYParameterUsedWithDock);
        should_not_use!(params.a_top, Error::AnchorParameterUsedWithDock);
        should_not_use!(params.a_bottom, Error::AnchorParameterUsedWithDock);
        should_not_use!(params.a_left, Error::AnchorParameterUsedWithDock);
        should_not_use!(params.a_right, Error::AnchorParameterUsedWithDock);
        should_not_use!(params.pivot, Error::PivotParameterUsedWithDock);
        should_not_use!(params.align, Error::AlignParameterUsedWithDock);

        match params.dock.unwrap() {
            Dock::Left => {
                should_not_use!(params.height, Error::HeightParameterUsedWithLeftOrRightDock);
                Ok(PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Absolute(0),
                    width: params.width.unwrap_or(Dimension16::Percentage(10000)),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopLeft,
                    anchor: Alignment::TopLeft,
                })
            }
            Dock::Right => {
                should_not_use!(params.height, Error::HeightParameterUsedWithLeftOrRightDock);
                Ok(PointAndSizeLayout {
                    x: Coordinate16::Percentage(10000),
                    y: Coordinate16::Absolute(0),
                    width: params.width.unwrap_or(Dimension16::Percentage(10000)),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopRight,
                    anchor: Alignment::TopRight,
                })
            }
            Dock::Top => {
                should_not_use!(params.width, Error::WidthParameterUsedWithTopOrBottomDock);
                Ok(PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Absolute(0),
                    width: Dimension16::Percentage(10000),
                    height: params.height.unwrap_or(Dimension16::Percentage(10000)),
                    align: Alignment::TopLeft,
                    anchor: Alignment::TopLeft,
                })
            }
            Dock::Bottom => {
                should_not_use!(params.width, Error::WidthParameterUsedWithTopOrBottomDock);
                Ok(PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Percentage(10000),
                    width: Dimension16::Percentage(10000),
                    height: params.height.unwrap_or(Dimension16::Percentage(10000)),
                    align: Alignment::BottomLeft,
                    anchor: Alignment::BottomLeft,
                })
            }
            Dock::Fill => {
                should_not_use!(params.height, Error::WidthOrHeightParameterUsedWithDockFill);
                should_not_use!(params.height, Error::WidthOrHeightParameterUsedWithDockFill);
                Ok(PointAndSizeLayout {
                    x: Coordinate16::Absolute(0),
                    y: Coordinate16::Absolute(0),
                    width: Dimension16::Percentage(10000),
                    height: Dimension16::Percentage(10000),
                    align: Alignment::TopLeft,
                    anchor: Alignment::TopLeft,
                })
            }
        }
    }

    pub(super) fn new_aligned(params: &Layout) -> Result<Self, Error> {
        should_not_use!(params.x, Error::XYParameterUsedWithAlign);
        should_not_use!(params.y, Error::XYParameterUsedWithAlign);
        should_not_use!(params.a_top, Error::AnchorParameterUsedWithAlign);
        should_not_use!(params.a_bottom, Error::AnchorParameterUsedWithAlign);
        should_not_use!(params.a_left, Error::AnchorParameterUsedWithAlign);
        should_not_use!(params.a_right, Error::AnchorParameterUsedWithAlign);
        should_not_use!(params.dock, Error::DockParameterUsedWithAlign);
        should_not_use!(params.pivot, Error::PivotParameterUsedWithAlign);
        Ok(PointAndSizeLayout {
            x: Coordinate16::Absolute(0),
            y: Coordinate16::Absolute(0),
            width: params.width.unwrap_or(Dimension16::Percentage(10000)),
            height: params.height.unwrap_or(Dimension16::Percentage(10000)),
            align: params.align.unwrap(),
            anchor: params.align.unwrap(),
        })
    }

    #[inline]
    pub(super) fn new_xy(params: &Layout) -> Result<Self, Error> {
        // it is assume that DOCK|D is not set (as it was process early in ProcessAlignmentedLayout)
        // if X and Y are set --> Left, Right, Top and Bottom should not be set
        should_not_use!(params.a_left, Error::AnchorParameterUsedWithXY);
        should_not_use!(params.a_right, Error::AnchorParameterUsedWithXY);
        should_not_use!(params.a_top, Error::AnchorParameterUsedWithXY);
        should_not_use!(params.a_bottom, Error::AnchorParameterUsedWithXY);

        let a = match params.pivot.unwrap_or(Pivot::TopLeft) {
            Pivot::TopLeft => Alignment::TopLeft,
            Pivot::TopCenter => Alignment::TopCenter,
            Pivot::TopRight => Alignment::TopRight,
            Pivot::CenterRight => Alignment::CenterRight,
            Pivot::BottomRight => Alignment::BottomRight,
            Pivot::BottomCenter => Alignment::BottomCenter,
            Pivot::BottomLeft => Alignment::BottomLeft,
            Pivot::CenterLeft => Alignment::CenterLeft,
            Pivot::Center => Alignment::Center,
        };
        Ok(PointAndSizeLayout {
            x: params.x.unwrap(),
            y: params.y.unwrap(),
            width: params.width.unwrap_or(Dimension16::Absolute(1)),
            height: params.height.unwrap_or(Dimension16::Absolute(1)),
            align: a,
            anchor: Alignment::TopLeft,
        })
    }

    #[inline]
    pub(super) fn new_corner_anchor(params: &Layout, anchor: Alignment) -> Result<Self, Error> {
        should_not_use!(params.x, Error::CornerAnchorParameterUsedWithXY);
        should_not_use!(params.y, Error::CornerAnchorParameterUsedWithXY);

        Ok(PointAndSizeLayout {
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
        })
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

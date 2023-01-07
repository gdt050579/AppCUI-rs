use super::Coordonate;
use super::Alignament;
use super::Size;
use super::LayoutParameters;
use super::should_not_use;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct PointAndSizeLayout {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub align: Alignament,
    pub anchor: Alignament,
}
impl PointAndSizeLayout {
    #[inline]
    pub (super) fn new_docked(params: &LayoutParameters) -> Self {
        should_not_use!(
            params.x,
            "When ('dock' or 'd') parameter is used,'x' parameter can not be used !"
        );
        should_not_use!(
            params.y,
            "When ('dock' or 'd') parameter is used,'y' parameter can not be used !"
        );
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
            params.align,
            "When ('dock' or 'd') parameter is used,('align' or 'a') parameters can not be used !"
        );

        PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: params.width.unwrap_or(Size::Percentage(1000)),
            height: params.height.unwrap_or(Size::Percentage(1000)),
            align: params.dock.unwrap(),
            anchor: params.dock.unwrap(),
        }
    }

    #[inline]
    pub (super) fn new_XYWH(params: &LayoutParameters) -> Self {
        // it is assume that DOCK|D is not set (as it was process early in ProcessDockedLayout)
        // if X and Y are set --> Left, Right, Top and Bottom should not be set
        should_not_use!(
            params.a_left,
            "When (x,y) parameters are used, ('left' or 'l') parameter can not be used !"
        );
        should_not_use!(
            params.a_right,
            "When (x,y) parameters are used, ('right' or 'r') parameter can not be used !"
        );
        should_not_use!(
            params.a_top,
            "When (x,y) parameters are used, ('top' or 't') parameter can not be used !"
        );
        should_not_use!(
            params.a_bottom,
            "When (x,y) parameters are used, ('bottom' or 'b') parameter can not be used !"
        );

        PointAndSizeLayout {
            x: params.x.unwrap(),
            y: params.y.unwrap(),
            width: params.width.unwrap_or(Size::Absolute(1)),
            height: params.height.unwrap_or(Size::Absolute(1)),
            align: params.align.unwrap_or(Alignament::TopLeft),
            anchor: Alignament::TopLeft,
        }
    }    

    #[inline]
    pub (super) fn new_corner_anchor(params: &LayoutParameters, anchor: Alignament) -> Self {
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
                Alignament::TopLeft | Alignament::BottomLeft => params.a_left.unwrap(),
                Alignament::TopRight | Alignament::BottomRight => params.a_right.unwrap(),
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            y: match anchor {
                Alignament::TopLeft | Alignament::TopRight => params.a_top.unwrap(),
                Alignament::BottomLeft | Alignament::BottomRight => params.a_bottom.unwrap(),
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            width: params.width.unwrap_or(Size::Absolute(1)),
            height: params.height.unwrap_or(Size::Absolute(1)),
            align: anchor,
            anchor: anchor,
        }
    }

}
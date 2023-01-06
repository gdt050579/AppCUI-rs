use super::Alignament;
use super::Anchors;
use super::Coordonate;
use super::LayoutParameters;
use super::Size;

macro_rules! should_not_use {
    ($param:expr, $msg:literal) => {
        if $param.is_some() {
            panic!($msg);
        }
    };
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct PointAndSizeLayout {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub align: Alignament,
    pub anchor: Alignament,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftRightAnchorsLayout {
    pub left: Coordonate,
    pub right: Coordonate,
    pub y: Coordonate,
    pub height: Size,
    pub align: Alignament,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopBottomAnchorsLayout {
    pub top: Coordonate,
    pub bottom: Coordonate,
    pub x: Coordonate,
    pub width: Size,
    pub align: Alignament,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftTopRightAnchorsLayout {
    pub left: Coordonate,
    pub top: Coordonate,
    pub right: Coordonate,
    pub height: Size,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftBottomRightAnchorsLayout {
    pub left: Coordonate,
    pub bottom: Coordonate,
    pub right: Coordonate,
    pub height: Size,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopLeftBottomAnchorsLayout {
    pub top: Coordonate,
    pub left: Coordonate,
    pub bottom: Coordonate,
    pub width: Size,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct TopRightBottomAnchorsLayout {
    pub top: Coordonate,
    pub right: Coordonate,
    pub bottom: Coordonate,
    pub width: Size,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct LeftTopRightBottomAnchorsLayout {
    pub left: Coordonate,
    pub top: Coordonate,
    pub right: Coordonate,
    pub bottom: Coordonate,
}

#[derive(Copy,Clone,PartialEq, Debug)]
pub(super) enum LayoutMode {
    PointAndSize(PointAndSizeLayout),
    LeftRightAnchors(LeftRightAnchorsLayout),
    TopBottomAnchors(TopBottomAnchorsLayout),

    LeftTopRightAnchors(LeftTopRightAnchorsLayout),
    LeftBottomRightAnchors(LeftBottomRightAnchorsLayout),
    TopLeftBottomAnchors(TopLeftBottomAnchorsLayout),
    TopRightBottomAnchors(TopRightBottomAnchorsLayout),

    LeftTopRightBottomAnchors(LeftTopRightBottomAnchorsLayout),
}
impl LayoutMode {
    fn new_docked_layout(params: &LayoutParameters) -> LayoutMode {
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

        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: params.width.unwrap_or(Size::Percentage(1000)),
            height: params.height.unwrap_or(Size::Percentage(1000)),
            align: params.dock.unwrap(),
            anchor: params.dock.unwrap(),
        })
    }
    fn new_XYWH_layout(params: &LayoutParameters) -> LayoutMode {
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

        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: params.x.unwrap(),
            y: params.y.unwrap(),
            width: params.width.unwrap_or(Size::Absolute(1)),
            height: params.height.unwrap_or(Size::Absolute(1)),
            align: params.align.unwrap_or(Alignament::TopLeft),
            anchor: Alignament::TopLeft,
        })
    }
    fn new_corner_anchor_layout(params: &LayoutParameters, anchor: Alignament) -> LayoutMode {
        should_not_use!(
            params.x,
            "When a corner anchor is being use (top,left,righ,bottom), 'x' can bot be used !"
        );
        should_not_use!(
            params.y,
            "When a corner anchor is being use (top,left,righ,bottom), 'y' can bot be used !"
        );

        LayoutMode::PointAndSize(PointAndSizeLayout {
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
        })
    }
    fn new_horizontal_anchor_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(
            params.x,
            "When (left,right) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(params.width,"When (left,right) parameters are used toghere, ('width' or 'w') parameters can not be used as the width is deduced from left-right difference");

        if let Some(align) = params.align {
            match align {
                Alignament::Top|Alignament::Center|Alignament::Bottom => {},
                _ => panic!("When (left,right) are provided, only Top(t), Center(c) and Bottom(b) alignament values are allowed !")
            }
        }

        LayoutMode::LeftRightAnchors(LeftRightAnchorsLayout {
            left: params.a_left.unwrap(),
            right: params.a_right.unwrap(),
            y: params.y.unwrap_or(Coordonate::Absolute(0)),
            height: params.height.unwrap_or(Size::Absolute(1)),
            align: params.align.unwrap_or(Alignament::Center),
        })
    }
    fn new_vertical_anchor_layout(params: &LayoutParameters) -> LayoutMode {
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

        LayoutMode::TopBottomAnchors(TopBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            bottom: params.a_bottom.unwrap(),
            x: params.x.unwrap_or(Coordonate::Absolute(0)),
            width: params.width.unwrap_or(Size::Absolute(1)),
            align: params.align.unwrap_or(Alignament::Center),
        })
    }
    fn new_LTR_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(
            params.x,
            "When (left,top,right) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (left,top,right) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.width,
            "When (left,top,right) parameters are used together, 'width' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (left,top,right) parameters are used together, 'align' parameter can not be used"
        );

        LayoutMode::LeftTopRightAnchors(LeftTopRightAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            height: params.height.unwrap_or(Size::Absolute(1)),
        })
    }
    fn new_LBR_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(
            params.x,
            "When (left,bottom,right) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (left,bottom,right) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.width,
            "When (left,bottom,right) parameters are used together, 'width' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (left,bottom,right) parameters are used together, 'align' parameter can not be used"
        );

        LayoutMode::LeftBottomRightAnchors(LeftBottomRightAnchorsLayout {
            left: params.a_left.unwrap(),
            bottom: params.a_bottom.unwrap(),
            right: params.a_right.unwrap(),
            height: params.height.unwrap_or(Size::Absolute(1)),
        })
    }
    fn new_TLB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
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

        LayoutMode::TopLeftBottomAnchors(TopLeftBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            left: params.a_left.unwrap(),
            bottom: params.a_bottom.unwrap(),
            width: params.width.unwrap_or(Size::Absolute(1)),
        })
    }
    fn new_TRB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(
            params.x,
            "When (top,right,bottom) parameters are used together, 'X' parameter can not be used"
        );
        should_not_use!(
            params.y,
            "When (top,right,bottom) parameters are used together, 'Y' parameter can not be used"
        );
        should_not_use!(
            params.height,
            "When (top,right,bottom) parameters are used together, 'height' parameter can not be used"
        );
        should_not_use!(
            params.align,
            "When (top,right,bottom) parameters are used together, 'align' parameter can not be used"
        );

        LayoutMode::TopRightBottomAnchors(TopRightBottomAnchorsLayout {
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
            width: params.width.unwrap_or(Size::Absolute(1)),
        })
    }
    fn new_LTRB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(params.x, "When (left,top,right,bottom) parameters are used together, 'X' parameter can not be used");
        should_not_use!(params.y, "When (left,top,right,bottom) parameters are used together, 'Y' parameter can not be used");
        should_not_use!(params.height, "When (left,top,right,bottom) parameters are used together, 'height' parameter can not be used");
        should_not_use!(params.width, "When (left,top,right,bottom) parameters are used together, 'widyj' parameter can not be used");
        should_not_use!(params.align, "When (left,top,right,bottom) parameters are used together, 'align' parameter can not be used");

        LayoutMode::LeftTopRightBottomAnchors(LeftTopRightBottomAnchorsLayout {
            left: params.a_left.unwrap(),
            top: params.a_top.unwrap(),
            right: params.a_right.unwrap(),
            bottom: params.a_bottom.unwrap(),
        })
    }

    pub(super) fn new(format: &str) -> LayoutMode {
        let params_list = LayoutParameters::new(format);

        // check if layout params are OK
        // Step 1 ==> if dock option is present
        if params_list.dock.is_some() {
            return LayoutMode::new_docked_layout(&params_list);
        }
        // Step 2 ==> check (X,Y) + (W,H) + (optional align)
        if params_list.x.is_some() && params_list.y.is_some() {
            return LayoutMode::new_XYWH_layout(&params_list);
        }

        let anchors = params_list.get_anchors();
        match anchors {
            Anchors::TopLeft => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopLeft);
            }
            Anchors::TopRight => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopRight);
            }
            Anchors::BottomRight => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomRight);
            }
            Anchors::BottomLeft => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomLeft);
            }
            Anchors::LeftRight=> {
                return LayoutMode::new_horizontal_anchor_layout(&params_list);
            }
            Anchors::TopBottom => {
                return LayoutMode::new_vertical_anchor_layout(&params_list);
            }
            Anchors::LeftTopRight => {
                return LayoutMode::new_LTR_anchors_layout(&params_list);
            }
            Anchors::LeftBottomRight => {
                return LayoutMode::new_LBR_anchors_layout(&params_list);
            }
            Anchors::TopLeftBottom => {
                return LayoutMode::new_TLB_anchors_layout(&params_list);
            }
            Anchors::TopRightBottom => {
                return LayoutMode::new_TRB_anchors_layout(&params_list);
            }
            Anchors::All => {
                return LayoutMode::new_LTRB_anchors_layout(&params_list);
            }
            _ => {
                panic!("Invalid format: {} --> this combination can not be used to create a layout for a control ", format);
            }
        }
    }
}

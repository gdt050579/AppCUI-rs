use super::Alignament;
use super::Coordonate;
use super::LayoutParameters;
use super::LayoutUsedParams;
use super::Size;

#[derive(Copy, Clone)]
pub(super) struct PointAndSizeLayout {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub align: Alignament,
    pub anchor: Alignament,
}

pub(super) enum LayoutMode {
    None,
    PointAndSize(PointAndSizeLayout),
    LeftRightAnchorsAndHeight,
    TopBottomAnchorsAndWidth,

    LeftTopRightAnchorsAndHeight,
    LeftBottomRightAnchorsAndHeight,
    TopLeftBottomAnchorsAndWidth,
    TopRightBottomAnchorsAndWidth,

    LeftTopRightBottomAnchors,
}
impl LayoutMode {
    fn new_docked_layout(params: &LayoutParameters) -> LayoutMode {
        if params.used_params.contains_one(
            LayoutUsedParams::X
                | LayoutUsedParams::Y
                | LayoutUsedParams::TOP
                | LayoutUsedParams::BOTTOM
                | LayoutUsedParams::LEFT
                | LayoutUsedParams::RIGHT,
        ) {
            panic!("When dock|d parameter is used, none of the position (x,y) or anchor (left,right,bottom,top) parameters can not be uesd");
        }
        if params.used_params.contains_one(LayoutUsedParams::ALIGN) {
            panic!("When dock|d parameter is used, 'align' parameter can not be used !");
        }
        // if width or height are not present, default them to 100%

        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: if params.used_params.contains(LayoutUsedParams::WIDTH) {
                params.width
            } else {
                Size::Percentage(10000)
            },
            height: if params.used_params.contains(LayoutUsedParams::HEIGHT) {
                params.height
            } else {
                Size::Percentage(10000)
            },
            align: params.dock,
            anchor: params.dock,
        })
    }
    fn new_XYWH_layout(params: &LayoutParameters) -> LayoutMode {
        // it is assume that DOCK|D is not set (as it was process early in ProcessDockedLayout)
        // if X and Y are set --> Left, Right, Top and Bottom should not be set
        if params.used_params.contains_one(
            LayoutUsedParams::LEFT
                | LayoutUsedParams::RIGHT
                | LayoutUsedParams::TOP
                | LayoutUsedParams::BOTTOM,
        ) {
            panic!("When (x,y) parameters are used, none of the anchor (left,right,bottom,top) parameters can not be used");
        }

        // if width or height are not present, they are defaulted to 1 character
        // if align is not presented, it is defaulted to TopLeft
        // anchor is always TopLeft

        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: params.x,
            y: params.y,
            width: if params.used_params.contains(LayoutUsedParams::WIDTH) {
                params.width
            } else {
                Size::Absolute(1)
            },
            height: if params.used_params.contains(LayoutUsedParams::HEIGHT) {
                params.height
            } else {
                Size::Absolute(1)
            },
            align: if params.used_params.contains(LayoutUsedParams::ALIGN) {
                params.align
            } else {
                Alignament::TopLeft
            },
            anchor: Alignament::TopLeft,
        })
    }
    fn new_corner_anchor_layout(params: &LayoutParameters, anchor: Alignament) -> LayoutMode {
        if params
            .used_params
            .contains_one(LayoutUsedParams::X | LayoutUsedParams::Y)
        {
            panic!("When a corner anchor is being use (top,left,righ,bottom) , (X,Y) coordonates can not be used");
        }

        // if width or height are not present, they are defaulted to 1 character
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: match anchor {
                Alignament::TopLeft | Alignament::BottomLeft => params.a_left,
                Alignament::TopRight | Alignament::BottomRight => params.a_right,
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            y: match anchor {
                Alignament::TopLeft | Alignament::TopRight => params.a_top,
                Alignament::BottomLeft | Alignament::BottomRight => params.a_bottom,
                _ => unreachable!("Internal error --> this point should not ne reached"),
            },
            width: if params.used_params.contains(LayoutUsedParams::WIDTH) {
                params.width
            } else {
                Size::Absolute(1)
            },
            height: if params.used_params.contains(LayoutUsedParams::HEIGHT) {
                params.height
            } else {
                Size::Absolute(1)
            },
            align: anchor,
            anchor: anchor,
        })
    }
    fn new_horizontal_anchor_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_vertical_anchor_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_LTR_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_LBR_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_TLB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_TRB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_LTRB_anchors_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    pub(super) fn new(format: &str) -> LayoutMode {
        let params_list = LayoutParameters::new(format);

        // check if layout params are OK
        // Step 1 ==> if dock option is present
        if params_list.used_params.contains(LayoutUsedParams::DOCK) {
            return LayoutMode::new_docked_layout(&params_list);
        }
        // Step 2 ==> check (X,Y) + (W,H) + (optional align)
        if params_list
            .used_params
            .contains(LayoutUsedParams::X | LayoutUsedParams::Y)
        {
            return LayoutMode::new_XYWH_layout(&params_list);
        }

        let anchors = (params_list.used_params
            & (LayoutUsedParams::LEFT
                | LayoutUsedParams::TOP
                | LayoutUsedParams::RIGHT
                | LayoutUsedParams::BOTTOM))
            .get_value();
        const LEFT_TOP_ANCHOR: u16 =
            LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::TOP.get_value();
        const RIGHT_TOP_ANCHOR: u16 =
            LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::TOP.get_value();
        const LEFT_BOTTOM_ANCHOR: u16 =
            LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const RIGHT_BOTTOM_ANCHOR: u16 =
            LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const HORIZONTAL_ANCHOR: u16 =
            LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::RIGHT.get_value();
        const VERTICAL_ANCHOR: u16 =
            LayoutUsedParams::TOP.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const LTR_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value()
            | LayoutUsedParams::TOP.get_value()
            | LayoutUsedParams::RIGHT.get_value();
        const LBR_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value()
            | LayoutUsedParams::BOTTOM.get_value()
            | LayoutUsedParams::RIGHT.get_value();
        const TLB_ANCHOR: u16 = LayoutUsedParams::TOP.get_value()
            | LayoutUsedParams::LEFT.get_value()
            | LayoutUsedParams::BOTTOM.get_value();
        const TRB_ANCHOR: u16 = LayoutUsedParams::TOP.get_value()
            | LayoutUsedParams::RIGHT.get_value()
            | LayoutUsedParams::BOTTOM.get_value();
        const LTRB_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value()
            | LayoutUsedParams::TOP.get_value()
            | LayoutUsedParams::RIGHT.get_value()
            | LayoutUsedParams::BOTTOM.get_value();
        match anchors {
            LEFT_TOP_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopLeft);
            }
            RIGHT_TOP_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopRight);
            }
            RIGHT_BOTTOM_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomRight);
            }
            LEFT_BOTTOM_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomLeft);
            }
            HORIZONTAL_ANCHOR => {
                return LayoutMode::new_horizontal_anchor_layout(&params_list);
            }
            VERTICAL_ANCHOR => {
                return LayoutMode::new_vertical_anchor_layout(&params_list);
            }
            LTR_ANCHOR => {
                return LayoutMode::new_LTR_anchors_layout(&params_list);
            }
            LBR_ANCHOR => {
                return LayoutMode::new_LBR_anchors_layout(&params_list);
            }
            TLB_ANCHOR => {
                return LayoutMode::new_TLB_anchors_layout(&params_list);
            }
            TRB_ANCHOR => {
                return LayoutMode::new_TRB_anchors_layout(&params_list);
            }
            LTRB_ANCHOR => {
                return LayoutMode::new_LTRB_anchors_layout(&params_list);
            }
            _ => {
                panic!("Invalid format: {} --> this combination can not be used to create a layout for a control ", format);
            }
        }
    }
}

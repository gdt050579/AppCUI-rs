use super::Alignament;
use super::Coordonate;
use super::LayoutParameters;
use super::LayoutUsedParams;
use super::Size;

macro_rules! should_not_use {
    ($param:expr, $msg:literal) => {
        if $param.is_some() {
            panic!($msg);
        }
    };
}

#[derive(Copy, Clone)]
pub(super) struct PointAndSizeLayout {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub align: Alignament,
    pub anchor: Alignament,
}
#[derive(Copy, Clone)]
pub(super) struct LeftRightAnchorsLayout {
    pub left: Coordonate,
    pub right: Coordonate,
    pub y: Coordonate,
    pub height: Size,
    pub align: Alignament,
}

pub(super) enum LayoutMode {
    None,
    PointAndSize(PointAndSizeLayout),
    LeftRightAnchors(LeftRightAnchorsLayout),
    TopBottomAnchorsAndWidth,

    LeftTopRightAnchorsAndHeight,
    LeftBottomRightAnchorsAndHeight,
    TopLeftBottomAnchorsAndWidth,
    TopRightBottomAnchorsAndWidth,

    LeftTopRightBottomAnchors,
}
impl LayoutMode {
    fn new_docked_layout(params: &LayoutParameters) -> LayoutMode {
        should_not_use!(params.x,"When ('dock' or 'd') parameter is used,'x' parameter can not be used !");
        should_not_use!(params.y,"When ('dock' or 'd') parameter is used,'y' parameter can not be used !");
        should_not_use!(params.a_top,"When ('dock' or 'd') parameter is used,('top' or 't') parameters can not be used !");
        should_not_use!(params.a_bottom,"When ('dock' or 'd') parameter is used,('bottom' or 'b') parameters can not be used !");
        should_not_use!(params.a_left,"When ('dock' or 'd') parameter is used,('left' or 'l') parameters can not be used !");
        should_not_use!(params.a_right,"When ('dock' or 'd') parameter is used,('right' or 'r') parameters can not be used !");
        should_not_use!(params.align,"When ('dock' or 'd') parameter is used,('align' or 'a') parameters can not be used !");

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
        if params.used_params.contains(LayoutUsedParams::X) {
            panic!("When (left,right) parameters are used toghere, 'X' parameter can not be used");
        }
        if params.used_params.contains(LayoutUsedParams::WIDTH) {
            panic!("When (left,right) parameters are used toghere, width can not be used as it is deduced from left-right difference");
        }
        if params.used_params.contains(LayoutUsedParams::ALIGN) {
            match params.align {
                Alignament::Top|Alignament::Center|Alignament::Bottom => {},
                _ => panic!("When (left,right) are provided, only Top(t), Center(c) and Bottom(b) alignament values are allowed !")
            }
        }
        // if y is not provided ==> default it to 0
        // if align is not provided ==> default it to Center
        // if height is not provided ==> default it to 1 char
        LayoutMode::LeftRightAnchors(LeftRightAnchorsLayout {
            left: params.a_left,
            right: params.a_right,
            y: (),
            height: (),
            align: (),
        })

        /*


        // if "align" is not provided, it is defaulted to center
        if ((inf.flags & LAYOUT_FLAG_ALIGN) == 0)
            inf.align = Alignament::Center;


        // if "height" is not provided, it is defaulted to 1
        if ((inf.flags & LAYOUT_FLAG_HEIGHT) == 0)
            this->Layout.Format.Height = { 1, LayoutValueType::CharacterOffset };
        else
            this->Layout.Format.Height = inf.height;

        // if "Y" is not provided, it is defaulted to 0
        if ((inf.flags & LAYOUT_FLAG_Y) == 0)
            this->Layout.Format.Y = { 0, LayoutValueType::CharacterOffset };
        else
            this->Layout.Format.Y = inf.y;

        // construct de layout
        this->Layout.Format.LayoutMode  = LayoutFormatMode::LeftRightAnchorsAndHeight;
        this->Layout.Format.AnchorLeft  = inf.a_left;
        this->Layout.Format.AnchorRight = inf.a_right;
        this->Layout.Format.Align       = inf.align;

        // all good
        return true;


            */
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

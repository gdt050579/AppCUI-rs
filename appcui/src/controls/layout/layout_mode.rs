use super::{alignament::Alignament, layout_parameters::LayoutUsedParams, LayoutParameters};

pub(super) enum LayoutMode {
    None,
    PointAndSize,
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
        todo!();
    }
    fn new_XYWH_layout(params: &LayoutParameters) -> LayoutMode {
        todo!();
    }
    fn new_corner_anchor_layout(params: &LayoutParameters, align: Alignament) -> LayoutMode {
        todo!();
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
                | LayoutUsedParams::BOTTOM)).get_value();
        const LEFT_TOP_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::TOP.get_value();
        const RIGHT_TOP_ANCHOR: u16 = LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::TOP.get_value();
        const LEFT_BOTTOM_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const RIGHT_BOTTOM_ANCHOR: u16 = LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const HORIZONTAL_ANCHOR: u16 =LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::RIGHT.get_value();
        const VERTICAL_ANCHOR: u16 = LayoutUsedParams::TOP.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const LTR_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::TOP.get_value() | LayoutUsedParams::RIGHT.get_value();
        const LBR_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::BOTTOM.get_value() | LayoutUsedParams::RIGHT.get_value();
        const TLB_ANCHOR: u16 = LayoutUsedParams::TOP.get_value() | LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const TRB_ANCHOR: u16 = LayoutUsedParams::TOP.get_value() | LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        const LTRB_ANCHOR: u16 = LayoutUsedParams::LEFT.get_value() | LayoutUsedParams::TOP.get_value() | LayoutUsedParams::RIGHT.get_value() | LayoutUsedParams::BOTTOM.get_value();
        match anchors {
            LEFT_TOP_ANCHOR  => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopLeft);
            },
            RIGHT_TOP_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopRight);
            },
            RIGHT_BOTTOM_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomRight);
            },
            LEFT_BOTTOM_ANCHOR => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomLeft);
            },
            HORIZONTAL_ANCHOR => {
                return LayoutMode::new_horizontal_anchor_layout(&params_list);
            },
            VERTICAL_ANCHOR => {
                return LayoutMode::new_vertical_anchor_layout(&params_list);
            },
            LTR_ANCHOR => {
                return LayoutMode::new_LTR_anchors_layout(&params_list);
            },
            LBR_ANCHOR=> {
                return LayoutMode::new_LBR_anchors_layout(&params_list);
            },
            TLB_ANCHOR => {
                return LayoutMode::new_TLB_anchors_layout(&params_list);
            },
            TRB_ANCHOR=> {
                return LayoutMode::new_TRB_anchors_layout(&params_list);
            },   
            LTRB_ANCHOR => {
                return LayoutMode::new_LTRB_anchors_layout(&params_list);
            },  
            _ => {
                panic!("Invalid format: {} --> this combination can not be used to create a layout for a control ", format);
            }       
        }
    }
}

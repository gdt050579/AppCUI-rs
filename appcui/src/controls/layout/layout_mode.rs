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

        let anchors = params_list.used_params
            & (LayoutUsedParams::LEFT
                | LayoutUsedParams::TOP
                | LayoutUsedParams::RIGHT
                | LayoutUsedParams::BOTTOM);
        match anchors {
            (LayoutUsedParams::LEFT | LayoutUsedParams::TOP) => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopLeft);
            },
            (LayoutUsedParams::RIGHT | LayoutUsedParams::TOP) => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::TopRight);
            },
            (LayoutUsedParams::RIGHT | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomRight);
            },
            (LayoutUsedParams::LEFT | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_corner_anchor_layout(&params_list, Alignament::BottomLeft);
            },
            (LayoutUsedParams::LEFT | LayoutUsedParams::RIGHT) => {
                return LayoutMode::new_horizontal_anchor_layout(&params_list);
            },
            (LayoutUsedParams::TOP | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_vertical_anchor_layout(&params_list);
            },
            (LayoutUsedParams::LEFT | LayoutUsedParams::TOP | LayoutUsedParams::RIGHT) => {
                return LayoutMode::new_LTR_anchors_layout(&params_list);
            },
            (LayoutUsedParams::LEFT | LayoutUsedParams::BOTTOM | LayoutUsedParams::RIGHT) => {
                return LayoutMode::new_LBR_anchors_layout(&params_list);
            },
            (LayoutUsedParams::TOP | LayoutUsedParams::LEFT | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_TLB_anchors_layout(&params_list);
            },
            (LayoutUsedParams::TOP | LayoutUsedParams::RIGHT | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_TRB_anchors_layout(&params_list);
            },   
            (LayoutUsedParams::LEFT|LayoutUsedParams::TOP | LayoutUsedParams::RIGHT | LayoutUsedParams::BOTTOM) => {
                return LayoutMode::new_LTRB_anchors_layout(&params_list);
            },  
            _ => {
                panic!("Invalid format: {} --> this combination can not be used to create a layout for a control ", format);
            }       
        }
    }
}

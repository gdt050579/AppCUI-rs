use super::should_not_use;
use super::Alignament;
use super::Anchors;
use super::Coordonate;
use super::LayoutParameters;
use super::LeftRightAnchorsLayout;
use super::TopBottomAnchorsLayout;
use super::LeftTopRightAnchorsLayout;
use super::LeftBottomRightAnchorsLayout;
use super::TopLeftBottomAnchorsLayout;
use super::PointAndSizeLayout;
use super::Size;



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

#[derive(Copy, Clone, PartialEq, Debug)]
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
            return LayoutMode::PointAndSize(PointAndSizeLayout::new_docked(&params_list));
        }
        // Step 2 ==> check (X,Y) + (W,H) + (optional align)
        if params_list.x.is_some() && params_list.y.is_some() {
            return LayoutMode::PointAndSize(PointAndSizeLayout::new_XYWH(&params_list));
        }

        let anchors = params_list.get_anchors();
        match anchors {
            Anchors::TopLeft => {
                return LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(
                    &params_list,
                    Alignament::TopLeft,
                ));
            }
            Anchors::TopRight => {
                return LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(
                    &params_list,
                    Alignament::TopRight,
                ));
            }
            Anchors::BottomRight => {
                return LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(
                    &params_list,
                    Alignament::BottomRight,
                ));
            }
            Anchors::BottomLeft => {
                return LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(
                    &params_list,
                    Alignament::BottomLeft,
                ));
            }
            Anchors::LeftRight => {
                return LayoutMode::LeftRightAnchors(LeftRightAnchorsLayout::new(&params_list));
            }
            Anchors::TopBottom => {
                return LayoutMode::TopBottomAnchors(TopBottomAnchorsLayout::new(&params_list));
            }
            Anchors::LeftTopRight => {
                return LayoutMode::LeftTopRightAnchors(LeftTopRightAnchorsLayout::new(&params_list));
            }
            Anchors::LeftBottomRight => {
                return LayoutMode::LeftBottomRightAnchors(LeftBottomRightAnchorsLayout::new(&params_list));
            }
            Anchors::TopLeftBottom => {
                return LayoutMode::TopLeftBottomAnchors(TopLeftBottomAnchorsLayout::new(&params_list));
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

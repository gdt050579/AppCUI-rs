use super::Alignament;
use super::Anchors;
use super::LayoutParameters;
use super::LeftRightAnchorsLayout;
use super::TopBottomAnchorsLayout;
use super::LeftTopRightAnchorsLayout;
use super::LeftBottomRightAnchorsLayout;
use super::TopLeftBottomAnchorsLayout;
use super::TopRightBottomAnchorsLayout;
use super::PointAndSizeLayout;
use super::AllAnchorsLayout;


#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) enum LayoutMode {
    PointAndSize(PointAndSizeLayout),
    LeftRightAnchors(LeftRightAnchorsLayout),
    TopBottomAnchors(TopBottomAnchorsLayout),

    LeftTopRightAnchors(LeftTopRightAnchorsLayout),
    LeftBottomRightAnchors(LeftBottomRightAnchorsLayout),
    TopLeftBottomAnchors(TopLeftBottomAnchorsLayout),
    TopRightBottomAnchors(TopRightBottomAnchorsLayout),

    AllAnchors(AllAnchorsLayout),
}
impl LayoutMode {
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
                return LayoutMode::TopRightBottomAnchors(TopRightBottomAnchorsLayout::new(&params_list));
            }
            Anchors::All => {
                return LayoutMode::AllAnchors(AllAnchorsLayout::new(&params_list));
            }
            _ => {
                panic!("Invalid format: {} --> this combination can not be used to create a layout for a control ", format);
            }
        }
    }
}

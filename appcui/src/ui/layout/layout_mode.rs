use super::AbsoluteLayout;
use super::Alignament;
use super::AllAnchorsLayout;
use super::Anchors;
use super::LayoutParameters;
use super::LeftBottomRightAnchorsLayout;
use super::LeftRightAnchorsLayout;
use super::LeftTopRightAnchorsLayout;
use super::PointAndSizeLayout;
use super::TopBottomAnchorsLayout;
use super::TopLeftBottomAnchorsLayout;
use super::TopRightBottomAnchorsLayout;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) enum LayoutMode {
    Absolute(AbsoluteLayout),
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
            // if all we have is (X,Y) + (W,H) check to see if it is an absolute layout
            if (params_list.width.is_some())
                && (params_list.height.is_some())
                && (params_list.align.is_none())
                && (params_list.a_top.is_none())
                && (params_list.a_left.is_none())
                && (params_list.a_bottom.is_none())
                && (params_list.a_right.is_none())
            {
                let x = params_list.x.unwrap();
                let y = params_list.y.unwrap();
                let w = params_list.width.unwrap();
                let h = params_list.height.unwrap();
                if x.is_absolute() && y.is_absolute() && w.is_absolute() && h.is_absolute() {
                    let w = w.absolute(0);
                    let h = h.absolute(0);
                    if (w > 0) && (h > 0) {
                        return LayoutMode::Absolute(AbsoluteLayout::new(x.absolute(0), y.absolute(0), w, h));
                    }
                }
            }
            return LayoutMode::PointAndSize(PointAndSizeLayout::new_xy_width_height(&params_list));
        }

        let anchors = params_list.get_anchors();
        match anchors {
            Anchors::TopLeft => LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(&params_list, Alignament::TopLeft)),
            Anchors::TopRight => LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(&params_list, Alignament::TopRight)),
            Anchors::BottomRight => LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(&params_list, Alignament::BottomRight)),
            Anchors::BottomLeft => LayoutMode::PointAndSize(PointAndSizeLayout::new_corner_anchor(&params_list, Alignament::BottomLeft)),
            Anchors::LeftRight => LayoutMode::LeftRightAnchors(LeftRightAnchorsLayout::new(&params_list)),
            Anchors::TopBottom => LayoutMode::TopBottomAnchors(TopBottomAnchorsLayout::new(&params_list)),
            Anchors::LeftTopRight => LayoutMode::LeftTopRightAnchors(LeftTopRightAnchorsLayout::new(&params_list)),
            Anchors::LeftBottomRight => LayoutMode::LeftBottomRightAnchors(LeftBottomRightAnchorsLayout::new(&params_list)),
            Anchors::TopLeftBottom => LayoutMode::TopLeftBottomAnchors(TopLeftBottomAnchorsLayout::new(&params_list)),
            Anchors::TopRightBottom => LayoutMode::TopRightBottomAnchors(TopRightBottomAnchorsLayout::new(&params_list)),
            Anchors::All => LayoutMode::AllAnchors(AllAnchorsLayout::new(&params_list)),
            _ => {
                panic!(
                    "Invalid format: {} --> this combination can not be used to create a layout for a control ",
                    format
                );
            }
        }
    }
}

impl Default for LayoutMode {
    fn default() -> LayoutMode {
        LayoutMode::Absolute(AbsoluteLayout::new(0, 0, 0, 0))
    }
}

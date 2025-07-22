use super::AbsoluteLayout;
use super::Alignment;
use super::AllAnchorsLayout;
use super::Anchors;
use super::Error;
use super::Layout;
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
    pub(super) fn new(layout: Layout) -> Result<LayoutMode, Error> {
        // check if layout params are OK
        // Step 1 ==> if dock option is present
        if layout.dock.is_some() {
            return PointAndSizeLayout::new_docked(&layout).map(|layout| LayoutMode::PointAndSize(layout));
        }
        // Step 2 ==> if align option is present
        if layout.align.is_some() {
            return PointAndSizeLayout::new_aligned(&layout).map(|layout| LayoutMode::PointAndSize(layout));
        }
        // Step 2 ==> check (X,Y) + (W,H) + (optional pivot)
        if layout.x.is_some() && layout.y.is_some() {
            // if all we have is (X,Y) + (W,H) check to see if it is an absolute layout
            if (layout.width.is_some())
                && (layout.height.is_some())
                && (layout.align.is_none())
                && (layout.pivot.is_none())
                && (layout.dock.is_none())
                && (layout.a_top.is_none())
                && (layout.a_left.is_none())
                && (layout.a_bottom.is_none())
                && (layout.a_right.is_none())
            {
                let x = layout.x.unwrap();
                let y = layout.y.unwrap();
                let w = layout.width.unwrap();
                let h = layout.height.unwrap();
                if x.is_absolute() && y.is_absolute() && w.is_absolute() && h.is_absolute() {
                    let w = w.absolute(0);
                    let h = h.absolute(0);
                    if (w > 0) && (h > 0) {
                        return Ok(LayoutMode::Absolute(AbsoluteLayout::new(x.absolute(0), y.absolute(0), w, h)));
                    }
                }
            }
            return PointAndSizeLayout::new_xy(&layout).map(|layout| LayoutMode::PointAndSize(layout));
        }

        let anchors = layout.anchors();
        match anchors {
            Anchors::TopLeft => PointAndSizeLayout::new_corner_anchor(&layout, Alignment::TopLeft).map(|layout| LayoutMode::PointAndSize(layout)),
            Anchors::TopRight => PointAndSizeLayout::new_corner_anchor(&layout, Alignment::TopRight).map(|layout| LayoutMode::PointAndSize(layout)),
            Anchors::BottomRight => PointAndSizeLayout::new_corner_anchor(&layout, Alignment::BottomRight).map(|layout| LayoutMode::PointAndSize(layout)),
            Anchors::BottomLeft => PointAndSizeLayout::new_corner_anchor(&layout, Alignment::BottomLeft).map(|layout| LayoutMode::PointAndSize(layout)),
            Anchors::LeftRight => LeftRightAnchorsLayout::new(&layout).map(|layout| LayoutMode::LeftRightAnchors(layout)),
            Anchors::TopBottom => LayoutMode::TopBottomAnchors(TopBottomAnchorsLayout::new(&layout)),
            Anchors::LeftTopRight => LeftTopRightAnchorsLayout::new(&layout).map(|layout| LayoutMode::LeftTopRightAnchors(layout)),
            Anchors::LeftBottomRight => LayoutMode::LeftBottomRightAnchors(LeftBottomRightAnchorsLayout::new(&layout)),
            Anchors::TopLeftBottom => LayoutMode::TopLeftBottomAnchors(TopLeftBottomAnchorsLayout::new(&layout)),
            Anchors::TopRightBottom => LayoutMode::TopRightBottomAnchors(TopRightBottomAnchorsLayout::new(&layout)),
            Anchors::All => AllAnchorsLayout::new(&layout).map(|layout| LayoutMode::AllAnchors(layout)),
            _ => {
                // different errors
                if layout.x.is_none()
                    && layout.y.is_none()
                    && layout.width.is_none()
                    && layout.height.is_none()
                    && layout.pivot.is_none()
                    && layout.align.is_none()
                    && layout.dock.is_none()
                    && layout.anchors() == Anchors::None
                {
                    return Err(Error::NoParameters);
                }
                return Err(Error::InvalidLayoutRule);
            }
        }
    }
}

impl Default for LayoutMode {
    fn default() -> LayoutMode {
        LayoutMode::Absolute(AbsoluteLayout::new(0, 0, 0, 0))
    }
}

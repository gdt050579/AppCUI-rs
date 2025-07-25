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
        let anchors = layout.anchors();

        if layout.x.is_none()
            && layout.y.is_none()
            && layout.width.is_none()
            && layout.height.is_none()
            && layout.pivot.is_none()
            && layout.align.is_none()
            && layout.dock.is_none()
            && anchors == Anchors::None
        {
            return Err(Error::NoParameters);
        }
        // Step 1 ==> if dock option is present
        if layout.dock.is_some() {
            return PointAndSizeLayout::new_docked(&layout).map(|layout| LayoutMode::PointAndSize(layout));
        }
        // Step 2 ==> if align option is present
        if layout.align.is_some() {
            return PointAndSizeLayout::new_aligned(&layout).map(|layout| LayoutMode::PointAndSize(layout));
        }
        // Step 3 ==> check (X,Y) + (W,H) + (optional pivot)
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

        // step 4 ==> check anchors
        match anchors {
            Anchors::TopLeft => {
                return PointAndSizeLayout::new_corner_anchor(&layout, Alignment::TopLeft).map(|layout| LayoutMode::PointAndSize(layout))
            }
            Anchors::TopRight => {
                return PointAndSizeLayout::new_corner_anchor(&layout, Alignment::TopRight).map(|layout| LayoutMode::PointAndSize(layout))
            }
            Anchors::BottomRight => {
                return PointAndSizeLayout::new_corner_anchor(&layout, Alignment::BottomRight).map(|layout| LayoutMode::PointAndSize(layout))
            }
            Anchors::BottomLeft => {
                return PointAndSizeLayout::new_corner_anchor(&layout, Alignment::BottomLeft).map(|layout| LayoutMode::PointAndSize(layout))
            }
            Anchors::LeftRight => return LeftRightAnchorsLayout::new(&layout).map(|layout| LayoutMode::LeftRightAnchors(layout)),
            Anchors::TopBottom => return TopBottomAnchorsLayout::new(&layout).map(|layout| LayoutMode::TopBottomAnchors(layout)),
            Anchors::LeftTopRight => return LeftTopRightAnchorsLayout::new(&layout).map(|layout| LayoutMode::LeftTopRightAnchors(layout)),
            Anchors::LeftBottomRight => return LeftBottomRightAnchorsLayout::new(&layout).map(|layout| LayoutMode::LeftBottomRightAnchors(layout)),
            Anchors::TopLeftBottom => return TopLeftBottomAnchorsLayout::new(&layout).map(|layout| LayoutMode::TopLeftBottomAnchors(layout)),
            Anchors::TopRightBottom => return TopRightBottomAnchorsLayout::new(&layout).map(|layout| LayoutMode::TopRightBottomAnchors(layout)),
            Anchors::All => return AllAnchorsLayout::new(&layout).map(|layout| LayoutMode::AllAnchors(layout)),
            Anchors::Left | Anchors::Right | Anchors::Top | Anchors::Bottom => return Err(Error::SingleAnchor),
            Anchors::None => {}
        }
        // diffrent errors
        if layout.x.is_some() && layout.y.is_none() {
            return Err(Error::XWithoutY);
        }
        if layout.x.is_none() && layout.y.is_some() {
            return Err(Error::YWithoutX);
        }
        if layout.pivot.is_some() {
            return Err(Error::PivotWithoutXorY)
        }
        return Err(Error::InvalidLayoutRule);
    }
}

impl Default for LayoutMode {
    fn default() -> LayoutMode {
        LayoutMode::Absolute(AbsoluteLayout::new(0, 0, 0, 0))
    }
}

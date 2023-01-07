use super::layout_mode::*;

pub struct Layout<'a> {
    format: &'a str,
}

impl Layout<'_> {
    pub fn new(format: &str) -> Layout {
        Layout { format: format }
    }
}

pub(crate) struct ControlLayout {
    mode: LayoutMode,
    x: i32,
    y: i32,
    width: u16,
    height: u16,
    min_width: u16,
    max_width: u16,
    min_height: u16,
    max_height: u16,
}

impl ControlLayout {
    pub(crate) fn new(format: &str) -> ControlLayout {
        ControlLayout {
            mode: LayoutMode::new(format),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            min_width: 1,
            min_height: 1,
            max_width: u16::MAX,
            max_height: u16::MAX,
        }
    }
    #[inline]
    pub(crate) fn resize(&mut self, width: u16, height: u16) {
        self.width = width.clamp(self.min_width, self.max_width);
        self.height = height.clamp(self.min_height, self.max_height);
    }
    #[inline]
    pub(crate) fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub(crate) fn update(&mut self, parent_width: u16, parent_height: u16) {
        match self.mode {
            LayoutMode::PointAndSize(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::LeftRightAnchors(layout_mode) => todo!(),
            LayoutMode::TopBottomAnchors(layout_mode) => todo!(),
            LayoutMode::LeftTopRightAnchors(layout_mode) => todo!(),
            LayoutMode::LeftBottomRightAnchors(layout_mode) => todo!(),
            LayoutMode::TopLeftBottomAnchors(layout_mode) => todo!(),
            LayoutMode::TopRightBottomAnchors(layout_mode) => todo!(),
            LayoutMode::LeftTopRightBottomAnchors(layout_mode) => todo!(),
        }
        /*
        LayoutMetricData md;
        Graphics::Size sz;

        if (controlParent == nullptr)
            controlParent = this->Parent;
        if (controlParent != nullptr)
        {
            sz = controlParent->GetClientSize();
        }
        else
        {
            CHECK(Application::GetDesktopSize(sz), false, "Fail to get desktop size !");
        }
        // translate values - X & Y Axes
        md.X            = this->Layout.Format.X.ToInt(sz.Width);
        md.Y            = this->Layout.Format.Y.ToInt(sz.Height);
        md.AnchorLeft   = this->Layout.Format.AnchorLeft.ToInt(sz.Width);
        md.AnchorTop    = this->Layout.Format.AnchorTop.ToInt(sz.Height);
        md.AnchorRight  = this->Layout.Format.AnchorRight.ToInt(sz.Width);
        md.AnchorBottom = this->Layout.Format.AnchorBottom.ToInt(sz.Height);
        md.Width        = this->Layout.Format.Width.ToInt(sz.Width);
        md.Height       = this->Layout.Format.Height.ToInt(sz.Height);

        // copy align & anchor
        md.Align       = this->Layout.Format.Align;
        md.Anchor      = this->Layout.Format.Anchor;
        md.ParentWidth = sz.Width;
        md.ParentHeigh = sz.Height;

        // compute position
        switch (this->Layout.Format.LayoutMode)
        {
        case LayoutFormatMode::PointAndSize:
            return RecomputeLayout_PointAndSize(md);
        case LayoutFormatMode::LeftRightAnchorsAndHeight:
            return RecomputeLayout_LeftRightAnchorsAndHeight(md);
        case LayoutFormatMode::TopBottomAnchorsAndWidth:
            return RecomputeLayout_TopBottomAnchorsAndWidth(md);
        case LayoutFormatMode::LeftTopRightAnchorsAndHeight:
            SetControlSize(md.ParentWidth - (md.AnchorLeft + md.AnchorRight), md.Height);
            this->Layout.X = md.AnchorLeft;
            this->Layout.Y = md.AnchorTop;
            return true;
        case LayoutFormatMode::LeftBottomRightAnchorsAndHeight:
            SetControlSize(md.ParentWidth - (md.AnchorLeft + md.AnchorRight), md.Height);
            this->Layout.X = md.AnchorLeft;
            this->Layout.Y = md.ParentHeigh - (md.AnchorBottom + this->Layout.Height);
            return true;
        case LayoutFormatMode::TopLeftBottomAnchorsAndWidth:
            SetControlSize(md.Width, md.ParentHeigh - (md.AnchorTop + md.AnchorBottom));
            this->Layout.X = md.AnchorLeft;
            this->Layout.Y = md.AnchorTop;
            return true;
        case LayoutFormatMode::TopRightBottomAnchorsAndWidth:
            SetControlSize(md.Width, md.ParentHeigh - (md.AnchorTop + md.AnchorBottom));
            this->Layout.X = md.ParentWidth - (md.AnchorRight + this->Layout.Width);
            this->Layout.Y = md.AnchorTop;
            return true;
        case LayoutFormatMode::LeftTopRightBottomAnchors:
            SetControlSize(
                  md.ParentWidth - (md.AnchorLeft + md.AnchorRight), md.ParentHeigh - (md.AnchorTop + md.AnchorBottom));
            this->Layout.X = md.AnchorLeft;
            this->Layout.Y = md.AnchorTop;
            return true;
        default:
            RETURNERROR(false, "Unknwon layout format mode: %d", (int) this->Layout.Format.LayoutMode);
        }


            */
    }
}

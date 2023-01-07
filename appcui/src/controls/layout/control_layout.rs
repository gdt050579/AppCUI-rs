use super::LayoutMode;

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
    #[inline]
    pub(crate) fn get_width(&self) -> u16 {
        self.width
    }
    #[inline]
    pub(crate) fn get_heght(&self) -> u16 {
        self.height
    }
    pub(crate) fn update(&mut self, parent_width: u16, parent_height: u16) {
        match self.mode {
            LayoutMode::PointAndSize(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::LeftRightAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::TopBottomAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::LeftTopRightAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::LeftBottomRightAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::TopLeftBottomAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::TopRightBottomAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
            LayoutMode::AllAnchors(layout_mode) => {
                layout_mode.update_control_layout(self, parent_width, parent_height)
            }
        }
        /*
        // compute position
        switch (this->Layout.Format.LayoutMode)
        {


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

#[derive(Copy, Clone, Debug)]
pub(crate) struct ClipArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    visible: bool,
}

impl ClipArea {
    #[inline]
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> ClipArea {
        ClipArea {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
            visible: (left <= right) && (top < bottom),
        }
    }
    #[inline]
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.visible = (left <= right) && (top <= bottom);
    }
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        return self.visible
            && (x >= self.left)
            && (x <= self.right)
            && (y >= self.top)
            && (y <= self.bottom);
    }
    #[inline]
    pub fn contains_y(&self, y: i32) -> bool {
        return self.visible && (y >= self.top) && (y <= self.bottom);
    }
    #[inline]
    pub fn intersect_with(&mut self, clip: &ClipArea) {
        self.set(
            i32::max(self.left, clip.left),
            i32::max(self.top, clip.top),
            i32::min(self.right, clip.right),
            i32::min(self.bottom, clip.bottom),
        )
    }
    fn set_relative_to_parent(
        &mut self,
        parent: &ClipArea,
        x: i32,
        y: i32,
        width: u16,
        height: u16,
    ) {
        // this->ScreenPosition.X = parent.ScreenPosition.X + x;
        // this->ScreenPosition.Y = parent.ScreenPosition.Y + y;
        // if parent.visible == false {
        //     self.visible = false;
        //     return;
        // }

        // if (this->ScreenPosition.X >= parent.ClipRect.X)
        // {
        //     this->ClipRect.X = this->ScreenPosition.X;
        // }
        // else
        // {
        //     this->ClipRect.X = parent.ClipRect.X;
        //     width -= (parent.ClipRect.X - this->ScreenPosition.X);
        // }
        // if (this->ScreenPosition.Y >= parent.ClipRect.Y)
        // {
        //     this->ClipRect.Y = this->ScreenPosition.Y;
        // }
        // else
        // {
        //     this->ClipRect.Y = parent.ClipRect.Y;
        //     height -= (parent.ClipRect.Y - this->ScreenPosition.Y);
        // }

        // if ((width > 0) && (height > 0))
        // {
        //     int tmp = parent.ClipRect.X + parent.ClipRect.Width;
        //     if ((this->ClipRect.X + width) > tmp)
        //         width = tmp - this->ClipRect.X;

        //     tmp = parent.ClipRect.Y + parent.ClipRect.Height;
        //     if ((this->ClipRect.Y + height) > tmp)
        //         height = tmp - this->ClipRect.Y;
        //     if ((width > 0) && (height > 0))
        //     {
        //         this->ClipRect.Width  = width;
        //         this->ClipRect.Height = height;
        //         this->Visible         = true;
        //         return;
        //     }
        // }
        // // invalid clip
        // this->ClipRect.Width = this->ClipRect.Height = 0;
        // this->Visible                                = false;
    }
}

impl Default for ClipArea {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            visible: false,
        }
    }
}

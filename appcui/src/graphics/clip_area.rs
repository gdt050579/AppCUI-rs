#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct ClipArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    visible: bool,
}

impl ClipArea {
    #[inline]
    pub(crate) fn new(left: i32, top: i32, right: i32, bottom: i32) -> ClipArea {
        ClipArea {
            left,
            top,
            right,
            bottom,
            visible: (left <= right) && (top < bottom),
        }
    }
    #[inline]
    pub(crate) fn with_size(x: i32, y: i32, width: u16, height: u16) -> ClipArea {
        ClipArea::new(x, y, x + (width as i32) - 1, y + (height as i32) - 1)
    }
    #[inline]
    pub(crate) fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.visible = (left <= right) && (top <= bottom);
    }
    #[inline]
    pub(crate) fn set_with_size(&mut self, x: i32, y: i32, width: u16, height: u16) {
        self.set(x, y, x + (width as i32) - 1, y + (height as i32) - 1);
    }
    #[inline(always)]
    pub(crate) fn is_visible(&self) -> bool {
        self.visible
    }
    #[inline(always)]
    pub(crate) fn contains(&self, x: i32, y: i32) -> bool {
        self.visible && (x >= self.left) && (x <= self.right) && (y >= self.top) && (y <= self.bottom)
    }
    #[inline]
    pub(crate) fn contains_y(&self, y: i32) -> bool {
        self.visible && (y >= self.top) && (y <= self.bottom)
    }
    #[inline]
    pub(crate) fn intersect_with(&mut self, clip: &ClipArea) {
        self.set(
            i32::max(self.left, clip.left),
            i32::max(self.top, clip.top),
            i32::min(self.right, clip.right),
            i32::min(self.bottom, clip.bottom),
        )
    }
    #[inline]
    pub(crate) fn reduce_margins(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left += left;
        self.right -= right;
        self.top += top;
        self.bottom -= bottom;
        self.visible = (self.left <= self.right) && (self.top <= self.bottom);
    }
}

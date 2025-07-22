use super::{AbsoluteLayout, LayoutMode, Layout};
use crate::graphics::Size;

#[derive(Default)]
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
    #[inline]
    pub(super) fn resize(&mut self, width: u16, height: u16) {
        self.width = width.clamp(self.min_width, self.max_width);
        self.height = height.clamp(self.min_height, self.max_height);
    }
    #[inline]
    pub(crate) fn set_size_bounds(
        &mut self,
        min_width: u16,
        min_height: u16,
        max_width: u16,
        max_height: u16,
    ) {
        if (min_width <= max_width) && (min_height <= max_height) {
            self.min_width = min_width;
            self.min_height = min_height;
            self.max_width = max_width;
            self.max_height = max_height;
        }
    }

    #[inline]
    pub(super) fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    #[inline(always)]
    pub(crate) fn get_width(&self) -> u16 {
        self.width
    }
    #[inline(always)]
    pub(crate) fn get_height(&self) -> u16 {
        self.height
    }
    #[inline(always)]
    pub(crate) fn get_size(&self) -> Size {
        Size{
            width: self.width as u32,
            height: self.height as u32,
        }
    }

    #[inline]
    pub(crate) fn get_x(&self) -> i32 {
        self.x
    }
    #[inline]
    pub(crate) fn get_y(&self) -> i32 {
        self.y
    }
    pub(crate) fn update(&mut self, parent_width: u16, parent_height: u16) {
        match self.mode {
            LayoutMode::Absolute(layout_mode) => layout_mode.update_control_layout(self),
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
    }
    pub(crate) fn layout_resize(&mut self, width: u16, height: u16) {
        match &mut self.mode {
            LayoutMode::Absolute(layout) => {
                layout.width = width;
                layout.height = height;
            }
            _ => {
                self.mode = LayoutMode::Absolute(AbsoluteLayout::new(self.x, self.y, width, height));
            }
        }
    }
    pub(crate) fn layout_set_position(&mut self, x: i32, y: i32) {
        match &mut self.mode {
            LayoutMode::Absolute(layout) => {
                layout.x = x;
                layout.y = y;
            }
            _ => {
                self.mode = LayoutMode::Absolute(AbsoluteLayout::new(x, y, self.get_width(), self.get_height()));
            }
        }
    }
}

impl From<Layout> for ControlLayout {
    fn from(value: Layout) -> Self {
        Self {
            mode: LayoutMode::new(value).unwrap(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            min_width: 1,
            min_height: 1,
            max_width: u16::MAX,
            max_height: u16::MAX,
        }
    }
}   
use super::{AbsoluteLayout, LayoutMode};
use crate::graphics::Size;

/// Creates a new layout instance with the specified format string.
/// 
/// The format string defines how a control should be positioned and sized within its parent container.
/// The format supports several layout modes:
/// 
/// 1. **Absolute Position** (`x`, `y`, `width`, `height`):
///    - Positions control at specific coordinates
///    - Example: `"x:8,y:5,w:33%,h:6"`
///    - Aliases: `w` for `width`, `h` for `height`
/// 
/// 2. **Anchors** (`left`, `right`, `top`, `bottom`):
///    - Positions control relative to parent edges
///    - Supports corner anchors, 3-margin anchors, and 4-margin anchors
///    - Example: `"t:10,r:20,w:50,h:20"`
///    - Aliases: `l` for `left`, `r` for `right`, `t` for `top`, `b` for `bottom`
/// 
/// 3. **Docking** (`dock` or `d`):
///    - Docks control to parent edges or corners
///    - Example: `"d:c,w:30,h:50%"`
///    - Aliases: `d` for `dock`
/// 
/// Additional parameters:
/// - `align` or `a`: Specifies alignment within the layout
///   - Values: `tl` (top-left), `tr` (top-right), `bl` (bottom-left), `br` (bottom-right), `c` (center)
///   - Example: `"x:8,y:5,w:33%,h:6,a:tl"`
/// 
/// Values can be specified in pixels or percentages. When using percentages, the control will
/// automatically adjust its size when the parent size changes.
/// 
/// # Parameters
/// * `format` - A string containing layout parameters in the format `"key:value"` separated by commas
/// 
/// # Examples
/// ```rust
/// use appcui::prelude::*;
/// 
/// // Absolute positioning with alignment
/// let layout = Layout::new("x:8,y:5,w:33%,h:6,a:tl");
/// 
/// // Anchors with short aliases
/// let layout = Layout::new("t:10,r:20,w:50,h:20");
/// 
/// // Docking with short alias
/// let layout = Layout::new("d:c,w:30,h:50%");
/// 
/// // Full anchors with short aliases
/// let layout = Layout::new("l:20,t:7,r:10,b:10");
/// ```
pub struct Layout<'a> {
    pub(in super::super) format: &'a str,
}

impl Layout<'_> {
    pub fn new(format: &str) -> Layout {
        Layout { format }
    }
}

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
    pub(crate) fn new(format: &str) -> ControlLayout {
        ControlLayout {
            mode: LayoutMode::new(format),
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

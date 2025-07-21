use crate::ui::LayoutBuilder;

use super::anchors::Anchors;
use super::Alignment;
use super::Coordinate16;
use super::Dimension16;
use super::Dock;
use super::Pivot;

/// Represents a new layout instance with the specified format string.
///
/// # Examples
/// ```rust
/// use appcui::prelude::*;
///
/// // Absolute positioning with alignment
/// let layout = layout!("x:8,y:5,w:33%,h:6,p:tl");
///
/// // Anchors with short aliases
/// let layout = layout!("t:10,r:20,w:50,h:20");
///
/// // Aligning to parent with short alias
/// let layout = layout!("a:c,w:30,h:50%");
///
/// // Full anchors with short aliases
/// let layout = layout!("l:20,t:7,r:10,b:10");
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Layout {
    pub(super) x: Option<Coordinate16>,
    pub(super) y: Option<Coordinate16>,
    pub(super) width: Option<Dimension16>,
    pub(super) height: Option<Dimension16>,
    pub(super) a_left: Option<Coordinate16>,
    pub(super) a_right: Option<Coordinate16>,
    pub(super) a_top: Option<Coordinate16>,
    pub(super) a_bottom: Option<Coordinate16>,
    pub(super) align: Option<Alignment>,
    pub(super) pivot: Option<Pivot>,
    pub(super) dock: Option<Dock>,
}

impl Layout {
    pub fn absolute(x: i32, y: i32, width: u32, height: u32) -> Self {
        LayoutBuilder::new().x(x).y(y).width(width).height(height).build()
    }
    pub fn fill() -> Self {
        LayoutBuilder::new().dock(Dock::Fill).build()
    }
    pub fn pivot(x: i32, y: i32, width: u32, height: u32, pivot: Pivot) -> Self {
        LayoutBuilder::new().x(x).y(y).width(width).height(height).pivot(pivot).build()
    }
    pub fn aligned(align: Alignment, width: u32, height: u32) -> Self {
        LayoutBuilder::new().width(width).height(height).alignment(align).build()
    }
    pub(super) fn get_anchors(&self) -> Anchors {
        Anchors::new(
            self.a_left.is_some(),
            self.a_top.is_some(),
            self.a_right.is_some(),
            self.a_bottom.is_some(),
        )
    }
}

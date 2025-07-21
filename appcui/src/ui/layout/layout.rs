use super::anchors::Anchors;
use super::Alignment;
use super::Dock;
use super::Pivot;
use super::Coordinate16;
use super::Dimension16;

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
///    - Example: `"a:c,w:30,h:50%"`
///    - Aliases: `d` for `dock`
///
/// Additional parameters:
///   - `align` or `a`: Specifies alignment within the layout
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
    pub(super) fn get_anchors(&self) -> Anchors {
        Anchors::new(
            self.a_left.is_some(),
            self.a_top.is_some(),
            self.a_right.is_some(),
            self.a_bottom.is_some(),
        )
    }    
}

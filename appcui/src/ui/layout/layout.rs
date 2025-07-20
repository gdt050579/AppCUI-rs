use super::LayoutParameters;

pub(super) enum LayoutBuildFormat<'a> {
    String(&'a str),
    Params(LayoutParameters),
}
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
pub struct Layout<'a> {
    pub(super) format: LayoutBuildFormat<'a>,
}

impl Layout<'_> {
    #[inline(always)]
    pub fn new(format: &str) -> Layout {
        Layout {
            format: LayoutBuildFormat::String(format),
        }
    }
    #[inline(always)]
    pub(super) fn with_layout_params(params: LayoutParameters) -> Self {
        Self {
            format: LayoutBuildFormat::Params(params),
        }
    }
}

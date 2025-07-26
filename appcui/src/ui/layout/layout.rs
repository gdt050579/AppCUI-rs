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
#[derive(Debug, Eq, PartialEq, Clone)]
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
    /// Creates a new layout instance with **absolute positioning**.
    ///
    /// In absolute positioning, the control's position is defined by its **top-left corner**
    /// at coordinates `(x, y)`, and its size is specified by `width` and `height`.
    /// The control does not automatically adjust to parent size changes and will remain
    /// at the specified fixed position.
    ///
    /// This is the simplest and most predictable layout mode, typically used when
    /// you need full control over the element's exact position on the screen.
    ///
    /// # Parameters
    /// * `x` - The X coordinate of the control's top-left corner (absolute or relative to the parent).
    /// * `y` - The Y coordinate of the control's top-left corner.
    /// * `width` - The width of the control (in character cells).
    /// * `height` - The height of the control (in character cells).
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Creates a control positioned at (8, 5) with a size of 33x6 characters.
    /// let layout = Layout::absolute(8, 5, 33, 6);
    /// ```
    pub fn absolute(x: i32, y: i32, width: u32, height: u32) -> Self {
        LayoutBuilder::new().x(x).y(y).width(width).height(height).build()
    }
    /// Creates a new layout instance that completely **fills the parent container**.
    ///
    /// This layout mode uses [`Dock::Fill`], meaning the control will occupy **all available space**
    /// inside its parent. It is typically used for elements like panels or views that should
    /// expand and resize automatically with the parent container.
    ///
    /// Unlike `absolute` or `pivot` layouts, a fill layout automatically adapts
    /// when the parent size changes, ensuring the control always covers the entire area.
    ///
    /// # Behavior
    /// - The control will **ignore its own width and height settings**.
    /// - It will stretch horizontally and vertically to match the parent’s size.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Creates a layout that occupies the entire parent container.
    /// let layout = Layout::fill();
    /// ```
    pub fn fill() -> Self {
        LayoutBuilder::new().dock(Dock::Fill).build()
    }

    /// Creates a new layout instance using a **pivot point**.
    ///
    /// A pivot point determines how the control is positioned relative to a given reference point `(x, y)`.
    /// Unlike absolute positioning (which always treats `(x, y)` as the top-left corner), a pivot allows
    /// `(x, y)` to represent any logical point of the control (e.g., center, bottom-right).
    ///
    /// The `pivot` parameter specifies which part of the control is aligned to the `(x, y)` coordinates:
    /// - `Pivot::TopLeft` → `(x, y)` will be the top-left corner of the control.
    /// - `Pivot::TopCenter` → `(x, y)` will align with the middle of the top edge.
    /// - `Pivot::Center` → `(x, y)` will be the center of the control.
    /// - `Pivot::BottomRight` → `(x, y)` will be the bottom-right corner.
    ///
    /// This is useful for positioning elements in a more natural way without manually adjusting offsets.
    ///
    /// # Parameters
    /// * `x` - The reference X coordinate (absolute or relative to the parent).
    /// * `y` - The reference Y coordinate.
    /// * `width` - The width of the control.
    /// * `height` - The height of the control.
    /// * `pivot` - A [`Pivot`] value indicating which point of the control attaches to `(x, y)`.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Create a layout for a 6x6 control, with its center positioned at (10, 10)
    /// let layout = Layout::pivot(10, 10, 6, 6, Pivot::Center);
    ///
    /// // Another example: align bottom-right corner at (50, 20)
    /// let layout_br = Layout::pivot(50, 20, 12, 5, Pivot::BottomRight);
    /// ```
    pub fn pivot(x: i32, y: i32, width: u32, height: u32, pivot: Pivot) -> Self {
        LayoutBuilder::new().x(x).y(y).width(width).height(height).pivot(pivot).build()
    }

    /// Creates a new layout instance that positions the control using **alignment**
    /// relative to its parent container.
    ///
    /// The `align` parameter determines which position within the parent the control
    /// will occupy. This can be one of nine predefined positions:
    ///
    /// - `Alignment::TopLeft`
    /// - `Alignment::TopCenter`
    /// - `Alignment::TopRight`
    /// - `Alignment::CenterLeft`
    /// - `Alignment::Center`
    /// - `Alignment::CenterRight`
    /// - `Alignment::BottomLeft`
    /// - `Alignment::BottomCenter`
    /// - `Alignment::BottomRight`
    ///
    /// The control’s size is fixed using `width` and `height`. Unlike a dock or fill
    /// layout, an aligned control does **not stretch**; it simply places the control
    /// at the specified alignment within its parent.
    ///
    /// # Parameters
    /// * `align` - An [`Alignment`] value specifying the position relative to the parent.
    /// * `width` - The width of the control (in character cells).
    /// * `height` - The height of the control (in character cells).
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Creates a control aligned to the bottom-right corner of the parent with a fixed size.
    /// let layout = Layout::aligned(Alignment::BottomRight, 20, 5);
    /// ```    
    pub fn aligned(align: Alignment, width: u32, height: u32) -> Self {
        LayoutBuilder::new().width(width).height(height).alignment(align).build()
    }
    pub(super) fn anchors(&self) -> Anchors {
        Anchors::new(
            self.a_left.is_some(),
            self.a_top.is_some(),
            self.a_right.is_some(),
            self.a_bottom.is_some(),
        )
    }
}

use super::Alignment;
use super::Coordinate16;
use super::Dimension16;
use super::Error;
use super::Layout;
use super::LayoutMode;
use super::Pivot;
use crate::ui::Dock;

pub struct LayoutBuilder {
    pub(super) inner_layout: Layout,
}

impl LayoutBuilder {
    /// Creates a new layout builder instance without any settings.
    /// You still need to call any other method to set the layout.
    pub fn new() -> Self {
        Self {
            inner_layout: Layout {
                x: None,
                y: None,
                width: None,
                height: None,
                a_left: None,
                a_right: None,
                a_top: None,
                a_bottom: None,
                align: None,
                pivot: None,
                dock: None,
            },
        }
    }

    /// Sets the horizontal position (`x` coordinate) of the control within its parent container.
    ///
    /// This value determines the control's position on the **X axis** relative to its parent.
    /// The meaning of this value depends on the layout mode:
    ///
    /// - For **absolute** and **pivot** layouts, `x` represents the reference horizontal position.
    /// - For **absolute** layout, `(x, y)` is the **top-left corner** of the control.
    /// - For **pivot** layout, `(x, y)` acts as a reference point combined with a [`Pivot`] value
    ///   that determines which part of the control is attached to this point.
    /// - For **dock** and **aligned** layouts, this property is typically ignored.
    ///
    /// The value can be provided as:
    /// - An **integer** (absolute position in character cells).
    /// - A **floating-point value usually between `0.0` and `1.0`** to specify a percentage of the parent width.
    ///   For example, `0.5` means **50% of the parent's width**.
    ///
    /// # Parameters
    /// * `x` - The X coordinate of the control (absolute or relative).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing different numeric types or percentage values.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Absolute layout: control starts at x = 10, y = 5
    /// let layout = LayoutBuilder::new()
    ///     .x(10)
    ///     .y(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Pivot layout: center of the control is at x = 50, y = 10
    /// let layout_pivot = LayoutBuilder::new()
    ///     .x(50)
    ///     .y(10)
    ///     .width(10)
    ///     .height(4)
    ///     .pivot(Pivot::Center)
    ///     .build();
    ///
    /// // Percentage-based positioning: x = 50% of parent's width
    /// let layout_percentage = LayoutBuilder::new()
    ///     .x(0.5)   // 50% from parent's left edge
    ///     .y(0.0)   // top of the parent
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    /// ```
    pub fn x<T>(mut self, x: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.x = Some(x.into());
        self
    }

    /// Sets the vertical position (`y` coordinate) of the control within its parent container.
    ///
    /// This value determines the control's position on the **Y axis** relative to its parent.
    /// The meaning of this value depends on the layout mode:
    ///
    /// - For **absolute** and **pivot** layouts, `y` represents the reference vertical position.
    /// - For **absolute** layout, `(x, y)` is the **top-left corner** of the control.
    /// - For **pivot** layout, `(x, y)` acts as a reference point combined with a [`Pivot`] value
    ///   that determines which part of the control is attached to this point.
    /// - For **dock** and **aligned** layouts, this property is typically ignored.
    ///
    /// The value can be provided as:
    /// - An **integer** (absolute position in character cells).
    /// - A **floating-point value usually between `0.0` and `1.0`** to specify a percentage of the parent height.
    ///   For example, `0.5` means **50% of the parent's height**.
    ///
    /// # Parameters
    /// * `y` - The Y coordinate of the control (absolute or relative).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing different numeric types or percentage values.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Absolute layout: control starts at x = 10, y = 5
    /// let layout = LayoutBuilder::new()
    ///     .x(10)
    ///     .y(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Pivot layout: center of the control is at (50, 10)
    /// let layout_pivot = LayoutBuilder::new()
    ///     .x(50)
    ///     .y(10)
    ///     .width(10)
    ///     .height(4)
    ///     .pivot(Pivot::Center)
    ///     .build();
    ///
    /// // Percentage-based positioning: y = 50% of parent's height
    /// let layout_percentage = LayoutBuilder::new()
    ///     .x(0.0)   // left edge of the parent
    ///     .y(0.5)   // 50% from parent's top edge
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    /// ```
    pub fn y<T>(mut self, y: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.y = Some(y.into());
        self
    }

    /// Sets the width of the control.
    ///
    /// This value determines how much horizontal space the control occupies.  
    /// The interpretation depends on the layout mode:
    ///
    /// - For **absolute**, **pivot**, and **aligned** layouts, `width` defines the control's fixed width in character cells.
    /// - For **dock** layouts (e.g., `Dock::Left`, `Dock::Right`), the `width` value is respected along the docking axis.
    /// - For **fill** layouts, this property is ignored since the control stretches to fill the entire parent width.
    /// - For **anchored** layouts:
    ///   - If both `left_anchor` and `right_anchor` are set, `width()` is ignored because the width is calculated dynamically.
    ///   - If only one anchor is set, the specified `width` is used.
    ///
    /// The value can be provided as:
    /// - An **unsigned integer type** (`u8`, `u16`, `u32`, `u64`) representing the width in character cells.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent width.
    ///   For example, `0.5` means **50% of the parent's width**.
    ///
    /// Negative values are **not allowed**.
    ///
    /// # Parameters
    /// * `width` - The control's width (absolute or relative).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Dimension16>`, allowing unsigned integers for absolute sizes
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Absolute width: 20 characters
    /// let layout = LayoutBuilder::new()
    ///     .x(10)
    ///     .y(5)
    ///     .width(20)    // absolute width
    ///     .height(5)
    ///     .build();
    ///
    /// // Percentage-based width: 50% of the parent's width
    /// let layout_percentage = LayoutBuilder::new()
    ///     .x(0.0)       // left edge
    ///     .y(0.0)       // top edge
    ///     .width(0.5)   // 50% width
    ///     .height(10)
    ///     .build();
    /// ```
    pub fn width<T>(mut self, width: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.inner_layout.width = Some(width.into());
        self
    }

    /// Sets the height of the control.
    ///
    /// This value determines how much vertical space the control occupies.  
    /// The interpretation depends on the layout mode:
    ///
    /// - For **absolute**, **pivot**, and **aligned** layouts, `height` defines the control's fixed height in character cells.
    /// - For **dock** layouts (e.g., `Dock::Top`, `Dock::Bottom`), the `height` value is respected along the docking axis.
    /// - For **fill** layouts, this property is ignored since the control stretches to fill the entire parent height.
    /// - For **anchored** layouts:
    ///   - If both `top_anchor` and `bottom_anchor` are set, `height()` is ignored because the height is calculated dynamically.
    ///   - If only one anchor is set, the specified `height` is used.
    ///
    /// The value can be provided as:
    /// - An **unsigned integer type** (`u8`, `u16`, `u32`, `u64`) representing the height in character cells.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent height.
    ///   For example, `0.5` means **50% of the parent's height**.
    ///
    /// Negative values are **not allowed**.
    ///
    /// # Parameters
    /// * `height` - The control's height (absolute or relative).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Dimension16>`, allowing unsigned integers for absolute sizes
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Absolute height: 10 characters
    /// let layout = LayoutBuilder::new()
    ///     .x(10)
    ///     .y(5)
    ///     .width(20)
    ///     .height(10)    // absolute height
    ///     .build();
    ///
    /// // Percentage-based height: 50% of the parent's height
    /// let layout_percentage = LayoutBuilder::new()
    ///     .x(0.0)        // left edge
    ///     .y(0.0)        // top edge
    ///     .width(20)
    ///     .height(0.5)   // 50% height
    ///     .build();
    /// ```
    pub fn height<T>(mut self, height: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.inner_layout.height = Some(height.into());
        self
    }

    /// Sets the **alignment** for the control, which determines its position relative to the parent
    /// container when using alignment-based layout.
    ///
    /// Alignment is a positioning strategy where the control is placed at a predefined location inside
    /// the parent (such as top-left, center, or bottom-right) while keeping its size fixed.
    /// The possible alignment values are defined by the [`Alignment`] enum:
    ///
    /// - [`Alignment::TopLeft`]     – Control is positioned at the top-left corner of the parent.
    /// - [`Alignment::TopCenter`]   – Control is horizontally centered at the top edge.
    /// - [`Alignment::TopRight`]    – Control is positioned at the top-right corner.
    /// - [`Alignment::CenterLeft`]  – Control is vertically centered on the left edge.
    /// - [`Alignment::Center`]      – Control is centered both horizontally and vertically.
    /// - [`Alignment::CenterRight`] – Control is vertically centered on the right edge.
    /// - [`Alignment::BottomLeft`]  – Control is positioned at the bottom-left corner.
    /// - [`Alignment::BottomCenter`]- Control is horizontally centered at the bottom edge.
    /// - [`Alignment::BottomRight`] – Control is positioned at the bottom-right corner.
    ///
    /// # Behavior
    /// - The control's size is **not stretched**; it remains as specified by `width()` and `height()`.
    /// - Alignment **overrides** manual coordinates (`x()`, `y()`) and anchors.
    /// - If combined with `dock()` or `pivot()`, alignment will be ignored.
    ///
    /// # Parameters
    /// * `align` - A value from the [`Alignment`] enum indicating the desired alignment position.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Align the control to the top-left corner of the parent
    /// let layout_top_left = LayoutBuilder::new()
    ///     .alignment(Alignment::TopLeft)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Align the control to the center of the parent
    /// let layout_center = LayoutBuilder::new()
    ///     .alignment(Alignment::Center)
    ///     .width(30)
    ///     .height(10)
    ///     .build();
    ///
    /// // Align the control to the bottom-right corner
    /// let layout_bottom_right = LayoutBuilder::new()
    ///     .alignment(Alignment::BottomRight)
    ///     .width(15)
    ///     .height(4)
    ///     .build();
    /// ```
    pub fn alignment(mut self, align: Alignment) -> Self {
        self.inner_layout.align = Some(align);
        self
    }

    /// Specifies which part of the control will align with the reference point or anchors
    /// when positioning the control using pivot-based logic.
    ///
    /// The `pivot()` method does **not** set the pivot point coordinates; that is determined by `x()` and `y()`.
    /// Instead, it defines which part of the control (e.g., top-left, center, bottom-right) will align
    /// with the reference point provided by `x()` and `y()`, or will be used in combination with
    /// anchors to resolve vertical/horizontal placement ambiguities.
    ///
    /// ## When is `pivot()` used?
    /// - **Pivot Layout**: When both `x()` and `y()` are set, `pivot()` defines which part of the control
    ///   is positioned at that point.
    /// - **Mixed Layouts (Anchors + Size)**:
    ///   - If the control is anchored horizontally (e.g., `left_anchor()` and `right_anchor()`),
    ///     its horizontal position and width are determined by the anchors.
    ///     In this case, `pivot()` is used for vertical alignment relative to `y()`.
    ///   - Similarly, if the control is anchored vertically (e.g., `top_anchor()` and `bottom_anchor()`),
    ///     `pivot()` is used for horizontal alignment relative to `x()`.
    ///
    /// The possible pivot values are defined by the [`Pivot`] enum:
    /// - [`Pivot::TopLeft`]      – Align the top-left corner.
    /// - [`Pivot::TopCenter`]    – Align the top edge centered horizontally.
    /// - [`Pivot::TopRight`]     – Align the top-right corner.
    /// - [`Pivot::CenterLeft`]   – Align the left edge centered vertically.
    /// - [`Pivot::Center`]       – Align the center of the control.
    /// - [`Pivot::CenterRight`]  – Align the right edge centered vertically.
    /// - [`Pivot::BottomLeft`]   – Align the bottom-left corner.
    /// - [`Pivot::BottomCenter`] – Align the bottom edge centered horizontally.
    /// - [`Pivot::BottomRight`]  – Align the bottom-right corner.
    ///
    /// # Behavior
    /// - Requires `x()` and/or `y()` for absolute reference points.
    /// - Does **not** affect control size; size is determined by `width()` and `height()`
    ///   or by anchors if both edges are specified.
    /// - If combined with `dock()` or `alignment()`, this setting is ignored.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Center the control at (50, 20)
    /// let layout_centered = LayoutBuilder::new()
    ///     .x(50)
    ///     .y(20)
    ///     .width(20)
    ///     .height(10)
    ///     .pivot(Pivot::Center)
    ///     .build();
    ///
    /// // Top-left corner at (5, 5)
    /// let layout_top_left = LayoutBuilder::new()
    ///     .x(5)
    ///     .y(5)
    ///     .width(15)
    ///     .height(6)
    ///     .pivot(Pivot::TopLeft)
    ///     .build();
    ///
    /// // Horizontal anchors with vertical pivot
    /// // (width is determined by left/right anchors, pivot applies vertically)
    /// let layout_mixed = LayoutBuilder::new()
    ///     .left_anchor(10)
    ///     .right_anchor(10)
    ///     .y(50)
    ///     .height(5)
    ///     .pivot(Pivot::Center)
    ///     .build();
    /// ```
    pub fn pivot(mut self, pivot: Pivot) -> Self {
        self.inner_layout.pivot = Some(pivot);
        self
    }

    /// Sets the **dock mode** for the control, which determines how it is positioned and stretched
    /// relative to its parent container.
    ///
    /// Docking is a layout strategy where a control is attached to one side of its parent or fills
    /// the entire available space. The control is resized automatically when the parent is resized.
    /// The possible docking options are defined by the [`Dock`] enum:
    ///
    /// - [`Dock::Left`]   – Control is attached to the left edge of the parent and stretches vertically.
    /// - [`Dock::Right`]  – Control is attached to the right edge of the parent and stretches vertically.
    /// - [`Dock::Top`]    – Control is attached to the top edge of the parent and stretches horizontally.
    /// - [`Dock::Bottom`] – Control is attached to the bottom edge of the parent and stretches horizontally.
    /// - [`Dock::Fill`]   – Control fills the entire remaining space of the parent.
    ///
    /// # Behavior
    /// - Docking overrides most manual positioning settings (`x()`, `y()`, `alignment()`, `anchors()`).
    /// - For side docking (`Left`, `Right`, `Top`, `Bottom`):
    ///   - The control **respects its size** on the docking axis if explicitly set.
    ///   - The control **stretches** on the opposite axis to fill the parent.
    /// - For `Dock::Fill`, the control **completely fills** the parent container, ignoring `width()` and `height()`.
    ///
    /// # Parameters
    /// * `dock` - A value from the [`Dock`] enum indicating how the control should be docked.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Dock the control to the left side of the parent
    /// let layout_left = LayoutBuilder::new()
    ///     .dock(Dock::Left)
    ///     .width(20)   // width respected, height will stretch
    ///     .build();
    ///
    /// // Dock the control to the bottom of the parent
    /// let layout_bottom = LayoutBuilder::new()
    ///     .dock(Dock::Bottom)
    ///     .height(3)   // height respected, width will stretch
    ///     .build();
    ///
    /// // Fill the entire parent area
    /// let layout_fill = LayoutBuilder::new()
    ///     .dock(Dock::Fill)
    ///     .build();
    /// ```
    pub fn dock(mut self, dock: Dock) -> Self {
        self.inner_layout.dock = Some(dock);
        self
    }

    /// Sets the **left anchor** for the control, which defines its distance from the **left edge of the parent container**.
    ///
    /// Anchors allow controls to automatically adjust their size and position when the parent is resized.
    /// The behavior of `left_anchor()` depends on the combination of anchors set:
    ///
    /// - If **only `left_anchor` is set**, the control's left edge will stay fixed at the specified distance from the parent's left edge.
    /// - If both `left_anchor` and `right_anchor` are set, the control will **stretch horizontally** when the parent resizes, and
    ///   the `width()` setting will be ignored because the width is calculated dynamically.
    /// - If `left_anchor` is combined with **dock** or **fill** modes, the anchor is ignored.
    ///
    /// The value can be provided as:
    /// - A **signed integer type** (`i8`, `i16`, `i32`, `i64`) representing the absolute distance in character cells.
    ///   - Negative values are allowed and will position the control beyond the parent's left edge.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent's width.
    ///   For example, `0.1` means **10% of the parent's width**.
    ///
    /// # Parameters
    /// * `left_anchor` - Distance from the parent's left edge (absolute or percentage).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing signed integers for absolute positions
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Control anchored 5 cells from the parent's left edge
    /// let layout = LayoutBuilder::new()
    ///     .left_anchor(5)
    ///     .top_anchor(2)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored -3 cells (outside the parent's left edge)
    /// let layout_negative = LayoutBuilder::new()
    ///     .left_anchor(-3)
    ///     .top_anchor(2)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored at 10% from the parent's left edge
    /// let layout_percentage = LayoutBuilder::new()
    ///     .left_anchor(0.1)  // 10% from the parent's left edge
    ///     .right_anchor(0.1) // 10% from the parent's right edge
    ///     .height(5)
    ///     .build();
    /// ```
    pub fn left_anchor<T>(mut self, left_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_left = Some(left_anchor.into());
        self
    }

    /// Sets the **right anchor** for the control, which defines its distance from the **right edge of the parent container**.
    ///
    /// Anchors allow controls to automatically adjust their size and position when the parent is resized.
    /// The behavior of `right_anchor()` depends on the combination of anchors set:
    ///
    /// - If **only `right_anchor` is set**, the control's right edge will stay fixed at the specified distance from the parent's right edge.
    /// - If both `left_anchor` and `right_anchor` are set, the control will **stretch horizontally** when the parent resizes, and
    ///   the `width()` setting will be ignored because the width is calculated dynamically.
    /// - If `right_anchor` is combined with **dock** or **fill** modes, the anchor is ignored.
    ///
    /// The value can be provided as:
    /// - A **signed integer type** (`i8`, `i16`, `i32`, `i64`) representing the absolute distance in character cells.
    ///   - Negative values are allowed and will position the control beyond the parent's right edge.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent's width.
    ///   For example, `0.1` means **10% of the parent's width**.
    ///
    /// # Parameters
    /// * `right_anchor` - Distance from the parent's right edge (absolute or percentage).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing signed integers for absolute positions
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Control anchored 5 cells from the parent's right edge
    /// let layout = LayoutBuilder::new()
    ///     .right_anchor(5)
    ///     .top_anchor(2)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored -4 cells beyond the parent's right edge (overflow)
    /// let layout_negative = LayoutBuilder::new()
    ///     .right_anchor(-4)
    ///     .top_anchor(2)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored at 15% from the parent's right edge
    /// let layout_percentage = LayoutBuilder::new()
    ///     .right_anchor(0.15) // 15% from the parent's right edge
    ///     .left_anchor(0.05)  // 5% from the parent's left edge
    ///     .height(5)
    ///     .build();
    /// ```
    pub fn right_anchor<T>(mut self, right_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_right = Some(right_anchor.into());
        self
    }

    /// Sets the **top anchor** for the control, which defines its distance from the **top edge of the parent container**.
    ///
    /// Anchors allow controls to automatically adjust their size and position when the parent is resized.
    /// The behavior of `top_anchor()` depends on the combination of anchors set:
    ///
    /// - If **only `top_anchor` is set**, the control's top edge will stay fixed at the specified distance from the parent's top edge.
    /// - If both `top_anchor` and `bottom_anchor` are set, the control will **stretch vertically** when the parent resizes, and
    ///   the `height()` setting will be ignored because the height is calculated dynamically.
    /// - If `top_anchor` is combined with **dock** or **fill** modes, the anchor is ignored.
    ///
    /// The value can be provided as:
    /// - A **signed integer type** (`i8`, `i16`, `i32`, `i64`) representing the absolute distance in character cells.
    ///   - Negative values are allowed and will position the control above the parent's top edge.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent's height.
    ///   For example, `0.2` means **20% of the parent's height**.
    ///
    /// # Parameters
    /// * `top_anchor` - Distance from the parent's top edge (absolute or percentage).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing signed integers for absolute positions
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Control anchored 3 cells from the parent's top edge
    /// let layout = LayoutBuilder::new()
    ///     .top_anchor(3)
    ///     .left_anchor(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored -2 cells above the parent's top edge (overflow)
    /// let layout_negative = LayoutBuilder::new()
    ///     .top_anchor(-2)
    ///     .left_anchor(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored at 10% from the parent's top edge
    /// let layout_percentage = LayoutBuilder::new()
    ///     .top_anchor(0.10)    // 10% from the parent's top edge
    ///     .bottom_anchor(0.05) // 5% from the parent's bottom edge
    ///     .width(30)
    ///     .build();
    /// ```
    pub fn top_anchor<T>(mut self, top_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_top = Some(top_anchor.into());
        self
    }

    /// Sets the **bottom anchor** for the control, which defines its distance from the **bottom edge of the parent container**.
    ///
    /// Anchors allow controls to automatically adjust their size and position when the parent is resized.
    /// The behavior of `bottom_anchor()` depends on the combination of anchors set:
    ///
    /// - If **only `bottom_anchor` is set**, the control's bottom edge will stay fixed at the specified distance from the parent's bottom edge.
    /// - If both `top_anchor` and `bottom_anchor` are set, the control will **stretch vertically** when the parent resizes, and
    ///   the `height()` setting will be ignored because the height is calculated dynamically.
    /// - If `bottom_anchor` is combined with **dock** or **fill** modes, the anchor is ignored.
    ///
    /// The value can be provided as:
    /// - A **signed integer type** (`i8`, `i16`, `i32`, `i64`) representing the absolute distance in character cells.
    ///   - Negative values are allowed and will position the control below the parent's bottom edge.
    /// - A **floating-point value between `0.0` and `1.0` (inclusive)** to specify a percentage of the parent's height.
    ///   For example, `0.2` means **20% of the parent's height**.
    ///
    /// # Parameters
    /// * `bottom_anchor` - Distance from the parent's bottom edge (absolute or percentage).
    ///
    /// # Type Constraints
    /// * `T` must implement `Into<Coordinate16>`, allowing signed integers for absolute positions
    ///   or `f32` values between `0.0` and `1.0` for percentages.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Control anchored 4 cells from the parent's bottom edge
    /// let layout = LayoutBuilder::new()
    ///     .bottom_anchor(4)
    ///     .left_anchor(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored -3 cells below the parent's bottom edge (overflow)
    /// let layout_negative = LayoutBuilder::new()
    ///     .bottom_anchor(-3)
    ///     .left_anchor(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Control anchored at 15% from the parent's bottom edge
    /// let layout_percentage = LayoutBuilder::new()
    ///     .top_anchor(0.10)     // 10% from top
    ///     .bottom_anchor(0.15)  // 15% from bottom
    ///     .width(30)
    ///     .build();
    /// ```
    pub fn bottom_anchor<T>(mut self, bottom_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_bottom = Some(bottom_anchor.into());
        self
    }

    /// Finalizes the layout configuration and returns a [`Layout`] instance.
    ///
    /// This method collects all the parameters set using the builder methods (such as
    /// `x()`, `y()`, `width()`, `height()`, `alignment()`, `pivot()`, `dock()`, and anchors)
    /// and constructs a `Layout` object that can be applied to a control.
    ///
    /// ## Behavior
    /// - Combines all provided settings (absolute coordinates, size, anchors, pivot, alignment, dock)
    ///   into a single `Layout` configuration.
    /// - Validates the consistency of the provided parameters before returning the result.
    ///
    /// ## Panic Conditions
    /// The method will **panic** if conflicting or invalid combinations of layout options are detected,
    /// such as:
    /// - Using `dock()` together with `alignment()` or `pivot()`.
    /// - Using `alignment()` together with anchors (`left_anchor()`, `top_anchor()`, etc.).
    /// - Providing both `width()`/`height()` and a combination of anchors that automatically
    ///   determine the same dimension (e.g., `left_anchor()` and `right_anchor()` with `width()`).
    ///
    /// These rules prevent ambiguous or undefined layout behavior.
    ///
    /// ## Notes
    /// - If only partial information is provided (e.g., `x()` without `y()`), the unspecified values
    ///   will remain unset and may use defaults or fail during layout application, depending on context.
    /// - The validation logic ensures that **only one major layout mode is active**:
    ///   - **Dock**
    ///   - **Alignment**
    ///   - **Anchors**
    ///   - **Pivot**
    ///   - **Absolute**
    ///
    /// # Returns
    /// A fully constructed [`Layout`] instance that can be assigned to a control.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // Absolute positioning
    /// let layout_absolute = LayoutBuilder::new()
    ///     .x(10)
    ///     .y(5)
    ///     .width(20)
    ///     .height(5)
    ///     .build();
    ///
    /// // Docked layout
    /// let layout_docked = LayoutBuilder::new()
    ///     .dock(Dock::Bottom)
    ///     .height(3)
    ///     .build();
    ///
    /// // Invalid combination (will panic)
    /// // LayoutBuilder::new()
    /// //     .dock(Dock::Fill)
    /// //     .alignment(Alignment::Center)
    /// //     .build();
    /// ```
    pub fn build(self) -> Layout {
        self.inner_layout
    }

    /// This method is similar to `build()`, but it returns a `Result` instead of a `Layout`.
    /// If the layout is invalid, an error is returned explaining the issue. This method is useful
    /// when you want to build a layout dynamically and want to check for errors.
    ///
    /// # Examples
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// // invalid layout - Dock and Alignment can not be used together
    /// if let Ok(layout) = LayoutBuilder::new()
    ///     .dock(Dock::Fill)
    ///     .alignment(Alignment::Center)
    ///     .try_build() 
    /// {
    ///     println!("Layout is valid");
    /// } else {
    ///     println!("Layout is invalid");
    /// }
    /// ```
    pub fn try_build(self) -> Result<Layout, Error> {
        let _ = LayoutMode::new(self.inner_layout.clone())?;
        Ok(self.inner_layout)
    }
}

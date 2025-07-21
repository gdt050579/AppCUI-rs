# Layout

AppCUI supports the following layout modes:
* **Absolute** - The control is positioned using an explicit top-left coordinate (x, y) and a fixed size (width, height).
(Example: `x = 10, y = 5, width = 20, height = 3`)

* **Pivot** - The control is positioned relative to a reference point (x, y) which acts as a pivot.
The pivot determines how the control’s rectangle is placed around that point (e.g., TopLeft, Center, BottomRight).
(Example: `pivot = Center means (x, y) is the center of the control`)

* **Dock** - The control is attached to one of its parent’s edges (Left, Right, Top, Bottom) or fills the remaining space (Fill).
The control stretches along the opposite axis to fully occupy the available space.
(Example: `Dock::Top → control fills width and stays at the top of the parent`)

* **Aligned** - The control is aligned relative to its parent’s bounding box, using one of nine alignment positions:
TopLeft, TopCenter, TopRight, CenterLeft, Center, CenterRight, BottomLeft, BottomCenter, BottomRight.
(Example: `Align::BottomRight → control is placed at the bottom-right corner of the parent with its own size`)

* **Anchored** - The control is attached to one or more of the parent’s margins (left, right, top, bottom).
Anchors determine how the control resizes when the parent changes size:
    - If both left and right anchors are set → control’s width is adjusted dynamically.
    - If both top and bottom anchors are set → control’s height is adjusted dynamically.


## Creating a layout

You can create a layout using the following 3 methods:

### 1. Using the `Layout` class
This is the simplest and fastest way to create a layout, useful for scenarios where you need a basic configuration:

```rs
// An absolute layout with:
// - x = 10
// - y = 20
// - width = 30
// - height = 40
let l = Layout::absolute(10, 20, 30, 40);
```

### 2. Using the `LayoutBuilder` class
LayoutBuilder gives you full access to the layout system, including support for percentages and combining various layout parameters:

```rs
// A pivot layout with:
// - x = 10
// - y position at 50% of the parent's height
// - width = 12 characters
// - height = 25% of the parent's height
// - pivot set to Center (the control is centered around the (x, y) point)
let l = LayoutBuilder::new()
    .x(10)
    .y(0.5)          // 50% of parent height
    .width(12)       // absolute width
    .height(0.25)    // 25% of parent height
    .pivot(Pivot::Center)
    .build();
```
### 3. Using the `layout!` procedural macro

The `layout!` macro provides the same capabilities as LayoutBuilder in a more concise and readable way:

```rs
let l = layout!("x:10, y:50%, w:12, h:25%, p:center");
```

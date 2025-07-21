# The `LayoutBuilder` object

The `LayoutBuilder` is a powerful builder pattern implementation that provides a fluent API for creating complex layout configurations in AppCUI. It allows you to define how controls are positioned and sized within their parent containers using various layout strategies.

## Methods

The following methods are available for the `LayoutBuilder` object:

| Method                    | Type               | Description                                       |
| ------------------------- | ------------------ | ------------------------------------------------- |
| `x(value)`                | numerical or float | Sets horizontal position (absolute or percentage) |
| `y(value)`                | numerical or float | Sets vertical position (absolute or percentage)   |
| `width(value)`            | numerical or float | Sets control width (absolute or percentage)       |
| `height(value)`           | numerical or float | Sets control height (absolute or percentage)      |
| `alignment(align)`        | Alignment          | Sets alignment within parent                      |
| `pivot(pivot)`            | Pivot              | Sets pivot point for reference-based positioning  |
| `dock(dock)`              | Dock               | Sets docking behavior                             |
| `left_anchor(distance)`   | numerical or float | Distance from parent's left edge                  |
| `right_anchor(distance)`  | numerical or float | Distance from parent's right edge                 |
| `top_anchor(distance)`    | numerical or float | Distance from parent's top edge                   |
| `bottom_anchor(distance)` | numerical or float | Distance from parent's bottom edge                |

## Layout Modes

The `LayoutBuilder` supports several distinct layout modes, each optimized for different use cases:

### 1. Absolute Layout

Position controls at fixed (top-left) coordinates with explicit dimensions (width and height). The following parameters are required: `x`, `y`. If `width` and `height` are not provided they are considered to be **100%** of the parent control.

```rust
use appcui::prelude::*;

let layout = LayoutBuilder::new()
    .x(10)
    .y(5)
    .width(20)
    .height(5)
    .build();
```

### 2. Pivot Layout

Position controls relative to a reference point, with a pivot point that determines how the control is positioned relative to a given reference point `(x, y)`.
Unlike absolute positioning (which always treats `(x, y)` as the top-left corner), a pivot allows `(x, y)` to represent any logical point of the control (e.g., center, bottom-right).
The following parameters are required: `x`, `y`. If `width` and `height` are not provided they are considered to be **100%** of the parent control. The `pivot` parameter is optional (if not provided it is considered to be `TopLeft`)

For example:
- `Pivot::TopLeft` → `(x, y)` will be the top-left corner of the control.
- `Pivot::TopCenter` → `(x, y)` will align with the middle of the top edge.
- `Pivot::Center` → `(x, y)` will be the center of the control.
- `Pivot::BottomRight` → `(x, y)` will be the bottom-right corner.

The possible pivot values are defined by the [`Pivot`] enum:
- `Pivot::TopLeft`      – Align the top-left corner.
- `Pivot::TopCenter`    – Align the top edge centered horizontally.
- `Pivot::TopRight`     – Align the top-right corner.
- `Pivot::CenterLeft`   – Align the left edge centered vertically.
- `Pivot::Center`       – Align the center of the control.
- `Pivot::CenterRight`  – Align the right edge centered vertically.
- `Pivot::BottomLeft`   – Align the bottom-left corner.
- `Pivot::BottomCenter` – Align the bottom edge centered horizontally.
- `Pivot::BottomRight`  – Align the bottom-right corner.

```rust
use appcui::prelude::*;

// Center of control at position (50, 20)
let layout = LayoutBuilder::new()
    .x(50)
    .y(20)
    .width(20)
    .height(10)
    .pivot(Pivot::Center)
    .build();
```

### 3. Alignment Layout

Position controls at predefined locations within the parent container (such as top-left, center, or bottom-right) while keeping its size fixed.
The possible alignment values are defined by the [`Alignment`] enum:
- `Alignment::TopLeft`     – Control is positioned at the top-left corner of the parent.
- `Alignment::TopCenter`   – Control is horizontally centered at the top edge.
- `Alignment::TopRight`    – Control is positioned at the top-right corner.
- `Alignment::CenterLeft`  – Control is vertically centered on the left edge.
- `Alignment::Center`      – Control is centered both horizontally and vertically.
- `Alignment::CenterRight` – Control is vertically centered on the right edge.
- `Alignment::BottomLeft`  – Control is positioned at the bottom-left corner.
- `Alignment::BottomCenter`- Control is horizontally centered at the bottom edge.
- `Alignment::BottomRight` – Control is positioned at the bottom-right corner.


```rust
use appcui::prelude::*;

// Control aligned to top-left corner
let layout = LayoutBuilder::new()
    .alignment(Alignment::TopLeft)
    .width(20)
    .height(5)
    .build();

// Control centered in parent
let layout = LayoutBuilder::new()
    .alignment(Alignment::Center)
    .width(30)
    .height(10)
    .build();
```

### 4. Dock Layout

Attach controls to edges of the parent container with automatic stretching. The control is resized automatically when the parent is resized.
The possible docking options are defined by the [`Dock`] enum:
- `Dock::Left`   – Control is attached to the left edge of the parent and stretches vertically.
- `Dock::Right`  – Control is attached to the right edge of the parent and stretches vertically.
- `Dock::Top`    – Control is attached to the top edge of the parent and stretches horizontally.
- `Dock::Bottom` – Control is attached to the bottom edge of the parent and stretches horizontally.
- `Dock::Fill`   – Control fills the entire remaining space of the parent.

```rust
use appcui::prelude::*;

// Dock to left side, stretch vertically
let layout = LayoutBuilder::new()
    .dock(Dock::Left)
    .width(20)  // width respected, height stretches
    .build();

// Fill entire parent area
let layout = LayoutBuilder::new()
    .dock(Dock::Fill)
    .build();
```

### 5. Anchor Layout

Create responsive layouts that automatically adjust when the parent resizes based on the following anchors:
- `left_anchor(distance)`  – Distance from the parent's left edge (absolute or percentage).
- `right_anchor(distance)` – Distance from the parent's right edge (absolute or percentage).
- `top_anchor(distance)`   – Distance from the parent's top edge (absolute or percentage).
- `bottom_anchor(distance)` – Distance from the parent's bottom edge (absolute or percentage).


```rust
use appcui::prelude::*;

// Anchored to all edges - control stretches with parent
let layout = LayoutBuilder::new()
    .left_anchor(10)
    .right_anchor(10)
    .top_anchor(5)
    .bottom_anchor(5)
    .build();

// Mixed anchoring - fixed width, flexible height
let layout = LayoutBuilder::new()
    .left_anchor(5)
    .top_anchor(2)
    .bottom_anchor(2)
    .width(30)
    .build();
```


## Value Types

The `LayoutBuilder` accepts flexible value types for positions and dimensions:

### Absolute Values

Use integer types for fixed positions/sizes in character cells:
```rust
let layout = LayoutBuilder::new()
    .x(10)        // 10 characters from left
    .width(20)    // 20 characters wide
```

### Percentage Values

Use float values (0.0-1.0) for responsive sizing:
```rust
let layout = LayoutBuilder::new()
    .x(0.5)       // 50% of parent width from left
    .width(0.8)   // 80% of parent width
```

### Negative Values

Anchors support negative values for positioning outside parent bounds:
```rust
let layout = LayoutBuilder::new()
    .left_anchor(-5)  // 5 characters to the left of parent
```

## Advanced Examples

### Responsive Side Panel

```rust
use appcui::prelude::*;

let sidebar = LayoutBuilder::new()
    .dock(Dock::Left)
    .width(0.25)  // 25% of parent width
    .build();
```

### Centered Dialog with Fixed Size

```rust
use appcui::prelude::*;

let dialog = LayoutBuilder::new()
    .alignment(Alignment::Center)
    .width(50)
    .height(20)
    .build();
```

### Status Bar with Margin

```rust
use appcui::prelude::*;

let status_bar = LayoutBuilder::new()
    .left_anchor(1)
    .right_anchor(1)
    .bottom_anchor(1)
    .height(1)
    .build();
```

### Complex Multi-Anchor Layout

```rust
use appcui::prelude::*;

let content_area = LayoutBuilder::new()
    .left_anchor(2)       // Fixed left margin
    .right_anchor(0.1)    // 10% right margin
    .top_anchor(5)        // Fixed top margin
    .bottom_anchor(3)     // Fixed bottom margin
    .build();
```

## Validation and Error Handling

The `build()` method performs validation and will **panic** if conflicting layout options are detected:

- Using `dock()` with `alignment()` or `pivot()`
- Using `alignment()` with anchors
- Conflicting size specifications


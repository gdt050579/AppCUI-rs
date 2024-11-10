# Surface

A surface is a two-dimensional array of [Characters](screen.md#character) that can be displayed on the screen. It is the basic building block of the UI system. Surfaces can be created and manipulated using the `Surface` class.

<img src="img/surface.png" width=400/>

A surface has the following properties:
- a clipper area that restricts the drawing operations to a specific region of the surface
- an origin point that is used as the reference point for all drawing operations
- a cursor (that can be moved, enabled or disabled)
- an array (vector) of [characters](screen.md#character) that represent the content of the surface

**Remarks**: A screen is in fact a surface that covers the entire console visible space and it is created automatically when the application starts.

## Creating a Surface

To create a new surface, you can use the method `Surface::new()` - with two parameters, `width` and `height` - that returns a new surface with the specified dimensions. Both `width` and `height` must be greater than zero and smaller than **10000**. Any value outside this range will be clamped to the nearest valid value.

The surface will be filled with the space character `' '` with a `White` foreground and `Black` background. The surface will have the origin set to (0,0) and the clip area will be the entire surface. The cursor associated with the surface will be disabled.

```rust
use appcui::graphics::{Surface};
let mut surface = Surface::new(100, 50);
```

**Remarks**: Creating a surface is rarely needed, as the library will create the main screen surface automatically when the application starts and will provide a mutable reference to that surface whenever the on_paint event is called for a control.

## Clip Area and Origin point

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

Every surface has a clip area and an origin point. The clip area restricts the drawing operations to a specific region of the surface. The origin point is used as the reference point for all drawing operations.

For example, if the clip area is set to (10,10,20,20) and the origin point is set to (5,5), then the drawing operations will be restricted to the area (15,15,25,25) of the surface.

The following methods can be used to manipulate the clip area and the origin point of a surface:

| Method                   | Description                                                                                                                       |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------- |
| `set_origin(...)`        | Sets the origin point of the surface                                                                                              |
| `reset_origin()`         | Resets the origin point                                                                                                           |
| `set_clip(...)`          | Sets the clip area of the surface. This methods take 4 parameters (left, top, right and bottom)                                   |
| `set_relative_clip(...)` | Sets the clip area of the surface relative to the current clip area. This methods take 4 parameters (left, top, right and bottom) |
| `reduce_clip_by(...)`    | Reduces the clip area of the surface. This methods take 4 parameter (left margin, top margin, right margin and bottom margin)     |
| `reset_clip()`           | Resets the clip area of the surface                                                                                               |

Example:

```rust
use appcui::graphics::*;

let mut surface = Surface::new(100, 50);
// Set the origin point to (10,10)
surface.set_origin(10, 10);
// Set the clip area to (10,10,20,20)
surface.set_clip(10, 10, 20, 20);
// draw a border around the clip area
surface.draw_rect(
    Rect::new(0,0,9,9), // left,top,right,bottom relativ to origin
    LineType::Single,
    CharAttribute::with_color(Color::White, Color::DarkRed)
);
// reduce the clip area by 1 character on each side
// so that we will not draw over the border
surface.reduce_clip_by(1, 1, 1, 1);
// draw something else
// ...

/// finally, reset the clip area and origin point
/// to the entire surface
surface.reset_clip();
surface.reset_origin();
```

## Cursor

Every surface has an associated cursor that can be moved, enabled or disabled. The cursor is used to indicate the current position where the next character will be drawn.
Depending on the terminal, the cursor can be a blinking rectangle, a blinking underline or a blinking vertical line.

The following methods can be used to manipulate the clip area and the origin point of a surface:

| Method            | Description                                                                                                                                          |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set_cursor(...)` | Sets the position of the cursor relativ to the origin point. If the cursor is within the clip area, it will be visible. Otherwise it will be hidden. |
| `hide_cursor()`   | Hides the cursor                                                                                                                                     |

Example:

```rust
use appcui::graphics::{Surface};

let mut surface = Surface::new(100, 50);
surface.set_cursor(10, 10);
```

## Drawing characters on a Surface

The most basic operation that can be performed on a surface is drawing a character at a specific position. This allows for more complex operations like drawing text, lines, rectangles, etc. to be built on top of it.

A surface has the following methods that can be used to manipulate characters and how they are drown on the surface:

| Method            | Description                                                                                                                                              |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_char(...)` | Writes a [character](screen.md#character) at the specified position. If the position is outside the clip area, the character will not be drawn.          |
| `char(...)`       | Returns the current [character](screen.md#character) at the specified position or `None` if the position is outside the clip area or invalid.            |
| `clear(...)`      | Clears/Fills the entire clip area with the specified [character](screen.md#character). If the clip area is not visible, the surface will not be cleared. |

Example:

```rust
use appcui::graphics::*;

let mut surface = Surface::new(100, 50);
// Set the origin point to (10,10)
surface.set_origin(10, 10);
// Set the clip area to (10,10,20,20)
surface.set_clip(10, 10, 20, 20);
// Clear the clip area
surface.clear(Character::new('*', Color::Silver, Color::Black, CharFlags::None))
// write a character at position (5,5) relativ to the origin
// point (10,10) => the character will be drawn at position (15,15)
surface.write_char(5, 5, Character::new('A', Color::Yellow, Color::DarkBlue, CharFlags::None));
```

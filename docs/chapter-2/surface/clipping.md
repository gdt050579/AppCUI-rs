# Clip Area and Origin point

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
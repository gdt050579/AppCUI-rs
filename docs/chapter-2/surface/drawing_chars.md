# Drawing characters on a Surface

The most basic operation that can be performed on a surface is drawing a character at a specific position. This allows for more complex operations like drawing text, lines, rectangles, etc. to be built on top of it.

A surface has the following methods that can be used to manipulate characters and how they are drown on the surface:

| Method            | Description                                                                                                                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_char(...)` | Writes a [character](../screen.md#character) at the specified position. If the position is outside the clip area, the character will not be drawn.                 |
| `char(...)`       | Returns the current [character](../screen.md#character) at the specified position or `None` if the position is outside the clip area or invalid.            |
| `clear(...)`      | Clears/Fills the entire clip area with the specified [character](../screen.md#character). If the clip area is not visible, the surface will not be cleared. |

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
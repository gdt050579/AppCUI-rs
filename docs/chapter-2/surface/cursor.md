# Cursor

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
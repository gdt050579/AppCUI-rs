# Drawing characters on a Surface

The most basic operation that can be performed on a surface is drawing a character at a specific position. This allows for more complex operations like drawing text, lines, rectangles, etc. to be built on top of it.

A surface has the following methods that can be used to manipulate characters and how they are drown on the surface:

| Method                              | Description                                                                                                                                                                                                 |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_char(...)`                   | Writes a [character](../screen.md#character) at the specified position. If the position is outside the clip area, the character will not be drawn.                                                          |
| `write_chars(...)`                  | Writes a slice of [characters](../screen.md#character) from left to right starting at a given position. Each character keeps its own code, colors and flags. Cells outside the clip area are skipped.      |
| `char(...)`                         | Returns the current [character](../screen.md#character) at the specified position or `None` if the position is outside the clip area or invalid.                                                            |
| `clear(...)`                        | Clears/Fills the entire **clip area** with the specified [character](../screen.md#character). If the clip area is not visible, the surface will not be cleared. Origin and clip are left unchanged.         |
| `reset(...)`                        | Fills the **entire** surface with the specified [character](../screen.md#character) and restores the origin and clip to the full surface. Use this when preparing an overlay that will be copied later.     |
| `draw_surface(...)`                 | Copies another surface onto this one at a given position. Transparent foreground or background colors in the source do not overwrite the destination, which allows layered compositing.                     |
| `draw_surface_with_transform(...)`  | Same as `draw_surface`, but a callback can remap each source character (code, colors or flags) before it is written.                                                                                        |

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

`clear` only paints the current clip. `reset` is stronger: it restores clip and origin and then fills every cell. That is the usual way to prepare a secondary surface with transparent characters before copying it with `draw_surface`:

```rust
use appcui::graphics::*;

let mut overlay = Surface::new(10, 5);
overlay.reset(Character::new('\0', Color::Transparent, Color::Transparent, CharFlags::None));
overlay.write_string(0, 0, "Hi", CharAttribute::with_color(Color::Yellow, Color::Transparent), false);

let mut surface = Surface::new(40, 12);
surface.clear(Character::new(' ', Color::White, Color::DarkBlue, CharFlags::None));
surface.draw_surface(2, 2, &overlay);
```

`draw_surface_with_transform` is the same copy, but you can tint or mask the source:

```rust
surface.draw_surface_with_transform(2, 2, &overlay, |ch| {
    Character::new(ch.code, Color::Red, ch.background, ch.flags)
});
```

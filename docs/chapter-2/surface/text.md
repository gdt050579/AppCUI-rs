# Text

Writing text on a surface is a common task in GUI programming, that can be achieved using the following methods:
1. `write_string(...)` - writes a string (`String` or `&str`) on the surface starting from a specific position, color and character attribute.
2. `write_ascii(...)` - similar to **write_string**, but it writes only ASCII characters.
3. `write_text(...)` - a more complex method that allows alignament, wrapping and text formatting.

## Wriring a string

The `write_string(...)` method writes a string on the surface starting from a specific position. The method has the following signature:

```rust
pub fn write_string(&mut self, 
                    x: i32, 
                    y: i32, 
                    text: &str, 
                    attr: CharAttribute, 
                    multi_line: bool)
```

The `multi-line` parameter specifices if the text should interpret new line characters as a new line or not. if set to `false` the code of this method is optimized to write the text faster. The text will be written from left to right, starting from the specified position (x,y).

Example:

```rust
use appcui::graphics::{Surface, CharAttribute, Color};

let mut surface = Surface::new(100, 50);
surface.write_string(10, 10, 
                    "Hello World!", 
                    CharAttribute::with_color(Color::White, Color::Black), 
                    false);
```

## Wriring an ASCII string

The `write_ascii(...)` method writes an ASCII string on the surface starting from a specific position. The method has the following signature:

```rust
pub fn write_ascii(&mut self, 
                   x:i32, 
                   y:i32, 
                   ascii_buffer: &[u8], 
                   attr: CharAttribute, 
                   multi_line: bool)
```

The `multi-line` parameter specifices if the text should interpret new line characters as a new line or not. if set to `false` the code of this method is optimized to write the text faster. The text will be written from left to right, starting from the specified position (x,y).

Example:

```rust
use appcui::graphics::{Surface, CharAttribute, Color};

let mut surface = Surface::new(100, 50);
surface.write_ascii(10, 10,
                   b"Hello World!",
                   CharAttribute::with_color(Color::White, Color::Black),
                   false);
```
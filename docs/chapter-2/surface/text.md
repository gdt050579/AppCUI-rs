# Text

Writing text on a surface is a common task in GUI programming, that can be achieved using the following methods:
1. `write_string(...)` - writes a string (`String` or `&str`) on the surface starting from a specific position, color and character attribute.
2. `write_ascii(...)` - similar to **write_string**, but it writes only ASCII characters.
3. `write_text(...)` - a more complex method that allows alignment, wrapping and text formatting.

## Write a string

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

## Write an ASCII string

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

## Write a formatted text

In some cases, you may need to write a text that is formatted in a specific way (like alignment, wrapping, etc). The `write_text(...)` method allows you to do this. The method has the following signature:

```rust
pub fn write_text(&mut self, text: &str, format: &TextFormat)
```

where the `TextFormat` structure can be created using the `TextFormatBuilder` in the following way:

| Method             | Description                                                                                                                        |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| `new()`            | Creates a new `TextFormatBuilder` object                                                                                           |
| `position(...)`    | Sets the position where the text will be written (X and Y axes)                                                                    |
| `attribute(...)`   | Sets the character attribute for the text (forecolor, backcolor and other attributes)                                              |
| `hotkey(...)`      | Sets the hotkey attribute and position for the text (if any)                                                                       |
| `align(...)`       | Sets the text alignment (left, right, center)                                                                                     |
| `wrap_type(...)`   | Sets the text wrapping type of the code (`WrapType` enum)                                                                          |
| `chars_count(...)` | Sets the number of characters in the text (this is useful to optimize several operations especially if this value is aready known) |
| `build()`          | Builds the `TextFormat` object                                                                                                     |

Example:

```rust
use appcui::graphics::{Surface, CharAttribute, Color, TextFormatBuilder, WrapType};
let format = TextFormatBuilder::new()
    .position(10, 10)
    .attribute(CharAttribute::with_color(Color::White, Color::Black))
    .align(Alignment::Center)
    .wrap_type(WrapType::Word(20))
    .build();
surface.write_text("Hello World!", &format);
```

Once a `TextFormat` object is created, you can modify it and use it using the following methods:

| Method                 | Description                                                                                                                        |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `set_position(...)`    | Sets the position where the text will be written (X and Y axes)                                                                    |
| `set_attribute(...)`   | Sets the character attribute for the text (forecolor, backcolor and other attributes)                                              |
| `set_hotkey(...)`      | Sets the hotkey attribute and position for the text (if any)                                                                       |
| `clear_hotkey()`       | Clears the hotkey attribute from the text                                                                                          |
| `set_align(...)`       | Sets the text alignment (left, right, center)                                                                                     |
| `set_wrap_type(...)`   | Sets the text wrapping type of the code (`WrapType` enum)                                                                          |
| `set_chars_count(...)` | Sets the number of characters in the text (this is useful to optimize several operations especially if this value is aready known) |

The `WrapType` enum is defined as follows:

```rust
pub enum WrapType {
    WordWrap(u16),
    CharacterWrap(u16),
    MultiLine,
    SingleLine,
    SingleLineWrap(u16),
}
```

with the following meaning:

| Method                  | Multi-line | Description                                                                                                                                                                                                                                              |
| ----------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| WordWrap(`width`)       | Yes        | Wraps the text around a specific `width` not separating words. The text will be printed on the next line if a new line character (`CR` or `LF` or combinations) is encountered or if the current word if printed will be outside the specfied width.     |
| CharacterWrap(`width`)  | Yes        | Wraps the text around a specific `width` separating words. The text will be printed on the next line if a new line character (`CR` or `LF` or combinations) is encountered or when the position of the current character is outside the specified width. |
| MultiLine               | Yes        | The text will be printed on the next line only if a new line character (`CR` or `LF` or combinations) is encountered.                                                                                                                                    |
| SingleLine              | No         | The text will be printed on the same line, ignoring any new line characters.                                                                                                                                                                             |
| SingleLineWrap(`width`) | No         | The text will be printed on the same line, but it will be wrapped around a specific `width`. One the `width` is reach, the printing stops.                                                                                                               |

Let's consider the following string `Hello World!\nFrom AppCUI`. This text will be printed as follows:

| WrapType                     | Result                                      |
| ---------------------------- | ------------------------------------------- |
| WrapType::WordWrap(10)       | Hello<br>World!<br><br>From <br>AppCUI      |
| WrapType::WordWrap(20)       | Hello World!<br>From AppCUI                 |
| WrapType::CharacterWrap(10)  | Hello Worl<br>d!<br>From AppC<br>UI         |
| WrapType::CharacterWrap(20)  | Hello World!<br>From AppCUI                 |
| WrapType::CharacterWrap(5)   | Hello<br> Worl<br>d!<br>From <br>AppCU<br>I |
| WrapType::MultiLine          | Hello World!<br>From AppCUI                 |
| WrapType::SingleLine         | Hello World!`\n`From AppCUI                 |
| WrapType::SingleLineWrap(5)  | Hello                                       |
| WrapType::SingleLineWrap(10) | Hello Worl                                  |
| WrapType::SingleLineWrap(20) | Hello World!`\n`From Ap                     |



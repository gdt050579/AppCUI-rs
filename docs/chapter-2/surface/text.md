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

## Write a formatted text

In some cases, you may need to write a text that is formatted in a specific way (like alignament, wrapping, etc). The `write_text(...)` method allows you to do this. The method has the following signature:

```rust
pub fn write_text(&mut self, text: &str, format: &TextFormat)
```

with the `TextFormat` structure defined as follows:

```rust
pub struct TextFormat {
    pub x: i32,
    pub y: i32,
    pub width: Option<u16>,
    pub char_attr: CharAttribute,
    pub hotkey_attr: Option<CharAttribute>,
    pub hotkey_pos: Option<usize>,
    pub chars_count: Option<u16>,
    pub align: TextAlignament,
    pub text_wrap: TextWrap,
    pub multi_line: bool,
}
```

where:
- `x` and `y` represent the starting position of the text
- `width` represents the maximum width of the text (if `None`, the text will be written until a new line character is found or until the end of the text)
- `char_attr` represents the character attribute that will be used to write the text
- `hotkey_attr` represents the character attribute that will be used to write the hotkey (if exists) or `None` if no hotkey is present
- `hotkey_pos` represents the position of the hotkey in the text (if exists) or `None` if no hotkey is present
- `chars_count` represents the number of characters in the provided text. If not provided it will be computed automatically upon writing the text. This is useful as a performance optimization (especially if the text is in Ascii - and the number of characters is the same as the number of bytes) or the number of characters is known in advance.
- `align` represents the alignament of the text (left, right, center, etc)
- `text_wrap` represents the text wrapping mode (word, character, none). This method ensures that the text will not be written outside the provided width.
- `multi_line` specifies if the text should interpret new line characters as a new line or not.

All of these fields are public and can be modified directly. However, since the `TextFormat` structure is quite complex, it is recommended to use the `TextFormatBuilder` to create a new `TextFormat` object.
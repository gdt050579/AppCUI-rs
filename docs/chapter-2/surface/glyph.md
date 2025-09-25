# Glyphs

Glyphs are F2D character grids designed for drawing text and character-based graphics in AppCUI. Unlike BitTiles which store binary data, or Images which store full color information, Glyphs store individual characters that can be rendered with specific foreground and background colors. They are perfect for text layouts, ASCII art, character-based UI elements, and any graphics that can be represented using Unicode characters.

To create a Glyph, the following methods can be used:

1. `Glyph::new(width, height)` creates an empty Glyph with the specified dimensions. All characters will be initially transparent (null character).
2. `Glyph::with_str(width, height, text)` creates a Glyph with the specified dimensions and initializes it with the provided text from the top-left corner.

## Methods

Once a Glyph is created, you can use the following methods to manipulate it:

| Method                           | Purpose                                                                                                                                                   |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `char(x, y)`                     | Returns `Some(char)` if there is a character at coordinates (x,y), or `None` if coordinates are out of bounds                                             |
| `set_char(x, y, ch)`             | Sets the character at coordinates (x,y) to the specified char. If the coordinates are outside the bounds of the Glyph, the operation is silently ignored. |
| `clear_char(x, y)`               | Clears the character at the specified coordinates by setting it to the transparent character                                                              |
| `size()`                         | Returns the size (width and height) of the Glyph as a `Size` struct                                                                                       |
| `clear()`                        | Clears the entire Glyph by setting all characters to the transparent character                                                                            |
| `fill(ch)`                       | Fills the entire Glyph with the specified character                                                                                                       |
| `resize(width, height)`          | Resizes the Glyph to the specified dimensions and clears it with the transparent character                                                                |
| `resize_with(width, height, ch)` | Resizes the Glyph to the specified dimensions and fills it with the specified character                                                                   |
| `write_str(x, y, text)`          | Writes the specified text to the Glyph starting from the given coordinates. Supports multi-line text with `\n` characters                                 |

## Size Limitations

Glyphs have built-in size constraints for memory management:

- **Maximum size**: `1024x1024` characters
- **Size clamping**: If you specify dimensions larger than 1024x1024, they will be automatically clamped
- **Zero dimensions**: If width or height is 0, an empty Glyph with zero size is created
- **Dynamic memory**: Uses `Vec<char>` for storage, allowing flexible sizing up to the maximum limit

## Usage

A typical workflow for creating and using Glyphs:

1. Create a new `Glyph` object with desired dimensions
2. Optionally, fill or clear the Glyph with a default character
3. Use `.set_char(...)` or `.write_str(...)` methods to add content
4. Render the Glyph to a Surface using appropriate character attributes

### Basic Example

The following example creates a simple 10x3 Glyph with a greeting:

```rs
use appcui::prelude::*;

let mut glyph = image::Glyph::new(10, 3);
glyph.write_str(0, 0, "Hello");
glyph.write_str(0, 1, "World!");
glyph.set_char(9, 2, '!');
```

### Creating ASCII Art

Here's how to create a simple ASCII art pattern:

```rs
use appcui::prelude::*;

let mut glyph = image::Glyph::new(7, 5);
glyph.write_str(0, 0, "  ___  ");
glyph.write_str(0, 1, " /   \\ ");
glyph.write_str(0, 2, "|  o  |");
glyph.write_str(0, 3, " \\___/ ");
glyph.write_str(0, 4, "   |   ");
```

### Multi-line Text

The `write_str` method supports multi-line text using newline characters:

```rs
use appcui::prelude::*;

let text = "Line 1\nLine 2\nLine 3";
let glyph = image::Glyph::with_str(20, 10, text);
```

## Transparent Characters

Glyphs use a special transparent character (null character `\0`) to represent empty spaces:

- **Transparent character**: `\0` (null character)
- **Purpose**: Allows parts of the Glyph to be "see-through" when rendered
- **Default state**: All characters in a new Glyph are initially transparent
- **Clearing**: Use `clear()` or `clear_char()` to set characters back to transparent

## Rendering Glyphs

AppCUI provides a dedicated method for rendering Glyphs to the screen through the [Surface](../surface.md) object `draw_glyph(...)` method:

```rs
impl Surface {
    pub fn draw_glyph(
        &mut self, 
        x: i32, 
        y: i32, 
        glyph: &Glyph,
        attr: CharAttribute
    ) { ... }
}
```

### CharAttribute

The `CharAttribute` parameter controls how the Glyph is rendered:

- **Foreground color**: The color of the characters
- **Background color**: The background color behind the characters  
- **Character flags**: Additional styling like bold, underline, etc.

### Example Usage

```rs
use appcui::prelude::*;

// Create a simple text glyph
let glyph = image::Glyph::with_str(15, 3, "Hello, AppCUI!\nThis is a test\nof multi-line!");

// Define rendering attributes
let attr = CharAttribute::with_color(Color::Yellow, Color::Blue);

// Render to surface
surface.draw_glyph(10, 5, &glyph, attr);
```

### Advanced Rendering Example

```rs
use appcui::prelude::*;

// Create a border pattern
let mut border_glyph = image::Glyph::new(5, 5);
border_glyph.write_str(0, 0, "┌───┐");
border_glyph.write_str(0, 1, "│   │");
border_glyph.write_str(0, 2, "│ X │");
border_glyph.write_str(0, 3, "│   │");
border_glyph.write_str(0, 4, "└───┘");

// Render with different attributes
let border_attr = CharAttribute::with_color(Color::White, Color::Black);
surface.draw_glyph(0, 0, &border_glyph, border_attr);
```


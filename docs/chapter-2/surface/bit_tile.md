# Bit Tiles

BitTiles are compact, memory-efficient structures designed for storing and rendering monochrome (black and white) images in AppCUI. Unlike full-color images, a BitTile represents each pixel using a single bit - either set (1) or unset (0) - making them perfect for icons, simple graphics, and patterns where memory usage is a concern.

To create a BitTile, the following methods can be used:

1. `BitTile::new(width, height)` creates a BitTile with custom storage size. All pixels will be initially unset (false).
2. `BitTile::from_str(...)` creates a BitTile from a string representation using the pipe (`|`) format.
3. Predefined types like `BitTileU16::from_u16(width, height, bits)` for working with specific integer representations. The following predefined types are available:
    - `BitTileU16`
    - `BitTileU32`
    - `BitTileU64`
    - `BitTileU128`

## Methods

Once a BitTile is created, you can use the following methods to manipulate it:

| Method             | Purpose                                                                                                                                                          |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `get(x, y)`        | Returns `Some(bool)` if the pixel at coordinates (x,y) is set, or `None` if coordinates are out of bounds                                                        |
| `set(x, y, value)` | Sets the pixel at coordinates (x,y) to the specified boolean value. If the coordinates are outside the bounds of the BitTile, the operation is silently ignored. |
| `width()`          | Returns the width of the BitTile in pixels (as `u8`)                                                                                                             |
| `height()`         | Returns the height of the BitTile in pixels (as `u8`)                                                                                                            |
| `size()`           | Returns the size (width and height) of the BitTile as a `Size` struct                                                                                            |
| `clear(value)`     | Fills the entire BitTile with the specified boolean value (**true** for all set, **false** for all unset)                                                        |

## BitTile Types

AppCUI provides several predefined BitTile types for common use cases:

| Type          | Alias Type  | Storage  | Max Pixels | Purpose                            |
| ------------- | ----------- | -------- | ---------- | ---------------------------------- |
| `BitTileU16`  | BitTile<2>  | 2 bytes  | 16         | Small icons and simple patterns    |
| `BitTileU32`  | BitTile<4>  | 4 bytes  | 32         | Medium-sized graphics              |
| `BitTileU64`  | BitTile<8>  | 8 bytes  | 64         | Larger icons and detailed patterns |
| `BitTileU128` | BitTile<16> | 16 bytes | 128        | Complex graphics with fine detail  |

### Working with Predefined Types

Predefined types offer additional methods for working with integer representations:

```rs
// Create a 4x4 BitTileU16 from a u16 value
let tile = BitTileU16::from_u16(4, 4, 0b1001_0110_1001_0110).unwrap();

// Get the integer representation
let bits: u16 = tile.to_u16();

// Reset the tile with new bit pattern
tile.reset(0b1111_0000_1111_0000);
```

## Usage

A typical workflow for creating and using BitTiles:

1. Create a new `BitTile` object
2. Optionally, clear the entire tile with a default value
3. Use `.set(...)` method to draw your pattern, or create from a string representation

The following example creates a simple 8x8 checkerboard pattern:

```rs
let mut tile = BitTileU64::new(8, 8).unwrap();
for y in 0..8 {
    for x in 0..8 {
        let is_set = (x + y) % 2 == 0;
        tile.set(x, y, is_set);
    }
}
```

## Building from a String

The most convenient way to create BitTiles is from a string representation. The format follows these rules:

* Each line is enclosed between pipe characters (`|`)
* Characters outside the pipes are ignored (typically used for spacing and alignment)
* Each line must have the same width (number of characters between `|` characters)
* Only two types of characters are recognized:
  - **Unset pixels**: ` ` (space) and `.` (point)
  - **Set pixels**: Any other character

For example, a `5x5` BitTile can be represented as:

```rs
let string_representation = r#"
      |.....|
      |.XXX.|
      |.X.X.|
      |.XXX.|
      |.....|
"#;
let tile = BitTileU32::from_str(string_representation).unwrap();
```

### Creating a Heart Icon

Here's how to create a heart-shaped icon using the string format:

```rs
let heart = r#"
    |...rr....rr...|
    |..rrrr..rrrr..|
    |.rrrrrrrrrrrr.|
    |.rrrrrrrrrrrr.|
    |..rrrrrrrrrr..|
    |   rrrrrrrr   |
    |....rrrrrr....|
    |.....rrrr.....|
    |......rr......|
"#;
let heart_tile = BitTileU128::from_str(heart).unwrap();
```

Note that any character except space (` `) and point (`.`) represents a set pixel - in this example, `r` characters create the heart shape.

## Rendering BitTiles

AppCUI provides three different methods for rendering BitTiles to the screen through the [Surface](../surface.md) object `draw_tile(...)` method that is defined as follows:

```rs
impl Surface {
    pub fn draw_tile<const STORAGE_BYTES: usize>(
        &mut self, 
        x: i32, 
        y: i32, 
        tile: &BitTile<STORAGE_BYTES>,
        set_bit_color: Color,
        unset_bit_color: Color,
        render_method: BitTileRenderMethod
    ) { ... }
}
```

### Render Methods

The `BitTileRenderMethod` enum provides three rendering approaches:

#### SmallBlocks
Uses half-block characters to render two vertical pixels per character. This is the most compact method:
- Each character represents 2 vertical pixels
- Uses `▀` (upper half), `▄` (lower half), `█` (full block), or space
- Efficient for detailed patterns in limited space

#### LargeBlocks  
Renders each pixel as two characters wide, providing maximum visibility:
- Each pixel becomes a 2x1 character block
- Best for small tiles that need to be clearly visible
- Uses twice the horizontal space

#### Braille
Uses Unicode Braille characters for ultra-compact rendering:
- Each character represents an 2x4 pixel area (8 pixels total)
- Most space-efficient method
- Uses Unicode range U+2800-U+28FF for Braille patterns
- Excellent for detailed graphics in minimal space

**Remarks**: The Braille rendering method is the most space-efficient method, but depending on the font used the characters might aligned differently on the vertical vs horizontal axis.

### Example Usage

```rs
use appcui::prelude::*;

// Create a simple cross pattern
let cross = r#"
    |..X..|
    |..X..|
    |XXXXX|
    |..X..|
    |..X..|
"#;

let tile = BitTileU32::from_str(cross).unwrap();

// Render using different methods
surface.draw_tile(10, 5, &tile, Color::Red, Color::Black, BitTileRenderMethod::SmallBlocks);
surface.draw_tile(20, 5, &tile, Color::Green, Color::Black, BitTileRenderMethod::LargeBlocks);
surface.draw_tile(30, 5, &tile, Color::Blue, Color::Black, BitTileRenderMethod::Braille);
```

## Error Handling

When creating BitTiles from strings, several errors can occur:

| Error                             | Cause                                                                                                                                                                                             |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `MultipleWidths`                  | Lines have different numbers of characters between `&#x7C;` characters (for example `&#x7C;...&#x7C; &#x7C;....&#x7C;` has one line with 3 pixels and another line with 4 pixels)                 |
| `ZeroHeight`                      | No valid lines found in the string (for example an empty string or a string without `&#x7C;` characters)                                                                                          |
| `ZeroWidth`                       | Lines contain no characters between `&#x7C;` characters (for example  `&#x7C;&#x7C;`)                                                                                                             |
| `ImageTooLarge`                   | Width or height the maximum allowed by the storage of the BitTile (for example a BitTileU16 can only have a width and height of up to 16 pixels)                                                  |
| `ImageDoesNotFitInAllocatedSpace` | Total pixels exceed the BitTile's storage capacity (for example a BitTileU16 can only have a width and height of up to 16 pixels so a string representation of a 5x4 image will cause this error) |
| `MissingCorrespondingMarker`      | Unmatched `&#x7C;` character in the string (for example `&#x7C;...` is missing the corresponding `&#x7C;` at the end)                                                                                                                                                       |

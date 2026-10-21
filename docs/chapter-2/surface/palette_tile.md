# Palette Tiles

A PaletteTile is a 2D grid of pixels where every pixel stores an **index** into a palette, not a color. The palette itself is not stored in the tile: it is given when the tile is drawn, as a function that turns an index into a `CharAttribute`. This makes PaletteTiles a good fit for pictures made of a few distinct regions (the slices of a chart, the areas of a map, the parts of a diagram), where you need to know which region each pixel belongs to and you may want to recolor a region without changing the pixels.

Compared with the other image types:

| Type          | A pixel stores                    | Colors                                       | Size                                |
| ------------- | --------------------------------- | -------------------------------------------- | ----------------------------------- |
| `Image`       | an RGBA color                     | stored in the image                          | any size                            |
| `BitTile`     | one bit (set or unset)            | two colors, given when drawn                 | small, fixed storage (no heap)      |
| `PaletteTile` | a palette index, or transparent   | any number, from a palette given when drawn  | any size, resizable                 |

A PaletteTile is created with `PaletteTile::new(width, height)`. All pixels are initially transparent. A width or height of `0` is raised to `1`.

## Methods

| Method                          | Purpose                                                                                                                                                         |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set(x, y, index)`              | Sets the pixel at coordinates (x,y) to the specified palette index. If the coordinates are outside the bounds of the PaletteTile, the operation is silently ignored. |
| `get(x, y)`                     | Returns `Some(index)` for the pixel at coordinates (x,y), or `None` if the pixel is transparent or the coordinates are out of bounds                           |
| `clear(x, y)`                   | Makes the pixel at coordinates (x,y) transparent                                                                                                                |
| `reset()`                       | Makes every pixel transparent (the size is not changed)                                                                                                         |
| `resize(width, height)`         | Changes the size of the PaletteTile and makes every pixel transparent                                                                                           |
| `size()`                        | Returns the size (width and height) of the PaletteTile, in pixels, as a `Size` struct                                                                           |
| `render_size(method)`           | Returns the size, in characters, of the PaletteTile when it is drawn with the given render method                                                               |
| `cell_index(x, y, method)`      | Returns the palette index shown by the character at (x,y) when the PaletteTile is drawn with the given render method (see [Finding what is under the mouse](#finding-what-is-under-the-mouse)) |

## Storage

To keep memory usage low, a PaletteTile starts with one byte per pixel. Indexes `0` to `254` fit in that storage (`255` marks a transparent pixel). The first time an index of `255` or higher is written, the PaletteTile switches to two bytes per pixel, keeping every pixel it already has. After that, indexes from `0` to `65534` can be used and `65535` (`u16::MAX`) marks a transparent pixel. A PaletteTile that switched to two bytes per pixel keeps that storage, even after `reset()` or `resize(...)`.

This is done automatically: `set(...)` always takes a `u16` index and `get(...)` always returns a `u16` index, whatever the storage is.

## Usage

A typical workflow for creating and using PaletteTiles:

1. Create a new `PaletteTile` object (or `resize(...)` an existing one, to reuse its memory)
2. Use `.set(...)` to write the palette index of every pixel that is not transparent
3. Draw it with `surface.draw_palette_tile(...)`, giving the palette as a function

The following example creates a 16x16 disk, split into four quarters (indexes `0` to `3`):

```rs
let mut tile = PaletteTile::new(16, 16);
for y in 0..16u32 {
    for x in 0..16u32 {
        let dx = x as f64 + 0.5 - 8.0;
        let dy = y as f64 + 0.5 - 8.0;
        if dx * dx + dy * dy <= 64.0 {
            let index = match (dx >= 0.0, dy >= 0.0) {
                (true, false) => 0,  // top-right
                (true, true) => 1,   // bottom-right
                (false, true) => 2,  // bottom-left
                (false, false) => 3, // top-left
            };
            tile.set(x, y, index);
        }
    }
}
```

## Rendering PaletteTiles

PaletteTiles are drawn through the [Surface](../surface.md) object `draw_palette_tile(...)` method that is defined as follows:

```rs
impl Surface {
    pub fn draw_palette_tile<P>(
        &mut self,
        x: i32,
        y: i32,
        tile: &PaletteTile,
        render_method: PaletteTileRenderMethod,
        palette: P,
    ) where P: Fn(u16) -> CharAttribute { ... }
}
```

The `palette` parameter is called with the index of a pixel and returns the `CharAttribute` used to draw it. It can be a closure over an array, a match, or any other computation:

```rs
let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow];
surface.draw_palette_tile(1, 1, &tile, PaletteTileRenderMethod::SmallBlocks, |index| {
    CharAttribute::with_fore_color(colors[index as usize])
});
```

Transparent pixels are not drawn, so whatever is under them stays visible.

### Render Methods

The `PaletteTileRenderMethod` enum provides four rendering approaches. Every method draws **square pixels** (a character is about twice as tall as it is wide), so the same PaletteTile gives the same picture with every method; only the resolution changes.

| Method         | Pixels per character               | Size in characters (`render_size`)       | Attributes used                                                     |
| -------------- | ---------------------------------- | ---------------------------------------- | ------------------------------------------------------------------- |
| `SmallBlocks`  | 1 x 2                              | `width` x `height / 2` (rounded up)      | only the foreground of each palette entry                           |
| `LargeBlocks`  | every pixel is 2 characters wide   | `width * 2` x `height`                   | foreground, background and flags                                    |
| `Braille`      | 2 x 4                              | `width / 2` x `height / 4` (rounded up)  | foreground, background and flags of the entry with the most dots    |
| `Custom(char)` | every pixel is 2 characters wide   | `width * 2` x `height`                   | foreground, background and flags                                    |

#### SmallBlocks
Uses half-block characters to render two vertical pixels per character:
- Each character represents 2 vertical pixels, that can have different palette indexes
- If both pixels are visible, the character is `▀`, with the top pixel's color as foreground and the bottom pixel's color as background
- If only the top pixel is visible, the character is `▀` and the background is left unchanged; if only the bottom pixel is visible, the character is `▄` and the background is left unchanged
- Since the two halves of a character can use two different palette entries, only the **foreground** of each palette entry is used

#### LargeBlocks
Renders each pixel as two full blocks (`█`), drawn with all the attributes of the palette entry. This is the same as `Custom('█')`.

#### Braille
Uses Unicode Braille characters (range U+2800-U+28FF):
- Each character represents a 2x4 pixel area (8 pixels total); every visible pixel is a dot
- A character can have only one color, so it takes the palette entry that covers most of its dots (on a tie, the first one found, column by column)
- Best resolution of all methods, for detailed pictures in minimal space

**Remarks**: depending on the font used, Braille characters might be aligned differently on the vertical vs horizontal axis.

#### Custom(char)
Renders each pixel as two copies of the given character, drawn with all the attributes of the palette entry (foreground, background and flags). Useful for textures, for example `Custom('▒')` with a red foreground and a dark red background, or for terminals where half blocks are not displayed correctly.

### Example Usage

```rs
use appcui::prelude::*;

let palette = [
    CharAttribute::with_color(Color::Red, Color::DarkRed),
    CharAttribute::with_color(Color::Green, Color::DarkGreen),
    CharAttribute::with_color(Color::Blue, Color::DarkBlue),
    CharAttribute::with_color(Color::Yellow, Color::Olive),
];
let colors = |index: u16| palette[index as usize];

// Render the same tile using different methods
surface.draw_palette_tile(1, 1, &tile, PaletteTileRenderMethod::SmallBlocks, colors);
surface.draw_palette_tile(20, 1, &tile, PaletteTileRenderMethod::Braille, colors);
surface.draw_palette_tile(30, 1, &tile, PaletteTileRenderMethod::Custom('▒'), colors);
```

### Changing colors without changing the pixels

Since the colors are only chosen when the PaletteTile is drawn, a region can be recolored by changing the palette function. For example, to highlight the region with index `2` and draw all the others in gray:

```rs
let selected = 2;
surface.draw_palette_tile(1, 1, &tile, PaletteTileRenderMethod::SmallBlocks, |index| {
    if index == selected {
        CharAttribute::with_fore_color(Color::Yellow)
    } else {
        CharAttribute::with_fore_color(Color::Gray)
    }
});
```

## Finding what is under the mouse

`cell_index(x, y, method)` returns the palette index shown by the character at `(x, y)` of the drawn PaletteTile (`(0, 0)` is the top-left character of the drawn tile), or `None` if nothing is drawn there. It uses the same rules as `draw_palette_tile(...)`, so the index it returns is always the one whose attribute was used to draw that character:

| Method                        | Index of a character                                         |
| ----------------------------- | ------------------------------------------------------------ |
| `SmallBlocks`                 | the top pixel, or the bottom pixel if the top is transparent |
| `LargeBlocks`, `Custom(char)` | the pixel (every pixel is 2 characters wide)                 |
| `Braille`                     | the index that covers most of the dots                       |

For example, if a PaletteTile is drawn at `(tile_x, tile_y)`, the region under the mouse can be found like this:

```rs
fn region_under_mouse(tile: &PaletteTile, tile_x: i32, tile_y: i32, mouse_x: i32, mouse_y: i32) -> Option<u16> {
    let x = mouse_x - tile_x;
    let y = mouse_y - tile_y;
    if (x < 0) || (y < 0) {
        return None;
    }
    tile.cell_index(x as u32, y as u32, PaletteTileRenderMethod::SmallBlocks)
}
```

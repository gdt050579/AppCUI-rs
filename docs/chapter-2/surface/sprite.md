# Sprites

A Sprite is a 2D grid of [characters](../screen.md#character). Unlike a [Glyph](glyph.md), which stores only the glyph and is painted with one `CharAttribute`, every cell of a Sprite stores a full `Character`: the glyph, the foreground color, the background color, and the flags. Unlike a [BitTile](bit_tile.md) or a [PaletteTile](palette_tile.md), a Sprite is already measured in characters. One cell is drawn as one character, with no render method that packs several pixels into a character.

A Sprite has no origin and no clip area of its own. Those belong to the [Surface](../surface.md) it is drawn on.

| Type          | A cell stores                        | Colors                                  | Drawn as                     |
| ------------- | ------------------------------------ | --------------------------------------- | ---------------------------- |
| `Glyph`       | one character, or transparent (`\0`) | one `CharAttribute` for the whole glyph | one character per cell       |
| `Sprite`      | a full `Character`                   | stored in each cell                     | one character per cell       |
| `BitTile`     | one bit (set or unset)               | two colors, given when drawn            | several pixels per character |
| `PaletteTile` | a palette index, or transparent      | a palette given when drawn              | several pixels per character |

`Sprite::new(width, height)` creates a sprite filled with the default character: a space (`' '`) with a white foreground, a black background, and no flags.

## Methods

| Method                      | Purpose                                                                                                                             |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `new(width, height)`        | Creates a sprite filled with the default character                                                                                  |
| `size()`                    | Returns the size (width and height) as a `Size`                                                                                     |
| `get(x, y)`                 | Returns `Some(character)` at (x, y), or `None` if the coordinates are outside the sprite                                            |
| `set(x, y, ch)`             | Writes a character at (x, y). Coordinates outside the sprite are ignored                                                            |
| `clear(ch)`                 | Fills every cell with `ch`                                                                                                          |
| `resize(width, height, ch)` | Changes the size and fills **every** cell with `ch`, including cells that already existed                                           |
| `edit(f)`                   | Passes the sprite to a callback as a `Surface`, so it can be drawn with the surface API (see [Editing a sprite](#editing-a-sprite)) |

## Size limits

The width and height are clamped between `1` and `10000`, the same limits as a surface.

- A width or height of `0` becomes `1`.
- A width or height above `10000` becomes `10000`.
- `resize(...)` uses the same limits, then replaces every cell with the character given to it.

## Usage

A typical workflow is:

1. Create a `Sprite` with `Sprite::new(width, height)`
2. Write cells with `set(...)`, or draw into the sprite with `edit(...)`
3. Paint it with `surface.draw_sprite(...)`

The following example builds a two-cell sprite and reads a cell back:

```rs
use appcui::prelude::*;

let mut sprite = Sprite::new(2, 1);
let ch = Character::new('A', Color::Yellow, Color::Blue, CharFlags::Bold);
sprite.set(0, 0, ch);
assert_eq!(sprite.get(0, 0), Some(ch));
assert!(sprite.get(2, 0).is_none());
```

`get` returns `None` only when the coordinates are outside the sprite. A cell that still holds the default space is a real character, so `get` returns it.

## Editing a sprite

`edit` builds a temporary surface over the sprite's characters and calls the callback with it. Any drawing method of that surface (`write_char`, `write_string`, `fill_rect`, `draw_rect`, and so on) updates the sprite. When the callback returns, the sprite keeps the characters that were written and adopts the surface size.

```rs
use appcui::prelude::*;

let mut sprite = Sprite::new(12, 5);
sprite.edit(|surface| {
    surface.clear(Character::new('.', Color::Gray, Color::Black, CharFlags::None));
    surface.write_string(
        2,
        2,
        "Sprite",
        CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
        false,
    );
    surface.draw_rect(
        Rect::new(0, 0, 11, 4),
        LineType::Single,
        CharAttribute::with_color(Color::White, Color::Black),
    );
});
```

The clip and origin set inside the callback apply only while that callback runs. They are not stored on the sprite.

## Rendering sprites

Sprites are drawn through the [Surface](../surface.md) method `draw_sprite(...)`:

```rs
impl Surface {
    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) { ... }
}
```

`(x, y)` is the top-left corner of the sprite, relative to the surface origin. Characters are written from left to right, then top to bottom. Each cell is written with `write_char`, so the [clip area and the origin](clipping.md) of the destination surface apply. Cells that fall outside the clip are skipped. If the clip is not visible, nothing is drawn.

`write_char` merges the sprite character into the cell that is already there:

- a code of `'\0'` does not replace the glyph
- a `Color::Transparent` foreground or background does not replace that color
- flags are always written

This is the same rule described for [characters](../screen.md#character). A sprite cell built with `Character::with_char('X')` replaces the glyph and the flags, and leaves the colors underneath. A cell built with `Character::new(...)` and real colors replaces the glyph and both colors.

```rs
use appcui::prelude::*;

let mut surface = Surface::new(40, 12);
surface.draw_sprite(2, 2, &sprite);
```

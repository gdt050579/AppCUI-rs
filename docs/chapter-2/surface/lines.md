# Lines

Drawing lines is a common operation when building a UI. In **AppCUI** lines can be drawn in several ways:
- use special characters (single, double, thick, ASCII, rounded or Braille) for axis-aligned or free-form lines
- use a generic [character](../screen.md#character) to fill a line
- connect two points with an orthogonal (elbow) path
- connect a list of points as a polyline, with optional caps and joints

## Using special characters to draw lines

The following methods can be used to draw lines on a surface using special characters:

| Method                                | Description                                                                                                                                        |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `draw_horizontal_line(...)`           | Draws a horizontal line on the surface. The line will be drawn from left to right.                                                                 |
| `draw_vertical_line(...)`             | Draws a vertical line on the surface. The line will be drawn from top to bottom.                                                                   |
| `draw_horizontal_line_with_size(...)` | Draws a horizontal line on the surface with a specific length. The line will be drawn from left to right, starting from a given point and a width. |
| `draw_vertical_line_with_size(...)`   | Draws a vertical line on the surface with a specific length. The line will be drawn from top to bottom, starting from a given point and a width.   |
| `draw_line(...)`                      | Draws a straight line between two points using a specific line type and character attributes (Bresenham for diagonals)                             |
| `draw_orthogonal_line(...)`           | Connects two points with horizontal and vertical segments only (never a diagonal), using the corners from the given line type                      |
| `draw_polyline(...)`                  | Connects a list of points using a [`PolyLineFormat`](#polylines) (style, colors, optional caps and joints)                                         |

These methods take a parameter `line_type` (`LineType`) that specifies the visual style of the line. The line type can be one of the following values:

| Value         | Characters being used                                 |
| ------------- | ----------------------------------------------------- |
| `Single`      | `─`, `│`, `┌`, `┐`, `└`, `┘`, `├`, `┤`, `┬`, `┴`, `┼` |
| `Double`      | `═`, `║`, `╔`, `╗`, `╚`, `╝`, `╠`, `╣`, `╦`, `╩`, `╬` |
| `SingleThick` | `━`, `┃`, `┏`, `┓`, `┗`, `┛`, `┣`, `┫`, `┳`, `┻`, `╋` |
| `Border`      | `▄`, `▀`, `█`                                         |
| `Ascii`       | `\|`, `-`, `+`                                        |
| `AsciiRound`  | `\|`, `-`, `+`, `\\` , `\/`                           |
| `SingleRound` | `╭`, `╮`, `╯`, `╰`, `─`, `│`                          |
| `Braille`     | Braille characters                                    |

Example:

```rust
use appcui::graphics::{Surface, LineType, CharAttribute, Color};

let mut surface = Surface::new(100, 50);
surface.draw_vertical_line(10, 10, 20, 
                            LineType::Single, 
                            CharAttribute::with_color(Color::White, Color::Black));
```

`draw_line` works for horizontal, vertical and diagonal segments. For `Single`, `Double`, `SingleThick`, `SingleRound` and `Border` it uses a Bresenham path and picks matching box-drawing glyphs. `Ascii` and `AsciiRound` use ASCII approximations; `Braille` uses Braille dots.

## Orthogonal lines

`draw_orthogonal_line` never draws a diagonal. The two points are connected with horizontal and vertical pieces, and corners are taken from `LineType`. If the points already share an X or Y coordinate, a single straight segment is drawn. If they are the same point, nothing is drawn.

The `OrthogonalDirection` parameter chooses how the path bends:

| Value                     | Path                                                                                          |
| ------------------------- | --------------------------------------------------------------------------------------------- |
| `HorizontalFirst`         | Horizontal to the destination column, then vertical                                           |
| `VerticalFirst`           | Vertical to the destination row, then horizontal                                              |
| `HorizontalUntilMiddle`   | Horizontal to the midpoint, then vertical, then horizontal                                    |
| `VerticalUntilMiddle`     | Vertical to the midpoint, then horizontal, then vertical                                      |
| `Auto`                    | `HorizontalFirst` when the run is wider than it is tall, otherwise `VerticalFirst`            |

```
HorizontalFirst              VerticalFirst

(x1,y1) ────────┐            (x1,y1)
                │               │
                (x2,y2)         └──────── (x2,y2)

HorizontalUntilMiddle        VerticalUntilMiddle

(x1,y1) ──┐                  (x1,y1)
          │                     │
          └── (x2,y2)      ┌────┘
                           │
                           (x2,y2)
```

Example:

```rust
use appcui::prelude::*;

let mut surface = Surface::new(40, 12);
surface.draw_orthogonal_line(
    2, 2, 20, 8,
    LineType::Single,
    OrthogonalDirection::HorizontalFirst,
    charattr!("white,black"),
);
```

## Polylines

`draw_polyline` connects a list of `Point`s in order. Consecutive points are drawn with `draw_line`. Orthogonal turns (all eight incoming-to-outgoing direction pairs) get a matching corner glyph from `LineType`. Nothing is drawn if the slice has fewer than two points.

If the first and last points are the same, the path is treated as **closed**: start and end caps are skipped and the closing corner is written as well.

```
Open U (caps allowed)          Closed (first point repeated)

│        │                     ┌────────┐
│        │                     │        │
│        │                     │        │
└────────┘                     └────────┘
```

A path with several inner vertices gets a corner at every turn (here: `└`, `┐`, `└`, `┘`):

```
Automatic corners

│         │
│         │
└────┐    │
     │    │
     └────┘
```

The eight orthogonal turns resolve as follows (incoming direction → outgoing direction):

```
Right → Up          Down → Left         Up → Left           Right → Down

    │                   │                   ──┐                 ──┐
  ──┘                 ──┘                     │                   │


Left → Down         Up → Right          Down → Right        Left → Up

  ┌──                 ┌──                   │                   │
  │                   │                     └──                 └──
```

The look of the polyline is described by `PolyLineFormat`. Build one with `PolyLineFormatBuilder`:

| Builder method      | Description                                                                                                      |
| ------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `new(line_type, attr)` | Starts a builder with a line style and the attributes used for every segment                                  |
| `start_cap(...)`    | Glyph at the first point (`LineCap`). Ignored on a closed path                                                   |
| `start_attr(...)`   | Attributes of the start cap. If omitted, the line attributes are used                                            |
| `end_cap(...)`      | Glyph at the last point (`LineCap`). Ignored on a closed path                                                    |
| `end_attr(...)`     | Attributes of the end cap. If omitted, the line attributes are used                                              |
| `joint(...)`        | A custom character at every inner vertex, instead of an automatically chosen corner                              |
| `joint_attr(...)`   | Attributes of the joints (automatic corners or a custom joint). If omitted, the line attributes are used         |
| `build()`           | Builds the `PolyLineFormat` value                                                                                |

By default the format has no start cap, no end cap, and no custom joint. Orthogonal corners are then chosen automatically from the incoming and outgoing segment directions.

A custom `joint(...)` character replaces **every** inner vertex (the start and end of an open path stay as line endpoints or caps):

```
Automatic corners              Custom joint ('•')

│         │                    │         │
│         │                    │         │
└────┐    │                    •────•    │
     │    │                         │    │
     └────┘                         •────•
```

### Line caps

`LineCap` is the glyph drawn at the start or end of an **open** polyline. Caps are applied after the segments are drawn.

| Value          | Behavior                                                                                                                              |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `Auto`         | Merges the endpoint with neighboring box-drawing characters (a T-junction or similar). Intended for connectors that attach to a rectangle or another line |
| `Arrow`        | A directional arrow (`↑`, `↓`, `←`, `→`) inferred from the first or last segment                                                      |
| `Triangle`     | A directional triangle (`▲`, `▼`, `◀`, `▶`) inferred from the first or last segment                                                   |
| `Char(c)`      | The same custom character at the endpoint, regardless of direction                                                                    |

Caps are inferred from the **first** segment (start) and the **last** segment (end). On an open U that goes down, across, then up:

```
No caps            Arrow               Triangle            Char('S') / Char('E')
                                                          + joint '*'

│       │          ↓       ↑           ▼       ▲           S       E
│       │          │       │           │       │           │       │
│       │          │       │           │       │           │       │
└───────┘          └───────┘           └───────┘           *───────*
```

`LineCap::Auto` is different: it does not draw an arrow. It calls [`write_box_junction`](#box-junctions) at each end so the connector **merges** into whatever box-drawing characters are already there (`├` / `┤` below):

```
┌──────────┐
│          │
│          ├─────┐
│          │     │
│          │     │
└──────────┘     │
                 │      ┌──────────┐
                 └──────┤          │
                        │          │
                        └──────────┘
```

The first snippet below (Arrow start + Triangle end) looks like this:

```
↓       ▲
│       │
│       │
└───────┘
```

Example:

```rust
use appcui::prelude::*;

let mut surface = Surface::new(40, 12);

// Open orthogonal U with arrow and triangle caps
let format = PolyLineFormatBuilder::new(LineType::Single, charattr!("white,black"))
    .start_cap(LineCap::Arrow)
    .end_cap(LineCap::Triangle)
    .build();
surface.draw_polyline(
    &[
        Point::new(2, 1),
        Point::new(2, 6),
        Point::new(10, 6),
        Point::new(10, 1),
    ],
    &format,
);

// Closed rectangle (first point repeated at the end)
let rect = PolyLineFormatBuilder::new(LineType::Double, charattr!("aqua,black")).build();
surface.draw_polyline(
    &[
        Point::new(14, 1),
        Point::new(14, 6),
        Point::new(24, 6),
        Point::new(24, 1),
        Point::new(14, 1),
    ],
    &rect,
);

// Connector that merges into existing boxes at both ends
let connector = PolyLineFormatBuilder::new(LineType::Single, charattr!("yellow,black"))
    .start_cap(LineCap::Auto)
    .end_cap(LineCap::Auto)
    .build();
surface.draw_polyline(
    &[Point::new(10, 4), Point::new(14, 4)],
    &connector,
);
```

A `PolyLineFormat` can also be edited after it is built (`set_attr`, `set_line_type`, `set_start_cap`, `set_end_cap`, `set_joint`, and the matching `*_attr` setters). Pass `None` to a cap or joint setter to clear it.

## Box junctions

`write_box_junction(x, y)` rewrites the character at `(x, y)` as a box-drawing junction based on its four neighbors (left, above, right, below). If they form a recognized junction (T-split, cross, corner, and so on), the cell is replaced with that glyph. Colors and flags are left unchanged. If no junction matches, the cell is not modified.

This is useful after drawing overlapping lines, or after attaching a polyline with `LineCap::Auto`. Coordinates are relative to the current origin. The method also accepts a cell one character outside the clip rectangle so junctions on a border remain reachable.

A single crossing, before and after `write_box_junction` at the intersection:

```
Drawn lines (no junction)      After write_box_junction

─────│─────                    ─────┬─────
     │                              │
     │                              │
```

Several crossings on the same surface (single and double lines, plus the outer rectangle) produce T-junctions, mixed joints and a cross at every meeting point:

```
┌───┬───────────────────╥─────┐
│   │                   ║     │
│   │                   ║     │
│   │                   ║     │
├───┼───────────────────╫─────┤
│   │                   ║     │
│   │                   ║     │
╞═══╪═══════════════════╬═════╡
│   │                   ║     │
└───┴───────────────────╨─────┘
```

Call `write_box_junction` once for each meeting point (the inner crosses, the T-junctions on the border, and the mixed single/double joints).

```rust
use appcui::prelude::*;

let mut surface = Surface::new(40, 10);
let attr = charattr!("white,black");
surface.draw_line(0, 4, 30, 4, LineType::Single, attr);
surface.draw_line(0, 7, 30, 7, LineType::Double, attr);
surface.draw_line(4, 0, 4, 10, LineType::Single, attr);
surface.draw_line(24, 0, 24, 10, LineType::Double, attr);
surface.draw_rect(Rect::new(0, 0, 30, 9), LineType::Single, attr);

for (x, y) in [
    (4, 4), (24, 4), (4, 7), (24, 7),
    (0, 4), (30, 4), (4, 0), (24, 0),
    (4, 9), (24, 9), (0, 7), (30, 7),
] {
    surface.write_box_junction(x, y);
}
```

## Using a generic character to draw lines

The following methods can be used to draw lines on a surface using a generic character:

| Method                                | Description                                                                                                                                                   |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fill_horizontal_line(...)`           | Fills a horizontal line on the surface. The line will be filled from left to right with a provided [Character](../screen.md#character)                        |
| `fill_vertical_line(...)`             | Fills a vertical line on the surface. The line will be filled from top to bottom with a provided [Character](../screen.md#character)                          |
| `fill_horizontal_line_with_size(...)` | Fills a horizontal line on the surface with a specific length. The line will be filled from left to right with a provided [Character](../screen.md#character) |
| `fill_vertical_line_with_size(...)`   | Fills a vertical line on the surface with a specific length. The line will be filled from top to bottom with a provided [Character](../screen.md#character)   |
| `fill_line(...)`                      | Fills a line between two points with a given [Character](../screen.md#character) (Bresenham, including diagonals)                                              |
    

Example:

```rust
use appcui::graphics::{Surface, CharAttribute, Color, Character};

let mut surface = Surface::new(100, 50);
let c = Character::new('=', Color::White, Color::Black, CharFlags::None);
surface.fill_horizontal_line(10, 10, 20, c);
```
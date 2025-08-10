# Lines

Drawing lines is a common operation when building a UI. In **AppCUI** there are two methods that cen  be used to draw lines (vertical and horizontal) on a surface. 
- use special characters to draw the line (like single lines, double lines, etc) that are designed to be used in this context
- use a generic character to draw the line


## Using special characters to draw lines

The following methods can be used to draw lines on a surface using special characters:

| Method                                | Description                                                                                                                                        |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `draw_horizontal_line(...)`           | Draws a horizontal line on the surface. The line will be drawn from left to right.                                                                 |
| `draw_vertical_line(...)`             | Draws a vertical line on the surface. The line will be drawn from top to bottom.                                                                   |
| `draw_horizontal_line_with_size(...)` | Draws a horizontal line on the surface with a specific length. The line will be drawn from left to right, starting from a given point and a width. |
| `draw_vertical_line_with_size(...)`   | Draws a vertical line on the surface with a specific length. The line will be drawn from top to bottom, starting from a given point and a width.   |
| `draw_line(...)`                      | Draw a line between two points using a specific line type and character attributes                                                                 |

These methods take a parameter `line_type` that specifies the type of line that will be drawn. The line type can be one of the following values:

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

## Using a generic character to draw lines

The following methods can be used to draw lines on a surface using a generic character:

| Method                                | Description                                                                                                                                                   |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fill_horizontal_line(...)`           | Fills a horizontal line on the surface. The line will be filled from left to right with a provided [Character](../screen.md#character)                        |
| `fill_vertical_line(...)`             | Fills a vertical line on the surface. The line will be filled from top to bottom with a provided [Character](../screen.md#character)                          |
| `fill_horizontal_line_with_size(...)` | Fills a horizontal line on the surface with a specific length. The line will be filled from left to right with a provided [Character](../screen.md#character) |
| `fill_vertical_line_with_size(...)`   | Fills a vertical line on the surface with a specific length. The line will be filled from top to bottom with a provided [Character](../screen.md#character)   |
| `fill_line(...)`                      | FIlls a line between two points with a given [Character](../screen.md#character)                                                                              |


Example:

```rust
use appcui::graphics::{Surface, CharAttribute, Color, Character};

let mut surface = Surface::new(100, 50);
let c = Character::new('=', Color::White, Color::Black, CharFlags::None);
surface.fill_horizontal_line(10, 10, 20, c);
```
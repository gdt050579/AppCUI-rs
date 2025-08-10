use std::path::Path;

use super::CharAttribute;
use super::Character;
use super::ClipArea;
use super::Color;
use super::Cursor;
use super::Image;
use super::LineType;
use super::Point;
use super::Rect;
use super::Size;
use super::TextAlignment;
use super::TextFormat;
use crate::prelude::CharFlags;
use crate::prelude::RenderOptions;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
enum CharacterType {
    NewLine,
    Word,
    Space,
    Other,
    Undefined,
}
impl From<char> for CharacterType {
    fn from(value: char) -> Self {
        match value {
            '\n' | '\r' => CharacterType::NewLine,
            ' ' | '\t' => CharacterType::Space,
            'a'..='z' => CharacterType::Word,
            'A'..='Z' => CharacterType::Word,
            '0'..='9' => CharacterType::Word,
            '\u{80}'..=char::MAX => CharacterType::Word,
            _ => CharacterType::Other,
        }
    }
}

const MAX_SURFACE_WIDTH: u32 = 10000;
const MAX_SURFACE_HEIGHT: u32 = 10000;

/// A structure that represents a 2D surface for drawing characters and images.
/// The surface is defined as a matrix (width x height) of characters, where each character is of type [Character].
/// The surface has a size, an origin point, a clip area, and a cursor position.
/// The size of the surface is maximum 10000 x 10000 characters.
pub struct Surface {
    pub(crate) size: Size,
    pub(crate) chars: Vec<Character>,
    pub(crate) cursor: Cursor,
    origin: Point,
    clip: ClipArea,
    base_origin: Point,
    base_clip: ClipArea,
    right_most: i32,
    bottom_most: i32,
}

impl Surface {
    /// Creates a new surface with the specified width and height. The surface will be filled with space (empty) character with White foreground and Black background.
    /// The surface will have the origin set to `(0,0)` and the clip area will be the entire surface.
    /// The width and height of the surface will be clamped between `1` and `10000`.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface};
    /// let mut surface = Surface::new(100, 50);
    /// ```
    pub fn new(width: u32, height: u32) -> Surface {
        let w = width.clamp(1, MAX_SURFACE_WIDTH);
        let h = height.clamp(1, MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        let mut s = Surface {
            size: Size::new(w, h),
            origin: Point::default(),
            base_origin: Point::default(),
            chars: Vec::<Character>::with_capacity(count),
            clip: ClipArea::new(0, 0, (w - 1) as i32, (h - 1) as i32),
            base_clip: ClipArea::new(0, 0, (w - 1) as i32, (h - 1) as i32),
            cursor: Cursor::new(),
            right_most: (w - 1) as i32,
            bottom_most: (h - 1) as i32,
        };
        s.chars.resize(count, Character::default());
        s
    }

    /// Creates a new surface from a string with the specified size.
    /// The string will be written to the surface starting at position (0, 0).
    /// All characters will have white foreground and black background.
    /// If the string is longer than the surface area, it will be truncated.
    /// If the string is shorter than the surface area, the remaining area will be filled with spaces.
    ///
    /// # Arguments
    /// * `text` - The string to render on the surface
    /// * `size` - The size of the surface to create
    ///
    /// # Example
    /// ```rust
    /// use appcui::graphics::{Surface, Size};
    ///
    /// let surface = Surface::from_string("Hello World!", Size::new(20, 5));
    /// ```
    pub fn from_string(text: &str, size: Size) -> Surface {
        let mut surface = Surface::new(size.width, size.height);
        let attr = CharAttribute::with_color(Color::White, Color::Black);
        surface.write_string(0, 0, text, attr, true);
        surface
    }

    /// Returns the size of the surface (width and height).
    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }
    #[inline]
    fn coords_to_position(&self, x: i32, y: i32) -> Option<usize> {
        let x = x + self.origin.x;
        let y = y + self.origin.y;
        if !self.clip.contains(x, y) {
            return None;
        }
        let x_p = x as usize;
        let y_p = y as usize;
        let pos = y_p * (self.size.width as usize) + x_p;
        Some(pos)
    }

    /// Sets the origin of the surface. The origin is used to draw text and images relative to a specific point.    
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface};
    /// let mut surface = Surface::new(100, 50);
    /// surface.set_origin(10, 10);
    /// ```
    #[inline]
    pub fn set_origin(&mut self, x: i32, y: i32) {
        self.origin.x = x + self.base_origin.x;
        self.origin.y = y + self.base_origin.y;
    }

    /// Resets the origin of the surface to the base origin.
    #[inline]
    pub fn reset_origin(&mut self) {
        self.origin.x = self.base_origin.x;
        self.origin.y = self.base_origin.y;
    }
    #[inline]
    pub(crate) fn set_base_origin(&mut self, x: i32, y: i32) {
        self.base_origin.x = x;
        self.base_origin.y = y;
    }

    #[inline(always)]
    pub fn set_relative_clip(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.clip.set(
            i32::max(self.base_clip.left, self.base_origin.x + left),
            i32::max(self.base_clip.top, self.base_origin.y + top),
            i32::min(self.base_clip.right, self.base_origin.x + right),
            i32::min(self.base_clip.bottom, self.base_origin.y + bottom),
        );
    }

    /// Sets the clip area of the surface. The clip area is used to restrict the drawing operations to a specific area of the surface.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface};
    /// let mut surface = Surface::new(100, 50);
    /// surface.set_clip(10, 10, 20, 20);
    /// ```
    #[inline(always)]
    pub fn set_clip(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.clip.set(
            i32::max(self.base_clip.left, left),
            i32::max(self.base_clip.top, top),
            i32::min(self.base_clip.right, right),
            i32::min(self.base_clip.bottom, bottom),
        );
    }

    /// Reduces the clip area of the surface by the specified margins. This is useful when you want to draw a border around the surface.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface};
    /// let mut surface = Surface::new(100, 50);
    /// surface.set_clip(10, 10, 20, 20);
    /// // draw a border from (10,10) to (20,20)
    /// // reduce the clip area by one character to make sure
    /// // the border will not be overwritten by other drawing
    /// // operations
    /// surface.reduce_clip_by(1, 1, 1, 1);
    /// ```
    #[inline(always)]
    pub fn reduce_clip_by(&mut self, left_margin: u32, top_margin: u32, right_margin: u32, bottom_margin: u32) {
        self.set_clip(
            self.base_clip.left + left_margin as i32,
            self.base_clip.top + top_margin as i32,
            self.base_clip.right - right_margin as i32,
            self.base_clip.bottom - bottom_margin as i32,
        );
    }

    /// Resets the clip area of the surface to the base clip area.
    #[inline(always)]
    pub fn reset_clip(&mut self) {
        self.clip = self.base_clip;
    }

    #[inline]
    pub(crate) fn set_base_clip(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.base_clip.set(
            i32::max(0, left),
            i32::max(0, top),
            i32::min(self.right_most, right),
            i32::min(self.bottom_most, bottom),
        );
        self.clip.intersect_with(&self.base_clip);
    }

    #[inline]
    pub(crate) fn reset(&mut self) {
        self.set_base_clip(0, 0, self.right_most, self.bottom_most);
        self.reset_clip();
        self.set_base_origin(0, 0);
        self.reset_origin();
    }

    /// Sets the position of the cursor relativ to the origin point. If the cursor is within the clip area, it will be visible. Otherwise it will be hidden.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface};
    /// let mut surface = Surface::new(100, 50);
    /// surface.set_cursor(10, 10);
    /// ```
    #[inline]
    pub fn set_cursor(&mut self, x: i32, y: i32) {
        let x = x + self.origin.x;
        let y = y + self.origin.y;
        if self.clip.contains(x, y) {
            self.cursor.set(x as u32, y as u32);
        } else {
            self.cursor.hide();
        }
    }

    /// Hides the cursor.
    #[inline]
    pub fn hide_cursor(&mut self) {
        self.cursor.hide();
    }

    /// Writes a character at the specified position. If the position is outside the clip area, the character will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, Character, Color, CharFlags};
    /// let mut surface = Surface::new(100, 50);
    /// surface.write_char(10, 10, Character::new('A', Color::White, Color::Black, CharFlags::None));
    /// ```
    #[inline(always)]
    pub fn write_char(&mut self, x: i32, y: i32, ch: Character) {
        if let Some(pos) = self.coords_to_position(x, y) {
            self.chars[pos].set(ch);
        }
    }

    /// Returns the character at the specified position. If the position is outside the clip area, `None` will be returned.
    #[inline]
    pub fn char(&self, x: i32, y: i32) -> Option<&Character> {
        let pos = self.coords_to_position(x, y)?;
        Some(&(self.chars[pos]))
    }

    /// Clears/Fills the entire clip area with the specified character. If the clip area is not visible, the surface will not be cleared.
    pub fn clear(&mut self, ch: Character) {
        if !self.clip.is_visible() {
            return;
        }
        if (self.clip.left == 0) && (self.clip.top == 0) && (self.clip.right == self.right_most) && (self.clip.bottom == self.bottom_most) {
            // the entire screen has to be cleared
            for c in &mut self.chars {
                c.set(ch);
            }
        } else {
            // only the clip must pe cleared
            let mut pos = self.clip.left as usize;
            let sz = (self.clip.right + 1 - self.clip.left) as usize;
            pos += (self.clip.top as usize) * (self.size.width as usize);

            for _ in self.clip.top..=self.clip.bottom {
                for c in &mut self.chars[pos..(pos + sz)] {
                    c.set(ch);
                }
                pos += self.size.width as usize;
            }
        }
    }

    /// Fills a horizontal line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, Character, Color, CharFlags};
    /// let mut surface = Surface::new(100, 50);
    /// surface.fill_horizontal_line(10, 10, 20, Character::new('-', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn fill_horizontal_line(&mut self, left: i32, y: i32, right: i32, ch: Character) {
        let mut x = left;
        while x <= right {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(ch);
            }
            x += 1;
        }
    }

    /// Fills a horizontal line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    /// if the width is bigger than 0, this method will call `fill_horizontal_line` method
    pub fn fill_horizontal_line_with_size(&mut self, x: i32, y: i32, width: u32, ch: Character) {
        if width > 0 {
            self.fill_horizontal_line(x, y, x + ((width - 1) as i32), ch);
        }
    }

    /// Fills a vertical line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, Character, Color, CharFlags};
    /// let mut surface = Surface::new(100, 50);
    /// surface.fill_vertical_line(10, 10, 20, Character::new('|', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn fill_vertical_line(&mut self, x: i32, top: i32, bottom: i32, ch: Character) {
        let mut y = top;
        while y <= bottom {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(ch);
            }
            y += 1;
        }
    }

    /// Fills a vertical line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    /// if the height is bigger than 0, this method will call `fill_vertical_line` method
    pub fn fill_vertical_line_with_size(&mut self, x: i32, y: i32, height: u32, ch: Character) {
        if height > 0 {
            self.fill_vertical_line(x, y, y + ((height - 1) as i32), ch);
        }
    }

    /// Draws a vertical line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, LineType, CharAttribute, Color};
    /// let mut surface = Surface::new(100, 50);
    /// surface.draw_vertical_line(10, 10, 20,
    ///                            LineType::Single,
    ///                            CharAttribute::with_color(Color::White, Color::Black));
    /// ```
    pub fn draw_vertical_line(&mut self, x: i32, top: i32, bottom: i32, line_type: LineType, attr: CharAttribute) {
        self.fill_vertical_line(
            x,
            top,
            bottom,
            Character::new(line_type.charset().vertical, attr.foreground, attr.background, attr.flags),
        );
    }

    /// Draws a vertical line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.  
    /// if the height is bigger than 0, this method will call `draw_vertical_line` method
    pub fn draw_vertical_line_with_size(&mut self, x: i32, y: i32, height: u32, line_type: LineType, attr: CharAttribute) {
        if height > 0 {
            self.fill_vertical_line(
                x,
                y,
                y + ((height - 1) as i32),
                Character::new(line_type.charset().vertical, attr.foreground, attr.background, attr.flags),
            );
        }
    }

    /// Draws a horizontal line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, LineType, CharAttribute, Color};
    /// let mut surface = Surface::new(100, 50);
    /// surface.draw_horizontal_line(10, 10, 20,
    ///                              LineType::Single,
    ///                              CharAttribute::with_color(Color::White, Color::Black));
    /// ```
    pub fn draw_horizontal_line(&mut self, left: i32, y: i32, right: i32, line_type: LineType, attr: CharAttribute) {
        self.fill_horizontal_line(
            left,
            y,
            right,
            Character::new(line_type.charset().horizontal, attr.foreground, attr.background, attr.flags),
        );
    }

    /// Draws a horizontal line with the specified character type, color and attributes. If the line is outside the clip area, it will not be drawn.  
    /// if the height is bigger than 0, this method will call `draw_horizontal_line` method
    pub fn draw_horizontal_line_with_size(&mut self, x: i32, y: i32, width: u32, line_type: LineType, attr: CharAttribute) {
        if width > 0 {
            self.fill_horizontal_line(
                x,
                y,
                x + ((width - 1) as i32),
                Character::new(line_type.charset().horizontal, attr.foreground, attr.background, attr.flags),
            );
        }
    }

    fn draw_bresenham_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: LineType, attr: CharAttribute) {
        let line_chars = line_type.charset();
        let mut last = Point::new(x1, y1);
        let mut current = last;
        let end = Point::new(x2, y2);
        let mut ch = Character::with_attributes(' ', attr);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        ch.code = if dx >= 3 * dy {
            line_chars.horizontal
        } else if dy >= 3 * dx {
            line_chars.vertical
        } else if x2 > x1 && y2 > y1 {
            line_chars.corner_bottom_left
        } else if x2 < x1 && y2 > y1 {
            line_chars.corner_bottom_right
        } else if x2 < x1 && y2 < y1 {
            line_chars.corner_top_right
        } else {
            line_chars.corner_top_left
        };
        self.write_char(current.x, current.y, ch);

        loop {
            if current == end {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                current.x += sx;
            }
            if e2 < dx {
                err += dx;
                current.y += sy;
            }
            // draw last based on current
            let dir_x = (current.x - last.x).clamp(-1, 1);
            let dir_y = (current.y - last.y).clamp(-1, 1);
            match (dir_x, dir_y) {
                (0, 1) | (0, -1) => ch.code = line_chars.vertical,
                (1, 0) | (-1, 0) => ch.code = line_chars.horizontal,
                (1, 1) => {
                    ch.code = line_chars.corner_top_right;
                    self.write_char(last.x + 1, last.y, ch);
                    ch.code = line_chars.corner_bottom_left;
                }
                (-1, 1) => {
                    ch.code = line_chars.corner_top_left;
                    self.write_char(last.x - 1, last.y, ch);
                    ch.code = line_chars.corner_bottom_right;
                }
                (-1, -1) => {
                    ch.code = line_chars.corner_bottom_left;
                    self.write_char(last.x - 1, last.y, ch);
                    ch.code = line_chars.corner_top_right;
                }
                (1, -1) => {
                    ch.code = line_chars.corner_bottom_right;
                    self.write_char(last.x + 1, last.y, ch);
                    ch.code = line_chars.corner_top_left;
                }
                _ => ch.code = 0 as char,
            }
            self.write_char(current.x, current.y, ch);
            last = current;
        }
    }
    fn draw_ascii_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, attr: CharAttribute) {
        let mut last = Point::new(x1, y1);
        let mut current = last;
        let end = Point::new(x2, y2);
        let mut ch = Character::with_attributes(' ', attr);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        ch.code = if dx >= 3 * dy {
            '-'
        } else if dy >= 3 * dx {
            '|'
        } else if x2 > x1 {
            '\\'
        } else {
            '/'
        };
        self.write_char(current.x, current.y, ch);

        loop {
            if current == end {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                current.x += sx;
            }
            if e2 < dx {
                err += dx;
                current.y += sy;
            }
            // draw last based on current
            let dir_x = (current.x - last.x).clamp(-1, 1);
            let dir_y = (current.y - last.y).clamp(-1, 1);
            match (dir_x, dir_y) {
                (0, 1) | (0, -1) => ch.code = '|',
                (1, 0) | (-1, 0) => ch.code = '-',
                (1, 1) | (-1, -1) => ch.code = '\\',
                (-1, 1) | (1, -1) => ch.code = '/',
                _ => ch.code = 0 as char,
            }
            self.write_char(current.x, current.y, ch);
            last = current;
        }
    }
    pub fn draw_braille_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, attr: CharAttribute) {
        // Anchor to the center of each Braille cell: (2x+1, 4y+2)
        let mut px = x1 * 2 + 1;
        let mut py = y1 * 4 + 2;
        let tx = x2 * 2 + 1;
        let ty = y2 * 4 + 2;
        let mut ch = Character::with_attributes(' ', attr);

        let dx = (tx - px).abs();
        let dy = (ty - py).abs();
        let sx = if px < tx { 1 } else { -1 };
        let sy = if py < ty { 1 } else { -1 };
        let mut err = dx - dy;
        let mut last_point = Point::new(px.div_euclid(2), py.div_euclid(4));
        let mut current_bit_set = 0;

        loop {
            let new_point = Point::new(px.div_euclid(2), py.div_euclid(4));
            let sx_sub = px.rem_euclid(2);
            let sy_sub = py.rem_euclid(4);
            if last_point != new_point {
                ch.code = char::from_u32(0x2800 + current_bit_set).unwrap_or('\u{2800}');
                self.write_char(last_point.x, last_point.y, ch);
                last_point = new_point;
                current_bit_set = 0;
            }
            current_bit_set |= match (sx_sub, sy_sub) {
                (0, 0) => 1 << 0,
                (0, 1) => 1 << 1,
                (0, 2) => 1 << 2,
                (0, 3) => 1 << 6,
                (1, 0) => 1 << 3,
                (1, 1) => 1 << 4,
                (1, 2) => 1 << 5,
                (1, 3) => 1 << 7,
                _ => 0,
            };

            if px == tx && py == ty {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                px += sx;
            }
            if e2 < dx {
                err += dx;
                py += sy;
            }
        }
        ch.code = char::from_u32(0x2800 + current_bit_set).unwrap_or('\u{2800}');
        self.write_char(last_point.x, last_point.y, ch);
    }

    /// Draws a straight line between two points `(x1, y1)` and `(x2, y2)`
    /// using the specified line style (`LineType`) and character attributes.
    ///
    /// This method is similar to [`fill_line`](Self::fill_line), but instead of
    /// filling the line with a single [`Character`], it automatically chooses
    /// the appropriate glyphs for each segment based on the given [`LineType`]
    /// (e.g., single, double, thick, ASCII, rounded) and applies the specified
    /// [`CharAttribute`] (e.g., color, boldness, underline).
    ///
    /// # Parameters
    /// - `x1`, `y1`: Starting point coordinates.
    /// - `x2`, `y2`: Ending point coordinates.
    /// - `line_type`: The [`LineType`] variant to use for rendering the line.
    /// - `attr`: The [`CharAttribute`] to apply to each segment of the line.
    ///
    /// # Examples
    /// ```rust
    /// // Draw a horizontal single-line border in bold
    /// surface.draw_line(0, 0, 10, 0, LineType::Single, charattr!("white,Black"));
    ///
    /// // Draw a vertical double-line in red
    /// surface.draw_line(5, 2, 5, 8, LineType::Double, charattr!("red,Black");
    /// ```
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, line_type: LineType, attr: CharAttribute) {
        match line_type {
            LineType::Single | LineType::SingleThick | LineType::Double | LineType::SingleRound | LineType::Border => {
                self.draw_bresenham_line(x1, y1, x2, y2, line_type, attr)
            }
            LineType::Ascii | LineType::AsciiRound => self.draw_ascii_line(x1, y1, x2, y2, attr),
            LineType::Braille => self.draw_braille_line(x1, y1, x2, y2, attr),
        };
    }

    /// Draws a straight line between two points `(x1, y1)` and `(x2, y2)`
    /// on the surface, filling each point along the path with the given character.
    ///
    /// This method implements an integer-based **Bresenham's line algorithm**,
    /// which efficiently determines the set of coordinates that best approximate
    /// a straight line between two points in a grid. It works for all line
    /// orientations — horizontal, vertical, and diagonal
    ///
    /// # Parameters
    /// - `x1`, `y1`: Starting point coordinates.
    /// - `x2`, `y2`: Ending point coordinates.
    /// - `ch`: The [`Character`] to draw along the line.
    ///
    /// # Examples
    /// ```rust
    /// // Draws a diagonal line from (0, 0) to (5, 3) using '*'
    /// surface.fill_line(0, 0, 5, 3, Character::new('*', Color::White, Color::Bleck, CharFlags::None));
    ///
    /// // Draws a vertical line from (2, 1) to (2, 5)
    /// surface.fill_line(2, 1, 2, 5, char!("'|',white,black"));
    /// ```   
    pub fn fill_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, ch: Character) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut cx = x1;
        let mut cy = y1;

        self.write_char(cx, cy, ch);

        loop {
            if (cx == x2) && (cy == y2) {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                cx += sx;
            }
            if e2 < dx {
                err += dx;
                cy += sy;
            }
            self.write_char(cx, cy, ch);
        }
    }

    /// Draws a rectangle with the specified character type, color and attributes. If the rectangle is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::*;
    ///
    /// let mut surface = Surface::new(100, 50);
    /// let r = Rect::new(10, 10, 20, 20);
    /// surface.draw_rect(r, LineType::Single, CharAttribute::with_color(Color::White, Color::Black));
    /// ```
    pub fn draw_rect(&mut self, rect: Rect, line_type: LineType, attr: CharAttribute) {
        let left = rect.left();
        let right = rect.right();
        let top = rect.top();
        let bottom = rect.bottom();

        let line_chars = line_type.charset();
        let mut ch = Character::new(' ', attr.foreground, attr.background, attr.flags);
        ch.code = line_chars.horizontal_on_top;
        self.fill_horizontal_line(left, top, right, ch);
        ch.code = line_chars.horizontal_on_bottom;
        self.fill_horizontal_line(left, bottom, right, ch);
        ch.code = line_chars.vertical_on_left;
        self.fill_vertical_line(left, top, bottom, ch);
        ch.code = line_chars.vertical_on_right;
        self.fill_vertical_line(right, top, bottom, ch);
        ch.code = line_chars.corner_top_left;
        self.write_char(left, top, ch);
        ch.code = line_chars.corner_top_right;
        self.write_char(right, top, ch);
        ch.code = line_chars.corner_bottom_right;
        self.write_char(right, bottom, ch);
        ch.code = line_chars.corner_bottom_left;
        self.write_char(left, bottom, ch);
    }

    /// Fills a rectangle with the specified character type, color and attributes. If the rectangle is outside the clip area, it will not be drawn.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::*;
    ///
    /// let mut surface = Surface::new(100, 50);
    /// let r = Rect::new(10, 10, 20, 20);
    /// surface.fill_rect(r, Character::new(' ', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn fill_rect(&mut self, rect: Rect, ch: Character) {
        let left = rect.left();
        let right = rect.right();
        let top = rect.top();
        let bottom = rect.bottom();
        for x in left..=right {
            for y in top..=bottom {
                if let Some(pos) = self.coords_to_position(x, y) {
                    self.chars[pos].set(ch);
                }
            }
        }
    }

    pub fn draw_surface(&mut self, x: i32, y: i32, surface: &Surface) {
        if !self.clip.is_visible() {
            return;
        }
        let mut index = 0usize;
        for s_y in 0..=surface.bottom_most {
            for s_x in 0..=surface.right_most {
                self.write_char(x + s_x, y + s_y, surface.chars[index]);
                index += 1;
            }
        }
    }

    /// Writes a string at the specified position, from left to right using a specific character attribute. If the text is outside the clip area, it will not be drawn.
    /// The `multi-line` parameter specifices if the text should interpret new line characters as a new line or not. if set to `false` the code of this method is optimized to write the text faster.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, CharAttribute, Color};
    ///
    /// let mut surface = Surface::new(100, 50);
    /// surface.write_string(10, 10,
    ///                      "Hello World!",
    ///                      CharAttribute::with_color(Color::White, Color::Black),
    ///                      false);
    /// ```
    pub fn write_string(&mut self, x: i32, y: i32, text: &str, attr: CharAttribute, multi_line: bool) {
        let mut c = Character::new(' ', attr.foreground, attr.background, attr.flags);
        if !multi_line {
            // single line support
            if !self.clip.contains_y(y + self.origin.y) {
                return; // no need to draw
            }
            let mut p_x = x;
            for ch in text.chars() {
                if let Some(pos) = self.coords_to_position(p_x, y) {
                    c.code = ch;
                    self.chars[pos].set(c);
                }
                p_x += 1;
            }
        } else {
            let mut p_x = x;
            let mut p_y = y;
            for ch in text.chars() {
                if (ch == '\n') || (ch == '\r') {
                    p_y += 1;
                    p_x = x;
                    continue;
                }
                if let Some(pos) = self.coords_to_position(p_x, p_y) {
                    c.code = ch;
                    self.chars[pos].set(c);
                }
                p_x += 1;
            }
        }
    }

    /// Writes an ASCII buffer at the specified position, from left to right using a specific character attribute. If the text is outside the clip area, it will not be drawn.  
    /// The `multi-line` parameter specifices if the text should interpret new line characters as a new line or not. if set to `false` the code of this method is optimized to write the text faster.   
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::{Surface, CharAttribute, Color};
    ///
    /// let mut surface = Surface::new(100, 50);
    /// surface.write_ascii(10, 10,
    ///                    b"Hello World!",
    ///                    CharAttribute::with_color(Color::White, Color::Black),
    ///                    false);
    /// ```
    pub fn write_ascii(&mut self, x: i32, y: i32, ascii_buffer: &[u8], attr: CharAttribute, multi_line: bool) {
        let mut c = Character::with_attributes(' ', attr);
        if !multi_line {
            // single line support
            if !self.clip.contains_y(y + self.origin.y) {
                return; // no need to draw
            }
            let mut p_x = x;
            for ch in ascii_buffer {
                if let Some(pos) = self.coords_to_position(p_x, y) {
                    c.code = *ch as char;
                    self.chars[pos].set(c);
                }
                p_x += 1;
            }
        } else {
            let mut p_x = x;
            let mut p_y = y;
            for ch in ascii_buffer {
                if (*ch == b'\n') || (*ch == b'\r') {
                    p_y += 1;
                    p_x = x;
                    continue;
                }
                if let Some(pos) = self.coords_to_position(p_x, p_y) {
                    c.code = *ch as char;
                    self.chars[pos].set(c);
                }
                p_x += 1;
            }
        }
    }

    fn write_text_single_line(&mut self, text: &str, y: i32, chars_count: u16, ch_index: usize, format: &TextFormat, width: u16) {
        if !self.clip.contains_y(y + self.origin.y) {
            return; // no need to draw
        }
        let mut x = match format.align {
            TextAlignment::Left => format.x,
            TextAlignment::Center => format.x - (chars_count / 2) as i32,
            TextAlignment::Right => format.x + 1 - chars_count as i32,
        };
        let width = u16::min(width, chars_count);
        let left_margin = match format.align {
            TextAlignment::Left => format.x,
            TextAlignment::Center => format.x - (width / 2) as i32,
            TextAlignment::Right => format.x + 1 - width as i32,
        };
        let right_margin = left_margin + (width as i32);
        let mut c = Character::with_attributes(' ', format.char_attr);

        if format.has_hotkey() {
            let hkpos = format.hotkey_pos as usize;
            let mut cpos = ch_index;
            for ch in text.chars() {
                if (x >= left_margin) && (x < right_margin) {
                    if let Some(pos) = self.coords_to_position(x, y) {
                        if cpos == hkpos {
                            self.chars[pos].set(Character::with_attributes(ch, format.hotkey_attr));
                        } else {
                            c.code = ch;
                            self.chars[pos].set(c);
                        }
                    }
                }
                x += 1;
                cpos += 1;
            }
        } else {
            for ch in text.chars() {
                if (x >= left_margin) && (x < right_margin) {
                    if let Some(pos) = self.coords_to_position(x, y) {
                        c.code = ch;
                        self.chars[pos].set(c);
                    }
                }
                x += 1;
            }
        }
    }
    fn write_text_multi_line_no_wrap(&mut self, text: &str, format: &TextFormat) {
        let mut y = format.y;
        let mut start_ofs = 0usize;
        let mut chars_count = 0u16;
        let mut ch_index = 0usize;
        for (index, ch) in text.char_indices() {
            if (ch == '\n') || (ch == '\r') {
                if chars_count > 0 {
                    self.write_text_single_line(&text[start_ofs..index], y, chars_count, ch_index, format, chars_count);
                }
                y += 1;
                ch_index += (chars_count as usize) + 1;
                chars_count = 0;
                start_ofs = index + 1;
            } else {
                chars_count += 1;
            }
        }
        if chars_count > 0 {
            self.write_text_single_line(&text[start_ofs..], y, chars_count, ch_index, format, chars_count);
        }
    }
    fn write_text_multi_line_character_wrap(&mut self, text: &str, format: &TextFormat, width: u16) {
        if width == 0 {
            return; // nothing to draw
        }
        let mut y = format.y;
        let mut start_ofs = 0usize;
        let mut chars_count = 0u16;
        let mut ch_index = 0usize;
        for (index, ch) in text.char_indices() {
            if (ch == '\n') || (ch == '\r') {
                if chars_count > 0 {
                    self.write_text_single_line(&text[start_ofs..index], y, chars_count, ch_index, format, width);
                }
                y += 1;
                ch_index += (chars_count as usize) + 1;
                chars_count = 0;
                start_ofs = index + 1;
                continue;
            }
            if chars_count == width {
                self.write_text_single_line(&text[start_ofs..index], y, chars_count, ch_index, format, width);
                y += 1;
                ch_index += chars_count as usize;
                chars_count = 1; // current character
                start_ofs = index;
                continue;
            }
            chars_count += 1;
        }
        if chars_count > 0 {
            self.write_text_single_line(&text[start_ofs..], y, chars_count, ch_index, format, width);
        }
    }
    fn write_text_multi_line_word_wrap(&mut self, text: &str, format: &TextFormat, width: u16) {
        if width == 0 {
            return; // nothing to draw
        }
        let mut y = format.y;
        let mut start_ofs = 0usize;
        let mut end_ofs = 0usize;
        let mut next_ofs = 0usize;

        let mut chars_count_end_ofs = 0u16;
        let mut chars_count_next_ofs = 0u16;

        let mut chars_count = 0u16;
        let mut ch_index = 0usize;
        let mut current_char_index = 0usize;
        let mut current_char_index_on_next = 0usize;

        let mut last_char_type = CharacterType::Undefined;
        let mut strip_spaces = false;

        for (offset, ch) in text.char_indices() {
            let char_type = CharacterType::from(ch);
            current_char_index += 1;
            if strip_spaces {
                if char_type == CharacterType::Space {
                    continue;
                }
                start_ofs = offset;
                end_ofs = offset;
                chars_count = 0;
                strip_spaces = false;
                ch_index = current_char_index - 1;
            }
            if ((last_char_type == CharacterType::Word) && (last_char_type != char_type)) || (last_char_type == CharacterType::Other) {
                // we have either a word or a punctuation mark that is finished
                end_ofs = offset;
                chars_count_end_ofs = chars_count;
            }
            if ((char_type == CharacterType::Word) && (last_char_type != char_type)) || (char_type == CharacterType::Other) {
                // we have a possible new start (either an word or a punctuation mark)
                next_ofs = offset;
                chars_count_next_ofs = chars_count;
                current_char_index_on_next = current_char_index - 1;
            }

            if (char_type == CharacterType::NewLine) || (chars_count == width) {
                if end_ofs <= start_ofs {
                    //println!("Word bigger than the line (start={start_ofs}, end={end_ofs}, index={index})");
                    end_ofs = offset;
                    chars_count_end_ofs = chars_count;
                }
                // print the part
                // println!("start={} end={} =>'{}' , next={} index={} =>'{}'",start_ofs,end_ofs,&text[start_ofs..end_ofs],next_ofs,index,&text[next_ofs..index]);
                self.write_text_single_line(&text[start_ofs..end_ofs], y, chars_count_end_ofs, ch_index, format, width);
                if char_type == CharacterType::NewLine {
                    start_ofs = offset + 1;
                    ch_index = current_char_index;
                    chars_count = 0;
                } else if next_ofs >= end_ofs {
                    start_ofs = next_ofs;
                    ch_index = current_char_index_on_next;
                    chars_count = 1 + chars_count - chars_count_next_ofs;
                } else {
                    start_ofs = offset;
                    ch_index = current_char_index - 1;
                    chars_count = 1; // current char
                    strip_spaces = char_type == CharacterType::Space;
                }
                last_char_type = char_type;
                //println!("   ->new_start={}, chars_count={}, index={} [{strip_spaces}]=> '{}'\n",start_ofs,chars_count,index,&text[start_ofs..]);
                y += 1;
                continue;
            }
            last_char_type = char_type;
            chars_count += 1;
        }
        if chars_count > 0 {
            self.write_text_single_line(&text[start_ofs..], y, chars_count, ch_index, format, width);
        }
    }

    /// Writes a text using a specific format that allows specifying alignment, hotkey position and attributes, width, and height.
    ///
    /// Example:
    /// ```rust
    /// use appcui::graphics::*;
    ///
    /// let mut surface = Surface::new(100, 50);
    /// let format = TextFormatBuilder::new()
    ///                 .position(10, 10)
    ///                 .attribute(CharAttribute::with_color(Color::White, Color::Black))
    ///                 .align(TextAlignment::Left)
    ///                 .build();
    /// surface.write_text("Hello World!", &format);
    /// ```
    pub fn write_text(&mut self, text: &str, format: &TextFormat) {
        match format.wrap_type {
            super::text_format::WrapType::WordWrap(width) => self.write_text_multi_line_word_wrap(text, format, width),
            super::text_format::WrapType::CharacterWrap(width) => {
                self.write_text_multi_line_character_wrap(text, format, width);
            }
            super::text_format::WrapType::MultiLine => {
                self.write_text_multi_line_no_wrap(text, format);
            }
            super::text_format::WrapType::SingleLine => {
                let chars_count = if format.has_chars_count() {
                    format.chars_count
                } else {
                    text.chars().count() as u16
                };
                self.write_text_single_line(text, format.y, chars_count, 0, format, u16::MAX);
            }
            super::text_format::WrapType::SingleLineWrap(width) => {
                let chars_count = if format.has_chars_count() {
                    format.chars_count
                } else {
                    text.chars().count() as u16
                };
                self.write_text_single_line(text, format.y, chars_count, 0, format, width);
            } // TextWrap::Character => self.write_text_multi_line_character_wrap(text, format),
              // TextWrap::Word => self.write_text_multi_line_word_wrap(text, format),
        }
        // if format.is_multi_line() {
        //     if format.has_width() {
        //         match format.text_wrap {
        //             TextWrap::Character => ,
        //             TextWrap::Word => ,
        //         }
        //     } else {
        //         self.write_text_multi_line_no_wrap(text, format);
        //     }
        // } else {

        //     self.write_text_single_line(text, format.y, chars_count, 0, format);
        // }
    }

    /// Draws an image at the specified position using a RenderOptions structure to decide how to paint it.
    ///
    /// Example:
    /// ```rust
    /// use appcui::prelude::*;
    /// use std::str::FromStr;
    ///
    /// let mut surface = Surface::new(100, 50);
    /// let heart = r#"
    ///         |..rr.rr..|
    ///         |.rrrrrrr.|
    ///         |.rrrrrrr.|
    ///         |..rrrrr..|
    ///         |...rrr...|
    ///         |....r....|"#;
    /// let image = Image::from_str(heart).unwrap();
    /// let opt = RenderOptionsBuilder::new()
    ///                                .character_set(image::CharacterSet::LargeBlocks)
    ///                                .build();
    /// surface.draw_image(10, 10, &image, &opt);
    /// ```
    pub fn draw_image(&mut self, x: i32, y: i32, image: &Image, render_options: &RenderOptions) {
        image.paint(self, x, y, render_options);
    }
    pub(crate) fn resize(&mut self, size: Size) {
        let w = size.width.clamp(1, MAX_SURFACE_WIDTH);
        let h = size.height.clamp(1, MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        self.chars.clear();
        self.chars.reserve(count);
        self.chars.resize(count, Character::default());
        self.right_most = (w as i32) - 1;
        self.bottom_most = (h as i32) - 1;
        self.size.width = w;
        self.size.height = h;
        self.reset();
    }

    fn serialize_color(color: Color, output: &mut Vec<u8>) {
        match color {
            Color::Black => output.push(0),
            Color::DarkBlue => output.push(1),
            Color::DarkGreen => output.push(2),
            Color::Teal => output.push(3),
            Color::DarkRed => output.push(4),
            Color::Magenta => output.push(5),
            Color::Olive => output.push(6),
            Color::Silver => output.push(7),
            Color::Gray => output.push(8),
            Color::Blue => output.push(9),
            Color::Green => output.push(10),
            Color::Aqua => output.push(11),
            Color::Red => output.push(12),
            Color::Pink => output.push(13),
            Color::Yellow => output.push(14),
            Color::White => output.push(15),
            Color::Transparent => output.push(16),
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                output.push(17);
                output.extend_from_slice(&r.to_le_bytes());
                output.extend_from_slice(&g.to_le_bytes());
                output.extend_from_slice(&b.to_le_bytes());
            }
        }
    }
    pub(super) fn deserialize_color(buffer: &[u8]) -> Option<(Color, usize)> {
        match buffer[0] {
            0 => Some((Color::Black, 1)),
            1 => Some((Color::DarkBlue, 1)),
            2 => Some((Color::DarkGreen, 1)),
            3 => Some((Color::Teal, 1)),
            4 => Some((Color::DarkRed, 1)),
            5 => Some((Color::Magenta, 1)),
            6 => Some((Color::Olive, 1)),
            7 => Some((Color::Silver, 1)),
            8 => Some((Color::Gray, 1)),
            9 => Some((Color::Blue, 1)),
            10 => Some((Color::Green, 1)),
            11 => Some((Color::Aqua, 1)),
            12 => Some((Color::Red, 1)),
            13 => Some((Color::Pink, 1)),
            14 => Some((Color::Yellow, 1)),
            15 => Some((Color::White, 1)),
            16 => Some((Color::Transparent, 1)),
            17 => {
                if buffer.len() < 4 {
                    None
                } else {
                    let r = buffer[1];
                    let g = buffer[2];
                    let b = buffer[3];
                    Some((Color::from_rgb(r, g, b), 4))
                }
            }
            _ => None,
        }
    }
    /// Serializes the surface to a byte buffer. The buffer will contain the magic number, version, size, and character buffer.
    /// The format is as follows:
    /// - Magic number: 3 bytes (SRF)
    /// - Version: 1 byte
    /// - Size: 8 bytes (width and height, each 4 bytes, little-endian)
    /// - Character buffer: for each character:
    ///    - Code: 4 bytes (u32, little-endian)
    ///    - Flags: 2 bytes (u16, little-endian)
    ///    - Foreground color: 1 byte (u8) - in case of RGB colors it will be 17, followed by 3 bytes for the RGB values
    ///    - Background color: 1 byte (u8) - in case of RGB colors it will be 17, followed by 3 bytes for the RGB values
    pub fn serialize_to_buffer(&self, output: &mut Vec<u8>) {
        output.clear();
        // magic
        output.push(b'S');
        output.push(b'R');
        output.push(b'F');
        // version
        output.push(1);
        // size
        output.extend_from_slice(self.size.width.to_le_bytes().as_slice());
        output.extend_from_slice(self.size.height.to_le_bytes().as_slice());
        // character buffer
        for ch in &self.chars {
            output.extend_from_slice((ch.code as u32).to_le_bytes().as_slice());
            output.extend_from_slice(ch.flags.get_value().to_le_bytes().as_slice());
            Self::serialize_color(ch.foreground, output);
            Self::serialize_color(ch.background, output);
        }
        /*
        Alternativ:
        - caracterul e scris UTF-8 (1-4 bytes)
        - 1 caracter (primii 3 biti)
            0 - culoare pe 1 byte (16 cu 16) /
            1 - culoare pe 2 bytes (16 si 16),
            2 - culoarea exact ca precedentul,
            4 - acelasi foreground
            5 - acelasi background
        - urmatorii 3 biti sunt flags:
            0 - pe un singur octet
            1 - pe 2 octeti
            2 - la fel ca precedentul
         */
    }

    /// Serializes the surface to a byte buffer and saves it to the specified file path.
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut output = Vec::new();
        self.serialize_to_buffer(&mut output);
        std::fs::write(path, output)?;
        Ok(())
    }

    /// Creates a new surface from a byte buffer. The buffer must contain the magic number, version, size, and character buffer.
    pub fn from_buffer(buffer: &[u8]) -> Result<Surface, String> {
        if buffer.len() < 12 {
            return Err("Buffer is too small to be a valid surface".to_string());
        }
        if buffer[0] != b'S' || buffer[1] != b'R' || buffer[2] != b'F' {
            return Err("Invalid magic number".to_string());
        }
        if buffer[3] != 1 {
            return Err("Unsupported version".to_string());
        }
        let width = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let height = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
        let mut surface = Surface::new(width, height);
        let mut pos = 12;
        let len = buffer.len();
        for ch in &mut surface.chars {
            if pos + 6 >= len {
                return Err("Buffer is too small for character data".to_string());
            }
            let code = u32::from_le_bytes([buffer[pos], buffer[pos + 1], buffer[pos + 2], buffer[pos + 3]]);
            ch.code = char::from_u32(code).expect("Invalid UTF-8 character");
            let flags = u16::from_le_bytes([buffer[pos + 4], buffer[pos + 5]]);
            ch.flags = CharFlags::from_value(flags).expect("Invalid combination of flags");
            pos += 6;
            if pos >= len {
                return Err("Buffer is too small for foreground character colors".to_string());
            }
            let (fore, sz) = Self::deserialize_color(&buffer[pos..]).expect("Invalid foreground color");
            ch.foreground = fore;
            pos += sz;
            if pos >= len {
                return Err("Buffer is too small for background character colors".to_string());
            }
            let (back, sz) = Self::deserialize_color(&buffer[pos..]).expect("Invalid background color");
            ch.background = back;
            pos += sz;
        }
        Ok(surface)
    }

    /// Creates a new surface from a file. The file must contain the magic number, version, size, and character buffer.
    pub fn from_file(path: &Path) -> Result<Surface, String> {
        let buffer = std::fs::read(path).map_err(|e| e.to_string())?;
        Self::from_buffer(&buffer)
    }
}

use std::u16;

use super::image;
use super::Renderer;

use super::CharAttribute;
use super::Rect;
use super::Size;
use super::Character;
use super::ClipArea;
use super::Cursor;
use super::Image;
use super::LineType;
use super::Point;
use super::TextAlignament;
use super::TextFormat;

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

pub struct Surface {
    pub(crate) size: Size,
    pub(crate) chars: Vec<Character>,
    pub(crate) cursor: Cursor,
    origin: Point,
    base_origin: Point,
    clip: ClipArea,
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
    pub fn fill_vertical_line_width_size(&mut self, x: i32, y: i32, height: u32, ch: Character) {
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
            Character::new(line_type.get_chars().vertical, attr.foreground, attr.background, attr.flags),
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
                Character::new(line_type.get_chars().vertical, attr.foreground, attr.background, attr.flags),
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
            Character::new(line_type.get_chars().horizontal, attr.foreground, attr.background, attr.flags),
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
                Character::new(line_type.get_chars().horizontal, attr.foreground, attr.background, attr.flags),
            );
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

        let line_chars = line_type.get_chars();
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
            TextAlignament::Left => format.x,
            TextAlignament::Center => format.x - (chars_count / 2) as i32,
            TextAlignament::Right => format.x + 1 - chars_count as i32,
        };
        let width = u16::min(width, chars_count);
        let left_margin = match format.align {
            TextAlignament::Left => format.x,
            TextAlignament::Center => format.x - (width / 2) as i32,
            TextAlignament::Right => format.x + 1 - width as i32,
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
    ///                 .align(TextAlignament::Left)
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
            }
            // TextWrap::Character => self.write_text_multi_line_character_wrap(text, format),
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

    /// Draws an image at the specified position. The image will be drawn using the specified rendering method and scale method.
    /// The rendering method can be `SmallBlocks`, `LargeBlocks64Colors`, `GrayScale` or `AsciiArt`.
    ///
    /// Example:
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let mut surface = Surface::new(100, 50);
    /// let heart = r#"
    ///         |..rr.rr..|
    ///         |.rrrrrrr.|
    ///         |.rrrrrrr.|
    ///         |..rrrrr..|
    ///         |...rrr...|
    ///         |....r....|"#;
    /// let image = Image::with_str(heart).unwrap();
    /// surface.draw_image(10, 10, &image,
    ///                            image::RenderMethod::LargeBlocks64Colors,
    ///                            image::Scale::NoScale);
    /// ```
    pub fn draw_image(&mut self, x: i32, y: i32, image: &Image, rendering_method: image::RenderMethod, scale_method: image::Scale) {
        let rap = scale_method as u32;
        match rendering_method {
            image::RenderMethod::SmallBlocks => Renderer::render_with_small_blocks(self, image, x, y, rap),
            image::RenderMethod::LargeBlocks64Colors => Renderer::render_with_large_blocks_64(self, image, x, y, rap),
            image::RenderMethod::GrayScale => Renderer::render_with_gray_scale(self, image, x, y, rap),
            image::RenderMethod::AsciiArt => Renderer::render_ascii_art(self, image, x, y, rap),
        }
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
}

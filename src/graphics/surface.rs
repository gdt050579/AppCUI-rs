use super::Attribute;
use super::Character;
use super::Color;
use super::LineType;

pub struct Surface {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) translate_x: i32,
    pub(crate) translate_y: i32,
    pub(crate) chars: Vec<Character>,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Surface {
        let w = width.clamp(5, 10000);
        let h = height.clamp(5, 10000);
        let count = (w as usize) * (h as usize);
        let mut s = Surface {
            width: w,
            height: h,
            translate_x: 0,
            translate_y: 0,
            chars: Vec::<Character>::with_capacity(count),
        };
        let c = Character::new(' ', Color::White, Color::Black, Attribute::None);
        for _ in 0..count {
            s.chars.push(c);
        }
        return s;
    }
    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }
    #[inline]
    pub fn get_height(&self) -> u32 {
        self.height
    }
    #[inline]
    fn coords_to_position(&self, x: i32, y: i32) -> Option<usize> {
        let x = x + self.translate_x;
        let y = y + self.translate_y;
        if (x < 0) || (y < 0) {
            return None;
        };
        let x_p = x as u32;
        let y_p = y as u32;
        if (x_p >= self.width) || (y_p >= self.height) {
            return None;
        }
        let pos = (y_p as usize) * (self.width as usize) + (x_p as usize);
        return Some(pos);
    }
    #[inline]
    pub fn set_origin(&mut self, x: i32, y: i32) {
        self.translate_x = x;
        self.translate_y = y;
    }

    #[inline]
    pub fn reset_origin(&mut self) {
        self.translate_x = 0;
        self.translate_y = 0;
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, ch: Character) {
        if let Some(pos) = self.coords_to_position(x, y) {
            self.chars[pos].set(&ch);
        }
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<&Character> {
        let pos = self.coords_to_position(x, y)?;
        return Some(&(self.chars[pos]));
    }

    pub fn clear(&mut self, ch: Character) {
        for c in &mut self.chars {
            c.set(&ch);
        }
    }

    pub fn fill_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, ch: Character) {
        if (left > right) || (top > bottom) {
            return;
        }
        for x in left..=right {
            for y in top..=bottom {
                if let Some(pos) = self.coords_to_position(x, y) {
                    self.chars[pos].set(&ch);
                }
            }
        }
    }

    pub fn fill_rect_with_size(&mut self, x: i32, y: i32, width: u32, height: u32, ch: Character) {
        if (width > 0) && (height > 0) {
            self.fill_rect(x, y, x + (width as i32) - 1, y + (height as i32) - 1, ch)
        }
    }

    pub fn fill_horizontal_line(&mut self, x: i32, y: i32, width: u32, ch: Character) {
        let mut sz = width;
        let mut x = x;
        while sz > 0 {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(&ch);
            }
            x += 1;
            sz -= 1;
        }
    }

    pub fn fill_vertical_line(&mut self, x: i32, y: i32, height: u32, ch: Character) {
        let mut sz = height;
        let mut y = y;
        while sz > 0 {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(&ch);
            }
            y += 1;
            sz -= 1;
        }
    }

    pub fn draw_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, line_type: LineType) {
        if (left > right) || (bottom > top) {
            return;
        }
        todo!();
    }
}

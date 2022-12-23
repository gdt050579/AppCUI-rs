use super::CharAttribute;
use super::Character;
use super::ClipArea;
use super::Color;
use super::LineType;

pub struct Surface {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) translate_x: i32,
    pub(crate) translate_y: i32,
    pub(crate) chars: Vec<Character>,
    clip: ClipArea,
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
            clip: ClipArea::new(0, 0, (w - 1) as i32, (h - 1) as i32),
        };
        let c = Character::new(' ', Color::White, Color::Black, super::CharFlags::None);
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
        if self.clip.contains(x, y) == false {
            return None;
        }
        let x_p = x as usize;
        let y_p = y as usize;
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
    pub fn set_clip(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.clip.set(
            i32::max(0, left),
            i32::max(0, top),
            i32::min((self.width - 1) as i32, right),
            i32::min((self.height - 1) as i32, bottom),
        );
    }
    #[inline]
    pub fn reset_clip(&mut self) {
        self.clip
            .set(0, 0, (self.width - 1) as i32, (self.height - 1) as i32);
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

    pub fn fill_horizontal_line(&mut self, left: i32, y: i32, right: i32, ch: Character) {
        let mut x = left;
        while x <= right {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(&ch);
            }
            x += 1;
        }
    }
    pub fn fill_horizontal_line_with_size(&mut self, x: i32, y: i32, width: u32, ch: Character) {
        if width > 0 {
            self.fill_horizontal_line(x, y, x + ((width - 1) as i32), ch);
        }
    }

    pub fn fill_vertical_line(&mut self, x: i32, top: i32, bottom: i32, ch: Character) {
        let mut y = top;
        while y <= bottom {
            if let Some(pos) = self.coords_to_position(x, y) {
                self.chars[pos].set(&ch);
            }
            y += 1;
        }
    }

    pub fn fill_vertical_line_width_size(&mut self, x: i32, y: i32, height: u32, ch: Character) {
        if height > 0 {
            self.fill_vertical_line(x, y, y + ((height - 1) as i32), ch);
        }
    }

    pub fn draw_rect(
        &mut self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        if (left > right) || (top > bottom) {
            return;
        }
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
        self.set(left, top, ch);
        ch.code = line_chars.corner_top_right;
        self.set(right, top, ch);
        ch.code = line_chars.corner_bottom_right;
        self.set(right, bottom, ch);
        ch.code = line_chars.corner_bottom_left;
        self.set(left, bottom, ch);
    }
}

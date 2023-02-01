use crate::graphics::SpecialChar;

use super::CharAttribute;
//use super::CharFlags;
use super::text_format::TextWrap;
use super::Character;
use super::ClipArea;
use super::Color;
use super::Cursor;
use super::Image;
use super::LineType;
use super::Point;
use super::TextAlignament;
use super::TextFormat;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImageRenderingMethod {
    PixelTo16ColorsSmallBlock,
    PixelTo64ColorsLargeBlock,
    GrayScale,
    AsciiArt,
}
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImageScaleMethod {
    NoScale = 1,
    Scale50 = 2,
    Scale33 = 3,
    Scale25 = 4,
    Scale20 = 5,
    Scale10 = 10,
    Scale5 = 20,
}

const MAX_SURFACE_WIDTH: u32 = 10000;
const MAX_SURFACE_HEIGHT: u32 = 10000;

pub struct Surface {
    pub(crate) width: u32,
    pub(crate) height: u32,
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
    pub fn new(width: u32, height: u32) -> Surface {
        let w = width.clamp(1, MAX_SURFACE_WIDTH);
        let h = height.clamp(1, MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        let mut s = Surface {
            width: w,
            height: h,
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
        let x = x + self.origin.x;
        let y = y + self.origin.y;
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
        self.origin.x = x + self.base_origin.x;
        self.origin.y = y + self.base_origin.y;
    }
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

    #[inline]
    pub fn set_clip(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.clip.set(
            i32::max(self.base_clip.left, left),
            i32::max(self.base_clip.top, top),
            i32::min(self.base_clip.right, right),
            i32::min(self.base_clip.bottom, bottom),
        );
    }
    #[inline]
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
    pub fn set_cursor(&mut self, x: i32, y: i32) {
        let x = x + self.origin.x;
        let y = y + self.origin.y;
        if self.clip.contains(x, y) {
            self.cursor.set(x as u32, y as u32);
        } else {
            self.cursor.hide();
        }
    }
    #[inline]
    pub fn hide_cursor(&mut self) {
        self.cursor.hide();
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, ch: Character) {
        if let Some(pos) = self.coords_to_position(x, y) {
            self.chars[pos].set(ch);
        }
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<&Character> {
        let pos = self.coords_to_position(x, y)?;
        return Some(&(self.chars[pos]));
    }

    pub fn clear(&mut self, ch: Character) {
        if !self.clip.is_visible() {
            return;
        }
        if (self.clip.left == 0)
            && (self.clip.top == 0)
            && (self.clip.right == self.right_most)
            && (self.clip.bottom == self.bottom_most)
        {
            // the entire screen has to be cleared
            for c in &mut self.chars {
                c.set(ch);
            }
        } else {
            // only the clip must pe cleared
            let mut pos = self.clip.left as usize;
            let sz = (self.clip.right + 1 - self.clip.left) as usize;
            pos += (self.clip.top as usize) * (self.width as usize);

            for _ in self.clip.top..=self.clip.bottom {
                for c in &mut self.chars[pos..(pos + sz)] {
                    c.set(ch);
                }
                pos += self.width as usize;
            }
        }
    }

    pub fn fill_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, ch: Character) {
        if (left > right) || (top > bottom) {
            return;
        }
        for x in left..=right {
            for y in top..=bottom {
                if let Some(pos) = self.coords_to_position(x, y) {
                    self.chars[pos].set(ch);
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
                self.chars[pos].set(ch);
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
                self.chars[pos].set(ch);
            }
            y += 1;
        }
    }
    pub fn fill_vertical_line_width_size(&mut self, x: i32, y: i32, height: u32, ch: Character) {
        if height > 0 {
            self.fill_vertical_line(x, y, y + ((height - 1) as i32), ch);
        }
    }

    pub fn draw_vertical_line(
        &mut self,
        x: i32,
        top: i32,
        bottom: i32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        self.fill_vertical_line(
            x,
            top,
            bottom,
            Character::new(
                line_type.get_chars().vertical,
                attr.foreground,
                attr.background,
                attr.flags,
            ),
        );
    }

    pub fn draw_vertical_line_with_size(
        &mut self,
        x: i32,
        y: i32,
        height: u32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        if height > 0 {
            self.fill_vertical_line(
                x,
                y,
                y + ((height - 1) as i32),
                Character::new(
                    line_type.get_chars().vertical,
                    attr.foreground,
                    attr.background,
                    attr.flags,
                ),
            );
        }
    }

    pub fn draw_horizontal_line(
        &mut self,
        left: i32,
        y: i32,
        right: i32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        self.fill_horizontal_line(
            left,
            y,
            right,
            Character::new(
                line_type.get_chars().horizontal,
                attr.foreground,
                attr.background,
                attr.flags,
            ),
        );
    }

    pub fn draw_horizontal_line_with_size(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        if width > 0 {
            self.fill_horizontal_line(
                x,
                y,
                x + ((width - 1) as i32),
                Character::new(
                    line_type.get_chars().horizontal,
                    attr.foreground,
                    attr.background,
                    attr.flags,
                ),
            );
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

    pub fn draw_rect_with_size(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        line_type: LineType,
        attr: CharAttribute,
    ) {
        if (width > 0) && (height > 0) {
            self.draw_rect(
                x,
                y,
                x + ((width - 1) as i32),
                y + ((height - 1) as i32),
                line_type,
                attr,
            );
        }
    }

    pub fn draw_surface(&mut self, x: i32, y: i32, surface: &Surface) {
        if self.clip.is_visible() == false {
            return;
        }
        let mut index = 0usize;
        for s_y in 0..=surface.bottom_most {
            for s_x in 0..=surface.right_most {
                self.set(x + s_x, y + s_y, surface.chars[index]);
                index += 1;
            }
        }
    }

    pub fn write_string(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        attr: CharAttribute,
        multi_line: bool,
    ) {
        let mut c = Character::new(' ', attr.foreground, attr.background, attr.flags);
        if !multi_line {
            // single line support
            if self.clip.contains_y(y + self.origin.y) == false {
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

    fn write_text_single_line(
        &mut self,
        text: &str,
        y: i32,
        chars_count: u16,
        ch_index: usize,
        format: &TextFormat,
    ) {
        if self.clip.contains_y(y) == false {
            return; // no need to draw
        }
        let mut x = match format.align {
            TextAlignament::Left => format.x,
            TextAlignament::Center => format.x - (chars_count / 2) as i32,
            TextAlignament::Right => format.x + 1 - chars_count as i32,
        };
        let width = u16::min(format.width.unwrap_or(chars_count), chars_count);
        let left_margin = match format.align {
            TextAlignament::Left => format.x,
            TextAlignament::Center => format.x - (width / 2) as i32,
            TextAlignament::Right => format.x + 1 - width as i32,
        };
        let right_margin = left_margin + (width as i32);
        let mut c = Character::with_attributes(' ', format.char_attr);

        if format.hotkey_pos.is_some() && format.hotkey_attr.is_some() {
            let hkpos = format.hotkey_pos.unwrap();
            let mut cpos = ch_index;
            for ch in text.chars() {
                if (x >= left_margin) && (x < right_margin) {
                    if let Some(pos) = self.coords_to_position(x, y) {
                        if cpos == hkpos {
                            self.chars[pos]
                                .set(Character::with_attributes(ch, format.hotkey_attr.unwrap()));
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
        let mut y = format.y + self.origin.y;
        let mut start_ofs = 0usize;
        let mut chars_count = 0u16;
        let mut ch_index = 0usize;
        for (index, ch) in text.char_indices() {
            if (ch == '\n') || (ch == '\r') {
                if chars_count > 0 {
                    self.write_text_single_line(
                        &text[start_ofs..index],
                        y,
                        chars_count,
                        ch_index,
                        format,
                    );
                }
                y += 1;
                ch_index += chars_count as usize;
                chars_count = 0;
                start_ofs = index;
            } else {
                chars_count += 1;
            }
        }
    }
    fn write_text_multi_line(&mut self, text: &str, format: &TextFormat) {
        match format.text_wrap {
            TextWrap::None => self.write_text_multi_line_no_wrap(text, format),
            _ => unimplemented!(),
        }
    }
    pub fn write_text(&mut self, text: &str, format: &TextFormat) {
        if format.multi_lines {
            self.write_text_multi_line(text, format);
        } else {
            let chars_count = if format.chars_count.is_some() {
                format.chars_count.unwrap()
            } else {
                text.chars().count() as u16
            };
            self.write_text_single_line(text, format.y + self.origin.y, chars_count, 0, format);
        }
    }

    fn paint_small_blocks(&mut self, img: &Image, x: i32, y: i32, rap: u32) {
        let w = img.get_width();
        let h = img.get_height();
        let x_step = rap;
        let y_step = rap * 2;
        let mut cp = Character::default();
        let mut py = y;
        let mut img_y = 0;
        while img_y < h {
            let mut px = x;
            let mut img_x = 0u32;
            while img_x < w {
                if rap == 1 {
                    cp.foreground = img.get_pixel_or_default(img_x, img_y).to_color();
                    cp.background = img.get_pixel_or_default(img_x, img_y + 1).to_color();
                } else {
                    cp.foreground = img
                        .compute_square_average_color(img_x, img_y, rap)
                        .to_color();
                    cp.background = img
                        .compute_square_average_color(img_x, img_y + rap, rap)
                        .to_color();
                }

                if cp.background == cp.foreground {
                    if cp.background == Color::Black {
                        cp.code = ' ';
                    } else {
                        cp.code = char::from(SpecialChar::Block100);
                    }
                } else {
                    cp.code = char::from(SpecialChar::BlockUpperHalf);
                }
                self.set(px, py, cp);
                img_x += x_step;
                px += 1;
            }
            py += 1;
            img_y += y_step;
        }
    }

    fn paint_large_blocks(&mut self, img: &Image, x: i32, y: i32, rap: u32) {
        let w = img.get_width();
        let h = img.get_height();
        let mut img_y = 0u32;
        let mut p_y = y;
        while img_y < h {
            let mut p_x = x;
            let mut img_x = 0u32;
            while img_x < w {
                if rap == 1 {
                    self.fill_horizontal_line(
                        p_x,
                        p_y,
                        p_x + 1,
                        img.get_pixel_or_default(img_x, img_y).to_character(),
                    );
                } else {
                    self.fill_horizontal_line(
                        p_x,
                        p_y,
                        p_x + 1,
                        img.compute_square_average_color(img_x, img_y, rap)
                            .to_character(),
                    );
                }
                img_x += rap;
                p_x += 2;
            }
            img_y += rap;
            p_y += 1;
        }
    }

    fn paint_gray_scale(&mut self, img: &Image, x: i32, y: i32, rap: u32) {
        let w = img.get_width();
        let h = img.get_height();
        let mut img_y = 0u32;
        let mut p_y = y;
        while img_y < h {
            let mut p_x = x;
            let mut img_x = 0u32;
            while img_x < w {
                if rap == 1 {
                    self.fill_horizontal_line(
                        p_x,
                        p_y,
                        p_x + 1,
                        img.get_pixel_or_default(img_x, img_y).to_gray_scale(),
                    );
                } else {
                    self.fill_horizontal_line(
                        p_x,
                        p_y,
                        p_x + 1,
                        img.compute_square_average_color(img_x, img_y, rap)
                            .to_gray_scale(),
                    );
                }
                img_x += rap;
                p_x += 2;
            }
            img_y += rap;
            p_y += 1;
        }
    }

    pub fn draw_image(
        &mut self,
        x: i32,
        y: i32,
        image: &Image,
        rendering_method: ImageRenderingMethod,
        scale_method: ImageScaleMethod,
    ) {
        let rap = scale_method as u32;
        match rendering_method {
            ImageRenderingMethod::PixelTo16ColorsSmallBlock => {
                self.paint_small_blocks(image, x, y, rap)
            }
            ImageRenderingMethod::PixelTo64ColorsLargeBlock => {
                self.paint_large_blocks(image, x, y, rap)
            }
            ImageRenderingMethod::GrayScale => self.paint_gray_scale(image, x, y, rap),
            _ => {
                todo!()
            }
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        let w = width.clamp(1, MAX_SURFACE_WIDTH);
        let h = height.clamp(1, MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        self.chars.clear();
        self.chars.reserve(count);
        self.chars.resize(count, Character::default());
        self.right_most = (w as i32) - 1;
        self.bottom_most = (h as i32) - 1;
        self.width = w;
        self.height = h;
        self.set_base_clip(0, 0, self.right_most, self.bottom_most);
        self.reset_clip();
        self.set_base_origin(0, 0);
        self.reset_origin();
    }
}

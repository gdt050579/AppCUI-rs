use crate::graphics::SpecialChar;

use super::CharAttribute;
use super::Character;
use super::ClipArea;
use super::Color;
use super::Cursor;
use super::Image;
use super::LineType;
use super::Pixel;

const COLORMAP_16_COLORS: [Color; 27] = [
    /* 0*/ Color::Black, // (0, 0, 0)
    /* 1*/ Color::DarkBlue, // (0, 0, 1)
    /* 2*/ Color::Blue, // (0, 0, 2)
    /* 3*/ Color::DarkGreen, // (0, 1, 0)
    /* 4*/ Color::Teal, // (0, 1, 1)
    /* 5*/ Color::Teal, // (0, 1, 2) [Aprox]
    /* 6*/ Color::Green, // (0, 2, 0)
    /* 7*/ Color::Teal, // (0, 2, 1) [Aprox]
    /* 8*/ Color::Aqua, // (0, 2, 2)
    /* 9*/ Color::DarkRed, // (1, 0, 0)
    /*10*/ Color::Magenta, // (1, 0, 1)
    /*11*/ Color::Magenta, // (1, 0, 2) [Aprox]
    /*12*/ Color::Olive, // (1, 1, 0)
    /*13*/ Color::Gray, // (1, 1, 1)
    /*14*/ Color::Gray, // (1, 1, 2) [Aprox]
    /*15*/ Color::Olive, // (1, 2, 0) [Aprox]
    /*16*/ Color::Gray, // (1, 2, 1) [Aprox]
    /*17*/ Color::Silver, // (1, 2, 2) [Aprox]
    /*18*/ Color::Red, // (2, 0, 0)
    /*19*/ Color::Magenta, // (2, 0, 1) [Aprox]
    /*20*/ Color::Pink, // (2, 0, 2)
    /*21*/ Color::Olive, // (2, 1, 0) [Aprox]
    /*22*/ Color::Gray, // (2, 1, 1) [Aprox]
    /*23*/ Color::Silver, // (2, 1, 2) [Aprox]
    /*24*/ Color::Yellow, // (2, 2, 0)
    /*25*/ Color::Silver, // (2, 2, 1) [Aprox]
    /*26*/ Color::White, // (2, 2, 2)
];

fn rgb_to_color(pixel: Pixel) -> Color {
    let b = if pixel.blue <= 16 {
        0u32
    } else {
        if pixel.blue < 192 {
            1u32
        } else {
            2u32
        }
    };
    let r = if pixel.red <= 16 {
        0u32
    } else {
        if pixel.red < 192 {
            1u32
        } else {
            2u32
        }
    };
    let g = if pixel.green <= 16 {
        0u32
    } else {
        if pixel.green < 192 {
            1u32
        } else {
            2u32
        }
    };
    return COLORMAP_16_COLORS[(r * 9 + g * 3 + b) as usize];
}

#[repr(u32)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum ImageRenderingMethod {
    PixelTo16ColorsSmallBlock,
    PixelTo64ColorsLargeBlock,
    GrayScale,
    AsciiArt
}
#[repr(u32)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum ImageScaleMethod
{
    NoScale = 1,
    Scale50 = 2,
    Scale33 = 3,
    Scale25 = 4,
    Scale20 = 5,
    Scale10 = 10,
    Scale5  = 20
}


const MAX_SURFACE_WIDTH: u32 = 10000;
const MAX_SURFACE_HEIGHT: u32 = 10000;

pub struct Surface {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) translate_x: i32,
    pub(crate) translate_y: i32,
    pub(crate) chars: Vec<Character>,
    pub(crate) cursor: Cursor,
    clip: ClipArea,
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
            translate_x: 0,
            translate_y: 0,
            chars: Vec::<Character>::with_capacity(count),
            clip: ClipArea::new(0, 0, (w - 1) as i32, (h - 1) as i32),
            cursor: Cursor::new(),
            right_most: (w - 1) as i32,
            bottom_most: (h - 1) as i32,
        };
        let c = Character::default();
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
            i32::min(self.right_most, right),
            i32::min(self.bottom_most, bottom),
        );
    }
    #[inline]
    pub fn reset_clip(&mut self) {
        self.clip.set(0, 0, self.right_most, self.bottom_most);
    }

    #[inline]
    pub fn set_cursor(&mut self, x: i32, y: i32) {
        let x = x + self.translate_x;
        let y = y + self.translate_y;
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
            self.chars[pos].set(&ch);
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
                c.set(&ch);
            }
        } else {
            // only the clip must pe cleared
            let mut pos = self.clip.left as usize;
            let sz = (self.clip.right + 1 - self.clip.left) as usize;
            pos += (self.clip.top as usize) * (self.width as usize);

            for _ in self.clip.top..=self.clip.bottom {
                for c in &mut self.chars[pos..(pos + sz)] {
                    c.set(&ch);
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
                    cp.foreground = rgb_to_color(img.get_pixel_or_default(img_x, img_y));
                    cp.background = rgb_to_color(img.get_pixel_or_default(img_x, img_y + 1));
                } else {
                    cp.foreground =
                        rgb_to_color(img.compute_square_average_color(img_x, img_y, rap));
                    cp.background =
                        rgb_to_color(img.compute_square_average_color(img_x, img_y + rap, rap));
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

    pub fn draw_image(&mut self, x: i32, y: i32, image: &Image, rendering_method: ImageRenderingMethod, scale_method: ImageScaleMethod) {
        let rap = scale_method as u32;
        match rendering_method {
            ImageRenderingMethod::PixelTo16ColorsSmallBlock => self.paint_small_blocks(image, x, y, rap),
            _ => { todo!() }
        }
    }
}

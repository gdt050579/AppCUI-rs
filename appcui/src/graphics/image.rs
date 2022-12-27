use super::Color;
use super::Pixel;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Option<Image> {
        if (width < 1) || (height < 1) {
            return None;
        }
        if (width > 0xF000) || (height > 0xF000) {
            return None;
        }
        let sz = (width as usize) * (height as usize);
        let mut img = Image {
            width: width,
            height: height,
            pixels: Vec::with_capacity(sz),
        };
        let empty_pixel = Pixel::default();
        for _ in 0..sz {
            img.pixels.push(empty_pixel);
        }
        Some(img)
    }
    pub fn from_str(image: &str) -> Option<Image> {
        let buf = image.as_bytes();
        let mut w = 0u32;
        let mut h = 0u32;
        let mut temp_w = 0u32;
        let mut add_value = 0u32;
        for b in buf {
            if (*b) == b'|' {
                add_value = 1 - add_value;
                if add_value == 0 {
                    h += 1;
                    if w == 0 {
                        w = temp_w;
                    } else {
                        if temp_w != w {
                            return None;
                        }
                    }
                    temp_w = 0;
                }
            } else {
                temp_w += add_value;
            }
        }
        if (w < 1) || (h < 1) || (w > 0xF000) || (h > 0xF000) {
            return None;
        }
        let sz = (w as usize) * (h as usize);
        let mut img = Image {
            width: w,
            height: h,
            pixels: Vec::with_capacity(sz),
        };
        for b in buf {
            if (*b) == b'|' {
                add_value = 1 - add_value;
            } else if add_value == 1 {
                match *b {
                    b'0' | b' ' | b'.' => img.pixels.push(Pixel::from_color(Color::Black)),
                    b'B' | b'1' => img.pixels.push(Pixel::from_color(Color::DarkBlue)),
                    b'G' | b'2' => img.pixels.push(Pixel::from_color(Color::DarkGreen)),
                    b'T' | b'3' => img.pixels.push(Pixel::from_color(Color::Teal)),
                    b'R' | b'4' => img.pixels.push(Pixel::from_color(Color::DarkRed)),
                    b'M' | b'm' | b'5' => img.pixels.push(Pixel::from_color(Color::Magenta)),
                    b'6' => img.pixels.push(Pixel::from_color(Color::Olive)),
                    b'S' | b'7' => img.pixels.push(Pixel::from_color(Color::Silver)),
                    b's' | b'8' => img.pixels.push(Pixel::from_color(Color::Gray)),
                    b'b' | b'9' => img.pixels.push(Pixel::from_color(Color::Blue)),
                    b'g' => img.pixels.push(Pixel::from_color(Color::Green)),
                    b'A' | b'a' | b't' => img.pixels.push(Pixel::from_color(Color::Aqua)),
                    b'r' => img.pixels.push(Pixel::from_color(Color::Red)),
                    b'P' | b'p' => img.pixels.push(Pixel::from_color(Color::Pink)),
                    b'Y' | b'y' => img.pixels.push(Pixel::from_color(Color::Yellow)),
                    b'W' | b'w' => img.pixels.push(Pixel::from_color(Color::White)),
                    _ => img.pixels.push(Pixel::from_color(Color::Transparent)),
                }
            }
        }
        Some(img)
    }
    pub fn clear(&mut self, pixel: Pixel) {
        for px in &mut self.pixels {
            *px = pixel;
        }
    }
    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if (x < self.width) && (y < self.height) {
            self.pixels[(y as usize) * (self.width as usize) + (x as usize)] = pixel;
        }
    }
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if (x < self.width) && (y < self.height) {
            return Some(self.pixels[(y as usize) * (self.width as usize) + (x as usize)]);
        }
        return None;
    }
    #[inline]
    pub fn get_pixel_or_default(&self, x: u32, y: u32) -> Pixel {
        if (x < self.width) && (y < self.height) {
            return self.pixels[(y as usize) * (self.width as usize) + (x as usize)];
        }
        return Pixel::default();
    }
    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }
    #[inline]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub (super) fn compute_square_average_color(&self, x: u32, y: u32, sz: u32) -> Pixel {
        if (x >= self.width) || (y >= self.height) || (sz == 0) {
            return Pixel::default(); // nothing to compute
        }
        let e_x = (x + sz).clamp(0, self.width);
        let e_y = (y + sz).clamp(0, self.height);
        let actual_width = (e_x - x) as usize;
        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;
        let mut pos = (y as usize) * (self.width as usize) + (x as usize);
        let mut p_y = y;
        while p_y < e_y {
            for px in &self.pixels[pos..(pos + actual_width)] {
                sum_r += px.red as u32;
                sum_g += px.green as u32;
                sum_b += px.blue as u32;
            }
            pos += self.width as usize;
            p_y += 1;
        }
        let nr_pixels = sz * sz;
        return Pixel::from_rgb(
            (sum_r / nr_pixels) as u8,
            (sum_g / nr_pixels) as u8,
            (sum_b / nr_pixels) as u8,
        );
    }
}

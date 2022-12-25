use super::Pixel;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}

impl Image {
    pub fn new(width: u32, height: u32) -> Option<Image> {
        if (width<1) || (height<1) {
            return None;
        }
        if (width>0xF000) || (height>0xF000) {
            return None;
        }
        let sz = (width as usize) * (height as usize);
        let mut img = Image {width: width, height: height, pixels: Vec::with_capacity(sz)};
        let empty_pixel = Pixel::default();
        for _ in 0..sz {
            img.pixels.push(empty_pixel);
        }
        Some(img)
    }
    pub fn clear(&mut self, pixel: Pixel) {
        for px in &mut self.pixels {
            *px = pixel;
        }
    }
    #[inline]
    pub fn set(&mut self, x: u32, y:u32, pixel: Pixel) {
        if (x<self.width) && (y<self.height) {
            self.pixels[(y as usize)*(self.height as usize)+(x as usize)] = pixel;
        }
    }
    #[inline]
    pub fn get(&mut self, x: u32, y: u32)->Option<Pixel> {
        if (x<self.width) && (y<self.height) {
            return Some(self.pixels[(y as usize)*(self.height as usize)+(x as usize)]);
        }
        return None;
    }
    #[inline]
    pub fn get_width(&self) -> u32 { self.width }
    #[inline]
    pub fn get_height(&self) -> u32 { self.height }
}
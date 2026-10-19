use super::super::Size;
enum PixelsVec {
    U8(Vec<u8>),
    U16(Vec<u16>),
}
pub struct PaletteTile {
    pixels: PixelsVec,
    size: Size,
}
impl PaletteTile {
    pub(crate) const TRANSPARENT_U8: u8 = u8::MAX;
    pub(crate) const TRANSPARENT_U16: u16 = u16::MAX;
    pub fn new(width: u32, height: u32) -> Self {
        let w = width.max(1);
        let h = height.max(1);
        let sz = w as usize * h as usize;
        let mut v: Vec<u8> = Vec::with_capacity(sz);
        v.resize(sz, Self::TRANSPARENT_U8);
        Self {
            pixels: PixelsVec::U8(v),
            size: Size::new(w, h),
        }
    }
    pub fn set(&mut self, x: u32, y: u32, palette_index: u16) {
        if x >= self.size.width || y >= self.size.height  {
            return;
        }
        let idx = x as usize + y as usize * self.size.width as usize;
        match &mut self.pixels {
            PixelsVec::U8(pixels) => {
                if palette_index >= 255 {
                    let mut vu16 = Vec::with_capacity(pixels.capacity());
                    for i in pixels {
                        if *i == Self::TRANSPARENT_U8 {
                            vu16.push(Self::TRANSPARENT_U16);
                        } else {
                            vu16.push(*i as u16);
                        }
                    }
                    vu16[idx] = palette_index;
                    self.pixels = PixelsVec::U16(vu16);
                } else {
                    pixels[idx] = palette_index as u8;
                }
            }
            PixelsVec::U16(pixels) => pixels[idx] = palette_index,
        }
    }
    pub fn get(&self, x: u32, y: u32) -> Option<u16> {
        if x >= self.size.width || y >= self.size.height {
            return None;
        }
        let idx = x as usize + y as usize * self.size.width as usize;
        match &self.pixels {
            PixelsVec::U8(pixels) => {
                let v = pixels[idx];
                if v == Self::TRANSPARENT_U8 {
                    None
                } else {
                    Some(v as u16)
                }
            }
            PixelsVec::U16(pixels) => {
                let v = pixels[idx];
                if v == Self::TRANSPARENT_U16 {
                    None
                } else {
                    Some(v)
                }
            }
        }
    }
    pub fn clear(&mut self, x: u32, y: u32) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }
        let idx = x as usize + y as usize * self.size.width as usize;
        match &mut self.pixels {
            PixelsVec::U8(pixels) => pixels[idx] = Self::TRANSPARENT_U8,
            PixelsVec::U16(pixels) => pixels[idx] = Self::TRANSPARENT_U16,
        }
    }
    pub fn reset(&mut self) {
        match &mut self.pixels {
            PixelsVec::U8(pixels) => pixels.fill(Self::TRANSPARENT_U8),
            PixelsVec::U16(pixels) => pixels.fill(Self::TRANSPARENT_U16),
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        let w = width.max(1);
        let h = height.max(1);
        let sz = w as usize * h as usize;
        match &mut self.pixels {
            PixelsVec::U8(pixels) => {
                pixels.resize(sz, Self::TRANSPARENT_U8);
                pixels.fill(Self::TRANSPARENT_U8);
            }
            PixelsVec::U16(pixels) => {
                pixels.resize(sz, Self::TRANSPARENT_U16);
                pixels.fill(Self::TRANSPARENT_U16);
            }
        }
        self.size = Size::new(w, h);
    }
    #[inline(always)]
    pub fn size(&self) -> Size {
        self.size
    }

}
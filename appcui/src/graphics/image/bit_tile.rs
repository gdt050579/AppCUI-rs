use super::super::Size;

#[derive(Copy, Clone)]
pub struct BitTile<const STORAGE_BYTES: usize> {
    width: u8,
    height: u8,
    data: [u8; STORAGE_BYTES],
}

impl<const STORAGE_BYTES: usize> BitTile<STORAGE_BYTES> {
    const _ASSERT_CHECK_STORAGE_BITS_: () = assert!(STORAGE_BYTES >= 2 && STORAGE_BYTES <= 1024);
    pub fn new(width: u8, height: u8) -> Option<Self> {
        if (width == 0) || (height == 0) {
            return None;
        }
        if (width as usize) * (height as usize) > (STORAGE_BYTES << 3) {
            return None;
        }
        Some(Self {
            width,
            height,
            data: [0; STORAGE_BYTES],
        })
    }
    pub fn width(&self) -> u8 {
        self.width
    }
    pub fn height(&self) -> u8 {
        self.height
    }
    pub fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
    #[inline(always)]
    pub fn get(&self, x: u8, y: u8) -> Option<bool> {
        if (x >= self.width) || (y >= self.height) {
            None
        } else {
            let pos = (x as usize) + (y as usize) * (self.width as usize);
            let idx = pos >> 3;
            let mask = 1 << (pos & 7);
            if (self.data[idx] & mask) == 0 {
                Some(false)
            } else {
                Some(true)
            }
        }
    }
    #[inline(always)]
    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        if (x < self.width) && (y < self.height) {
            let pos = (x as usize) + (y as usize) * (self.width as usize);
            let idx = pos >> 3;
            let mask = 1 << (pos & 7);
            if value {
                self.data[idx] |= mask;
            } else {
                self.data[idx] &= !mask;
            }
        }
    }
    pub fn clear(&mut self, value: bool) {
        self.data.fill(if value { u8::MAX } else { 0 });
    }
}

impl BitTile<4> {
    fn from_u32(width: u8, height: u8, bits: u32) -> Option<BitTile<4>> {
        if (width == 0) || (height == 0) {
            return None;
        }
        if (width as usize) * (height as usize) > 32 {
            return None;
        }
        Some(Self {
            width,
            height,
            data: bits.to_ne_bytes(),
        })
    }
}

impl BitTile<8> {
    fn from_u64(width: u8, height: u8, bits: u64) -> Option<BitTile<8>> {
        if (width == 0) || (height == 0) {
            return None;
        }
        if (width as usize) * (height as usize) > 64 {
            return None;
        }
        Some(Self {
            width,
            height,
            data: bits.to_ne_bytes(),
        })
    }
}

impl BitTile<16> {
    fn from_u128(width: u8, height: u8, bits: u128) -> Option<BitTile<16>> {
        if (width == 0) || (height == 0) {
            return None;
        }
        if (width as usize) * (height as usize) > 128 {
            return None;
        }
        Some(Self {
            width,
            height,
            data: bits.to_ne_bytes(),
        })
    }
}

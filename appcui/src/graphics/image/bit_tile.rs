use super::super::Size;
use super::{StringFormatError, StringFormatParser};
use std::str::FromStr;

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

impl<const STORAGE_BYTES: usize> FromStr for BitTile<STORAGE_BYTES> {
    type Err = StringFormatError;

    fn from_str(image: &str) -> Result<Self, Self::Err> {
        let mut f = StringFormatParser::new(image);
        let size = f.size()?;
        if (size.width > 0xFF) || (size.height > 0xFF) {
            return Err(StringFormatError::ImageTooLarge);
        }
        let required_size = ((size.width as usize) * (size.height as usize)) >> 3;
        if required_size > STORAGE_BYTES {
            return Err(StringFormatError::ImageDoesNotFitInAllocatedSpace);
        }
        let mut tile = Self {
            width: size.width as u8,
            height: size.height as u8,
            data: [0; STORAGE_BYTES],
        };
        let mut idx = 0;
        let mut mask = 1;
        while let Some(line) = f.next_line() {
            for b in line {
                if ((*b) != b' ') && ((*b) != b'.') {
                    // not a 0 - put 1
                    tile.data[idx] |= mask;
                }
            }
            if mask < 0x80 {
                mask <<= 1;
            } else {
                mask = 1;
                idx += 1;
            }
        }

        Ok(tile)
    }
}

macro_rules! unsigned_int_implementation {
    ($name: ident, $int:ty, $bytes:expr) => {
        pub type $name = BitTile<$bytes>;
        impl BitTile<$bytes> {
            pub fn from_int(width: u8, height: u8, bits: $int) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                if (width as usize) * (height as usize) > <$int>::BITS as usize {
                    return None;
                }
                Some(Self {
                    width,
                    height,
                    data: bits.to_ne_bytes(),
                })
            }

            pub fn reset(&mut self, bits: $int) {
                self.data = bits.to_ne_bytes();
            }
        }
    };
}

unsigned_int_implementation!(BitTileU16, u16, 2);
unsigned_int_implementation!(BitTileU32, u32, 4);
unsigned_int_implementation!(BitTileU64, u64, 8);
unsigned_int_implementation!(BitTileU128, u128, 16);

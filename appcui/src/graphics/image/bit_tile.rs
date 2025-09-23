use crate::prelude::SpecialChar;

use super::super::{CharFlags, Character, Color, Point, Size, Surface};
use super::{StringFormatError, StringFormatParser};
use std::str::FromStr;

pub enum BitTileRenderMethod {
    SmallBlocks,
    LargeBlocks,
    Braille
}

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
    pub(in super::super) fn paint_large(&self, surface: &mut Surface, pos: Point, set_pixel_color: Color, unset_pixel_color: Color) {
        let mut ch = Character::new(' ', Color::White, Color::Transparent, CharFlags::None);
        for y in 0..self.height {
            let mut x_pos = pos.x;
            for x in 0..self.width {
                if let Some(is_set) = self.get(x, y) {
                    if is_set {
                        if set_pixel_color != Color::Transparent {
                            ch.background = set_pixel_color;
                            surface.write_char(x_pos, pos.y + y as i32, ch);
                            surface.write_char(x_pos + 1, pos.y + y as i32, ch);
                        }
                    } else if unset_pixel_color != Color::Transparent {
                        ch.background = unset_pixel_color;
                        surface.write_char(x_pos, pos.y + y as i32, ch);
                        surface.write_char(x_pos + 1, pos.y + y as i32, ch);
                    }
                }
                x_pos += 2;
            }
        }
    }
    pub(in super::super) fn paint_small(&self, surface: &mut Surface, pos: Point, set_pixel_color: Color, unset_pixel_color: Color) {
        let mut ch = Character::new(' ', set_pixel_color, unset_pixel_color, CharFlags::None);
        let mut y = 0u32;
        let h = self.height as u32;
        let mut y_pos = pos.y;
        while y < h {
            for x in 0..self.width {
                let px1 = self.get(x, y as u8).unwrap_or(false);
                let px2 = self.get(x, (y + 1) as u8).unwrap_or(false);
                ch.code = match (px1, px2) {
                    (true, true) => SpecialChar::Block100.into(),
                    (true, false) => SpecialChar::BlockUpperHalf.into(),
                    (false, true) => SpecialChar::BlockLowerHalf.into(),
                    (false, false) => ' ',
                };
                surface.write_char(pos.x + x as i32, y_pos, ch);
            }
            y += 2;
            y_pos += 1;
        }
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
                if mask < 0x80 {
                    mask <<= 1;
                } else {
                    mask = 1;
                    idx += 1;
                }
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

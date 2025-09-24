use crate::prelude::SpecialChar;

use super::super::{CharFlags, Character, Color, Point, Size, Surface};
use super::{StringFormatError, StringFormatParser};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BitTileRenderMethod {
    SmallBlocks,
    LargeBlocks,
    Braille,
}

/// A bit tile is a 2D array of bits. It is used to store a 2D image in a compact way (where a pixel is represented by a single bit that can be either set or unset)
/// The size of the bit tile is the STORAGE_BYTES generic parameter plus 2 bytes for the width and height.
/// Since the width and height are stored in 2 bytes, the maximum size of the bit tile is 255 x 255 pixels.
/// However, the STORAGE_BYTES parameter is limited to 1024 bytes, so the size of the tile should fit in this space.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BitTile<const STORAGE_BYTES: usize> {
    width: u8,
    height: u8,
    data: [u8; STORAGE_BYTES],
}

impl<const STORAGE_BYTES: usize> BitTile<STORAGE_BYTES> {
    const _ASSERT_CHECK_STORAGE_BITS_: () = assert!(STORAGE_BYTES >= 2 && STORAGE_BYTES <= 1024);

    /// Creates a new bit tile with the specified width and height.
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the bit tile in pixels.
    /// * `height` - The height of the bit tile in pixels.
    /// 
    /// # Returns
    /// 
    /// * `Some(BitTile)` - If the bit tile was created successfully.
    /// * `None` - If the width or height is 0 or if the bit tile does not fit in the allocated space.
    pub fn new(width: u8, height: u8) -> Option<Self> {
        if (width == 0) || (height == 0) {
            return None;
        }
        if (width as usize) * (height as usize) > (STORAGE_BYTES << 3) {
            return None;
        }
        Some(Self {width, height, data: [0; STORAGE_BYTES]})
    }

    /// Returns the width of the bit tile in pixels.
    #[inline(always)]
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Returns the height of the bit tile in pixels.
    #[inline(always)]
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Returns the size of the bit tile in pixels.
    #[inline(always)]
    pub fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }

    /// Returns the value of the pixel (set or unset) at the specified coordinates.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    /// 
    /// # Returns
    /// 
    /// * `Some(bool)` - If the pixel is set.
    /// * `None` - If the coordinates are outside the bounds of the bit tile.
    #[inline(always)]
    pub fn get(&self, x: u32, y: u32) -> Option<bool> {
        if (x >= self.width as u32) || (y >= self.height as u32) {
            None
        } else {
            let pos = (x as usize) + (y as usize) * (self.width as usize);
            Some((self.data[pos >> 3] & (1 << (pos & 7))) != 0)
        }
    }

    /// Sets the value of the pixel (set or unset) at the specified coordinates. If the coordinates are outside the bounds of the bit tile, the operation is silently ignored.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    /// * `value` - The value of the pixel (set or unset).
    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        if (x < self.width as u32) && (y < self.height as u32) {
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

    /// Clears the bit tile by setting all pixels to the specified value.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to set all pixels to (set or unset).
    pub fn clear(&mut self, value: bool) {
        self.data.fill(if value { u8::MAX } else { 0 });
    }
    pub(in super::super) fn paint_large(&self, surface: &mut Surface, pos: Point, set_pixel_color: Color, unset_pixel_color: Color) {
        let mut ch = Character::new(' ', Color::White, Color::Transparent, CharFlags::None);
        for y in 0..self.height {
            let mut x_pos = pos.x;
            for x in 0..self.width {
                if let Some(is_set) = self.get(x as u32, y as u32) {
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
                ch.code = match (self.get(x as u32, y).unwrap_or(false), self.get(x as u32, y + 1).unwrap_or(false)) {
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
    pub(in super::super) fn paint_braille(&self, surface: &mut Surface, pos: Point, set_pixel_color: Color, unset_pixel_color: Color) {
        let mut ch = Character::new(' ', set_pixel_color, unset_pixel_color, CharFlags::None);
        let mut y = 0u32;
        let h = self.height as u32;
        let w = self.width as u32;
        let mut y_pos = pos.y;
        while y < h {
            let mut x = 0u32;
            let mut x_pos = pos.x;
            while x < w {
                let mut code = 0;
                if self.get(x, y).unwrap_or(false) {
                    code |= 1;
                }
                if self.get(x, y + 1).unwrap_or(false) {
                    code |= 2;
                }
                if self.get(x, y + 2).unwrap_or(false) {
                    code |= 4;
                }
                if self.get(x, y + 3).unwrap_or(false) {
                    code |= 64;
                }
                if self.get(x + 1, y).unwrap_or(false) {
                    code |= 8;
                }
                if self.get(x + 1, y + 1).unwrap_or(false) {
                    code |= 16;
                }
                if self.get(x + 1, y + 2).unwrap_or(false) {
                    code |= 32;
                }
                if self.get(x + 1, y + 3).unwrap_or(false) {
                    code |= 128;
                }
                ch.code = unsafe { char::from_u32_unchecked(0x2800 + code) };
                surface.write_char(x_pos, y_pos, ch);
                x += 2;
                x_pos += 1;
            }
            y += 4;
            y_pos += 1;
        }
    }
}

impl<const STORAGE_BYTES: usize> FromStr for BitTile<STORAGE_BYTES> {
    type Err = StringFormatError;

    /// Creates a new bit tile from a string.
    /// The format uses pipes (characters `|`) to delimit rows, and single characters to represent different colored pixels.
    /// Since a tile is basically a black and white image, the characters used to represent the pixels are:
    /// * ` ` (space), `.` (point) - unset pixels
    /// * everything else - set pixels
    /// 
    /// For example, the following string will create a bit tile with a width of 14 and a height of 9:
    /// ```rust
    /// use appcui::prelude::*;
    /// use std::str::FromStr;
    ///     
    /// const HEART_TILE: &str = r#"
    ///     |...rr....rr...|
    ///     |..rrrr..rrrr..|
    ///     |.rrrrrrrrrrrr.|
    ///     |.rrrrrrrrrrrr.|
    ///     |..rrrrrrrrrr..|
    ///     |   rrrrrrrr   |
    ///     |....rrrrrr....|
    ///     |.....rrrr.....|
    ///     |......rr......|
    /// "#;
    /// 
    /// // use 16 bytes to store the bit tile (14 x 9 = 126 pixels)
    /// // a BitTile<16> implies 16 bytes x 8 bits/byte = 128 bits (128 pixels) maximum storage capacity.
    /// // since 14 x 9 = 126 pixels, this is well within the maximum storage capacity.
    /// let bit_tile: BitTile<16> = BitTile::from_str(HEART_TILE).unwrap();
    /// ```
    ///
    /// # Arguments
    /// 
    /// * `image` - The string to create the bit tile from.
    /// 
    /// # Returns
    /// 
    /// * `Ok(BitTile)` - If the bit tile was created successfully.
    /// * `Err(StringFormatError)` - If the string is not a valid bit tile.
    fn from_str(image: &str) -> Result<Self, Self::Err> {
        let mut f = StringFormatParser::new(image);
        let size = f.size()?;
        if (size.width > 0xFF) || (size.height > 0xFF) {
            return Err(StringFormatError::ImageTooLarge);
        }
        if ((size.width as usize) * (size.height as usize)) > STORAGE_BYTES * 8 {
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
    ($name:ident,$int:ty,$bytes:expr,$from_fn:ident,$to_fn:ident) => {
        pub type $name = BitTile<$bytes>;

        impl BitTile<$bytes> {
            pub fn $from_fn(width: u8, height: u8, bits: $int) -> Option<Self> {
                if width == 0 || height == 0 {
                    return None;
                }
                if (width as usize) * (height as usize) > <$int>::BITS as usize {
                    return None;
                }
                Some(Self {width, height, data: bits.to_ne_bytes()})
            }

            pub fn $to_fn(&self) -> $int {
                <$int>::from_ne_bytes(self.data)
            }

            pub fn reset(&mut self, bits: $int) {
                self.data = bits.to_ne_bytes();
            }
        }
    };
}

unsigned_int_implementation!(BitTileU16, u16, 2, from_u16, to_u16);
unsigned_int_implementation!(BitTileU32, u32, 4, from_u32, to_u32);
unsigned_int_implementation!(BitTileU64, u64, 8, from_u64, to_u64);
unsigned_int_implementation!(BitTileU128, u128, 16, from_u128, to_u128);

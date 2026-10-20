//! A 2D grid of palette indexes.
//!
//! [`PaletteTile`] stores one index per cell for an external palette, rather than a
//! color. Cells that have not been written are transparent. Indexes `0` through `254`
//! stay in 8-bit storage. The first index of `255` or higher copies every cell into
//! 16-bit storage, which the tile keeps after [`PaletteTile::reset`] and
//! [`PaletteTile::resize`]. In that mode `u16::MAX` is the transparent sentinel, so
//! it cannot be stored as a visible index.

use crate::graphics::{CharAttribute, Point, Surface};

use super::super::Size;

pub enum PaletteTileRenderMethod {
    SmallBlocks,
    LargeBlocks,
    Braille,
    Custom(char)
}

enum PixelsVec {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

/// A 2D grid of palette indexes.
///
/// Each cell stores an index into an external palette, not a color. Cells that have
/// not been written are transparent. Indexes `0` through `254` are kept in 8 bits.
/// The first time an index of `255` or higher is written, every cell is copied into
/// 16-bit storage, which the tile then keeps even after [`reset`](PaletteTile::reset)
/// or [`resize`](PaletteTile::resize). In that mode `u16::MAX` is the transparent
/// sentinel, so it cannot be stored as a visible index.
pub struct PaletteTile {
    pixels: PixelsVec,
    size: Size,
}
impl PaletteTile {
    /// Transparent cell in 8-bit storage (`255`).
    pub(crate) const TRANSPARENT_U8: u8 = u8::MAX;
    /// Transparent cell in 16-bit storage (`65535`).
    pub(crate) const TRANSPARENT_U16: u16 = u16::MAX;

    /// Creates a tile of the given size, with every cell transparent.
    ///
    /// A width or height of `0` is raised to `1`.
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
    /// Writes a palette index at `(x, y)`.
    ///
    /// Coordinates outside the tile are ignored. Index `0` is a valid color index.
    /// An index of `255` or higher promotes the tile to 16-bit storage; indexes already
    /// stored, and cells that are still transparent, are preserved. Writing `u16::MAX`
    /// after that promotion clears the cell, because that value is the transparent sentinel.
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
    /// Returns the palette index at `(x, y)`.
    ///
    /// Returns `None` when the coordinates are outside the tile or the cell is transparent.
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
    /// Makes the cell at `(x, y)` transparent.
    ///
    /// Coordinates outside the tile are ignored. Other cells are left unchanged.
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
    /// Makes every cell transparent. The size is unchanged.
    pub fn reset(&mut self) {
        match &mut self.pixels {
            PixelsVec::U8(pixels) => pixels.fill(Self::TRANSPARENT_U8),
            PixelsVec::U16(pixels) => pixels.fill(Self::TRANSPARENT_U16),
        }
    }
    /// Changes the tile size and makes every cell transparent.
    ///
    /// A width or height of `0` is raised to `1`. A tile that has already been promoted
    /// to 16-bit storage stays in that mode.
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
    /// Returns the width and height of the tile, in pixels.
    #[inline(always)]
    pub fn size(&self) -> Size {
        self.size
    }


    fn paint_small_u8<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u8], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    fn paint_small_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    
    #[inline(always)]
    pub(in super::super) fn paint_small<P>(&self, surface: &mut Surface, pos: Point, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        match &self.pixels {
            PixelsVec::U8(pixels) => Self::paint_small_u8(surface, pos, self.size, pixels, palette),
            PixelsVec::U16(pixels) => Self::paint_small_u16(surface, pos, self.size, pixels, palette),
        }
    }

    fn paint_braille_u8<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u8], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    fn paint_braille_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    #[inline(always)]
    pub(in super::super) fn paint_braille<P>(&self, surface: &mut Surface, pos: Point, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        match &self.pixels {
            PixelsVec::U8(pixels) => Self::paint_braille_u8(surface, pos, self.size, pixels, palette),
            PixelsVec::U16(pixels) => Self::paint_braille_u16(surface, pos, self.size, pixels, palette),
        }
    }

    fn paint_char_u8<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u8], ch: char, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    fn paint_char_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], ch: char, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        todo!()
    }
    #[inline(always)]
    pub(in super::super) fn paint_char<P>(&self, surface: &mut Surface, pos: Point, ch: char, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        match &self.pixels {
            PixelsVec::U8(pixels) => Self::paint_char_u8(surface, pos, self.size, pixels, ch, palette),
            PixelsVec::U16(pixels) => Self::paint_char_u16(surface, pos, self.size, pixels, ch, palette),
        }
    }
}
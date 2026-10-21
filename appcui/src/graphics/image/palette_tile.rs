//! A 2D grid of palette indexes.
//!
//! [`PaletteTile`] stores one index per cell for an external palette, rather than a
//! color. Cells that have not been written are transparent. Indexes `0` through `254`
//! stay in 8-bit storage. The first index of `255` or higher copies every cell into
//! 16-bit storage, which the tile keeps after [`PaletteTile::reset`] and
//! [`PaletteTile::resize`]. In that mode `u16::MAX` is the transparent sentinel, so
//! it cannot be stored as a visible index.

use crate::graphics::{CharAttribute, CharFlags, Character, Color, Point, SpecialChar, Surface};

use super::super::Size;

/// How a [`PaletteTile`] is drawn onto a [`Surface`].
///
/// Every method draws square pixels (a terminal character is about twice as tall as it
/// is wide), like the methods used for [`Image`](super::Image) and [`BitTile`](super::BitTile):
/// the same tile gives the same picture with every method, only the resolution changes.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PaletteTileRenderMethod {
    /// Half blocks (`▀`, `▄`): 1 x 2 pixels per character. Only the foreground of each
    /// palette entry is used, because the two halves can belong to different entries.
    SmallBlocks,
    /// Full blocks (`█`): every pixel is 2 characters wide.
    LargeBlocks,
    /// Braille dots: 2 x 4 pixels per character. A character takes the palette entry that
    /// covers most of its dots.
    Braille,
    /// The given character: every pixel is 2 characters wide.
    Custom(char),
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
    /// Returns the size, in characters, of the tile when it is drawn with `render_method`.
    ///
    /// - [`SmallBlocks`](PaletteTileRenderMethod::SmallBlocks): `width` x `height / 2` (rounded up)
    /// - [`LargeBlocks`](PaletteTileRenderMethod::LargeBlocks) and [`Custom`](PaletteTileRenderMethod::Custom): `width * 2` x `height`
    /// - [`Braille`](PaletteTileRenderMethod::Braille): `width / 2` x `height / 4` (both rounded up)
    pub fn render_size(&self, render_method: PaletteTileRenderMethod) -> Size {
        let (w, h) = (self.size.width, self.size.height);
        match render_method {
            PaletteTileRenderMethod::SmallBlocks => Size::new(w, h.div_ceil(2)),
            PaletteTileRenderMethod::LargeBlocks | PaletteTileRenderMethod::Custom(_) => Size::new(w * 2, h),
            PaletteTileRenderMethod::Braille => Size::new(w.div_ceil(2), h.div_ceil(4)),
        }
    }
    /// Returns the palette index shown by the character at `(x, y)` when the tile is drawn
    /// with `render_method` (`(0, 0)` is the top-left character of the drawn tile).
    ///
    /// This is the index whose attribute the character is drawn with, so it can be used to
    /// find what is under the mouse:
    /// - [`SmallBlocks`](PaletteTileRenderMethod::SmallBlocks): the top pixel, or the bottom one if the top is transparent
    /// - [`LargeBlocks`](PaletteTileRenderMethod::LargeBlocks) and [`Custom`](PaletteTileRenderMethod::Custom): the pixel (2 characters wide)
    /// - [`Braille`](PaletteTileRenderMethod::Braille): the index that covers most of the dots
    ///
    /// Returns `None` if nothing is drawn there (transparent, or outside the tile).
    pub fn cell_index(&self, x: u32, y: u32, render_method: PaletteTileRenderMethod) -> Option<u16> {
        match &self.pixels {
            PixelsVec::U8(pixels) => cell_index_impl(pixels, self.size, x, y, render_method),
            PixelsVec::U16(pixels) => cell_index_impl(pixels, self.size, x, y, render_method),
        }
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
        paint_small_impl(surface, pos, size, pixels, palette);
    }
    fn paint_small_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        paint_small_impl(surface, pos, size, pixels, palette);
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
        paint_braille_impl(surface, pos, size, pixels, palette);
    }
    fn paint_braille_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        paint_braille_impl(surface, pos, size, pixels, palette);
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
        paint_char_impl(surface, pos, size, pixels, ch, palette);
    }
    fn paint_char_u16<P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[u16], ch: char, palette: P)
    where
        P: Fn(u16) -> CharAttribute,
    {
        paint_char_impl(surface, pos, size, pixels, ch, palette);
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

// a stored pixel: its palette index, or None if it is transparent
trait PaletteIndex: Copy {
    fn index(self) -> Option<u16>;
}
impl PaletteIndex for u8 {
    #[inline(always)]
    fn index(self) -> Option<u16> {
        (self != PaletteTile::TRANSPARENT_U8).then_some(self as u16)
    }
}
impl PaletteIndex for u16 {
    #[inline(always)]
    fn index(self) -> Option<u16> {
        (self != PaletteTile::TRANSPARENT_U16).then_some(self)
    }
}

#[inline(always)]
fn pixel_at<T: PaletteIndex>(pixels: &[T], size: Size, x: u32, y: u32) -> Option<u16> {
    if (x < size.width) && (y < size.height) {
        pixels[x as usize + y as usize * size.width as usize].index()
    } else {
        None
    }
}

// 1 x 2 pixels per character. Only the foreground of each palette entry is used, because the
// two halves of a character can belong to different entries. A transparent half keeps
// whatever is under it.
fn paint_small_impl<T, P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[T], palette: P)
where
    T: PaletteIndex,
    P: Fn(u16) -> CharAttribute,
{
    for cell_y in 0..size.height.div_ceil(2) {
        for x in 0..size.width {
            let top = pixel_at(pixels, size, x, cell_y * 2);
            let bottom = pixel_at(pixels, size, x, cell_y * 2 + 1);
            let ch = match (top, bottom) {
                (Some(t), Some(b)) => Character::new(SpecialChar::BlockUpperHalf, palette(t).foreground, palette(b).foreground, CharFlags::None),
                (Some(t), None) => Character::new(SpecialChar::BlockUpperHalf, palette(t).foreground, Color::Transparent, CharFlags::None),
                (None, Some(b)) => Character::new(SpecialChar::BlockLowerHalf, palette(b).foreground, Color::Transparent, CharFlags::None),
                (None, None) => continue,
            };
            surface.write_char(pos.x + x as i32, pos.y + cell_y as i32, ch);
        }
    }
}

// 2 x 4 pixels per character. A character takes the palette entry that covers most of
// its dots (on a tie, the first one found, column by column), with all its attributes.
fn paint_braille_impl<T, P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[T], palette: P)
where
    T: PaletteIndex,
    P: Fn(u16) -> CharAttribute,
{
    for cell_y in 0..size.height.div_ceil(4) {
        for cell_x in 0..size.width.div_ceil(2) {
            if let Some((pattern, id)) = braille_cell(pixels, size, cell_x, cell_y) {
                let code = char::from_u32(0x2800 + pattern as u32).unwrap_or(' ');
                surface.write_char(pos.x + cell_x as i32, pos.y + cell_y as i32, Character::with_attributes(code, palette(id)));
            }
        }
    }
}

// the dots of a braille character and the palette index that covers most of them (on a tie,
// the first one found, column by column); None if every dot is transparent
fn braille_cell<T: PaletteIndex>(pixels: &[T], size: Size, cell_x: u32, cell_y: u32) -> Option<(u8, u16)> {
    // bit of each dot in a braille character, by (column, row)
    const DOTS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];
    let mut pattern = 0u8;
    let mut ids = [0u16; 8];
    let mut counts = [0u8; 8];
    let mut distinct = 0usize;
    for (col, bits) in DOTS.iter().enumerate() {
        for (row, bit) in bits.iter().enumerate() {
            let Some(id) = pixel_at(pixels, size, cell_x * 2 + col as u32, cell_y * 4 + row as u32) else {
                continue;
            };
            pattern |= bit;
            match ids[..distinct].iter().position(|i| *i == id) {
                Some(p) => counts[p] += 1,
                None => {
                    ids[distinct] = id;
                    counts[distinct] = 1;
                    distinct += 1;
                }
            }
        }
    }
    if pattern == 0 {
        return None;
    }
    let mut best = 0;
    for p in 1..distinct {
        if counts[p] > counts[best] {
            best = p;
        }
    }
    Some((pattern, ids[best]))
}

fn cell_index_impl<T: PaletteIndex>(pixels: &[T], size: Size, x: u32, y: u32, render_method: PaletteTileRenderMethod) -> Option<u16> {
    match render_method {
        PaletteTileRenderMethod::SmallBlocks => pixel_at(pixels, size, x, y * 2).or_else(|| pixel_at(pixels, size, x, y * 2 + 1)),
        PaletteTileRenderMethod::LargeBlocks | PaletteTileRenderMethod::Custom(_) => pixel_at(pixels, size, x / 2, y),
        PaletteTileRenderMethod::Braille => braille_cell(pixels, size, x, y).map(|(_, id)| id),
    }
}

// every pixel is 2 characters wide, drawn with `ch` and all the attributes of its palette entry
fn paint_char_impl<T, P>(surface: &mut Surface, pos: Point, size: Size, pixels: &[T], ch: char, palette: P)
where
    T: PaletteIndex,
    P: Fn(u16) -> CharAttribute,
{
    for y in 0..size.height {
        for x in 0..size.width {
            if let Some(id) = pixel_at(pixels, size, x, y) {
                let c = Character::with_attributes(ch, palette(id));
                let left = pos.x + x as i32 * 2;
                surface.write_char(left, pos.y + y as i32, c);
                surface.write_char(left + 1, pos.y + y as i32, c);
            }
        }
    }
}

use super::StringFormatError;
use super::StringFormatParser;
use crate::prelude::image::character_set::{
    ascii_art_renderer, braille_renderer, dithered_shades_renderer, large_blocks_renderer, sixel_renderer, small_blocks_renderer,
};
use crate::prelude::RenderOptions;

use super::super::{Color, Size, Surface};
use super::pixel::Pixel;
use super::CharacterSet;
use std::str::FromStr;

/// A structure representing a raster image with RGBA pixels.
///
/// Images are stored in memory as a vector of pixels with a specified width and height.
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    /// Creates a new empty image with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the image in pixels
    /// * `height` - Height of the image in pixels
    ///
    /// # Returns
    ///
    /// * `Some(Image)` - If the dimensions are valid
    /// * `None` - If either dimension is 0 or exceeds 0xF000
    pub fn new(width: u32, height: u32) -> Option<Image> {
        if (width < 1) || (height < 1) {
            return None;
        }
        if (width > 0xF000) || (height > 0xF000) {
            return None;
        }
        let sz = (width as usize) * (height as usize);
        let mut img = Image {
            width,
            height,
            pixels: Vec::with_capacity(sz),
        };
        let empty_pixel = Pixel::default();
        for _ in 0..sz {
            img.pixels.push(empty_pixel);
        }
        Some(img)
    }

    /// Creates a new image from a buffer of pixels raw values (the pixels are encoded as u32 value)
    ///
    /// # Arguments
    ///
    /// * `buffer` - a &[[u32]] buffer that contains the pixels of the image. Each pixels is encoded as an ARGB value, meaening:
    ///     - **blue** is encoded as the least significant 8 bits : (value & 0xFF)
    ///     - **green** is encoded as the next 8 bits : ((value >> 8) & 0xFF)
    ///     - **red** is the next 8 bits after green  : ((value >> 16) & 0xFF)
    ///     - **alpha** is the most significant 8 bits (value >> 24)
    /// * `size` - The size of the image
    /// * `store_alpha_channel` - true if the alpha channel shold be stored, or false otherwise. If false the alpha will be consider be be 255 for all pixels (each pixel will be opaque)
    ///
    /// # Remarks
    ///
    /// The size of the image must be the same length as the raw pixels buffer (buffer.len() == size.width * size.height)
    ///
    /// # Returns
    ///
    /// * `Some(Image)` - If the dimensions are valid
    /// * `None` - If either dimension is 0 or exceeds 0xF000
    pub fn from_buffer(buffer: &[u32], size: Size, store_alpha_channel: bool) -> Option<Image> {
        if (size.width < 1) || (size.height < 1) {
            return None;
        }
        if (size.width > 0xF000) || (size.height > 0xF000) {
            return None;
        }
        if ((size.width as usize) * (size.height as usize)) != buffer.len() {
            return None;
        }
        // all good - create
        let mut me = Self {
            width: size.width,
            height: size.height,
            pixels: Vec::with_capacity(buffer.len()),
        };
        for pixel_value in buffer {
            me.pixels.push(Pixel::from(*pixel_value));
        }
        if !store_alpha_channel {
            me.remove_alpha();
        }
        Some(me)
    }

    /// Clears the entire image with the specified pixel color.
    ///
    /// # Arguments
    ///
    /// * `pixel` - The pixel value to fill the image with
    pub fn clear(&mut self, pixel: Pixel) {
        for px in &mut self.pixels {
            *px = pixel;
        }
    }
    /// Sets a pixel at the specified coordinates.
    ///
    /// If the coordinates are outside the image dimensions, the operation is silently ignored.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 is leftmost)
    /// * `y` - Y coordinate (0 is topmost)
    /// * `pixel` - The pixel value to set
    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if (x < self.width) && (y < self.height) {
            self.pixels[(y as usize) * (self.width as usize) + (x as usize)] = pixel;
        }
    }
    /// Gets the pixel at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 is leftmost)
    /// * `y` - Y coordinate (0 is topmost)
    ///
    /// # Returns
    ///
    /// * `Some(Pixel)` - If the coordinates are within the image dimensions
    /// * `None` - If the coordinates are outside the image
    #[inline]
    pub fn pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if (x < self.width) && (y < self.height) {
            return Some(self.pixels[(y as usize) * (self.width as usize) + (x as usize)]);
        }
        None
    }
    /// Returns the width of the image in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Returns the height of the image in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Returns the size (width and height) of the image as a Size object.
    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
    /// Calculates the rendered size of the image based on the rendering method and scaling.
    ///
    /// # Arguments
    ///
    /// * `render_options` - The RenderOptions struct that would be used to draw theimage
    ///
    /// # Returns
    ///
    /// The resulting Size object after applying the rendering and scaling methods
    #[inline]
    pub fn render_size(&self, render_options: &RenderOptions) -> Size {
        let unscale_size = match render_options.char_set {
            CharacterSet::SmallBlocks => small_blocks_renderer::size(self),
            CharacterSet::LargeBlocks => large_blocks_renderer::size(self),
            CharacterSet::DitheredShades => dithered_shades_renderer::size(self),
            CharacterSet::Braille => braille_renderer::size(self),
            CharacterSet::AsciiArt => ascii_art_renderer::size(self),
            CharacterSet::Sixel => sixel_renderer::size(self),
        };
        let rap = render_options.scale as u32;
        if rap == 1 {
            unscale_size
        } else {
            Size::new(unscale_size.width.div_ceil(rap), unscale_size.height.div_ceil(rap))
        }
    }

    #[inline(always)]
    pub(crate) fn paint(&self, surface: &mut Surface, x: i32, y: i32, render_options: &RenderOptions) {
        match render_options.char_set {
            CharacterSet::SmallBlocks => small_blocks_renderer::paint(surface, self, x, y, render_options),
            CharacterSet::LargeBlocks => large_blocks_renderer::paint(surface, self, x, y, render_options),
            CharacterSet::DitheredShades => dithered_shades_renderer::paint(surface, self, x, y, render_options),
            CharacterSet::Braille => braille_renderer::paint(surface, self, x, y, render_options),
            CharacterSet::AsciiArt => ascii_art_renderer::paint(surface, self, x, y, render_options),
            CharacterSet::Sixel => sixel_renderer::paint(surface, self, x, y, render_options),
        }
    }

    pub fn to_sixel(&self, render_options: &RenderOptions) -> String {
        sixel_renderer::generate_sixel(self, render_options)
    }

    /// Removes the alpha channel from all pixels
    pub fn remove_alpha(&mut self) {
        for p in &mut self.pixels {
            p.alpha = u8::MAX;
        }
    }

    pub(super) fn compute_square_average_color(&self, x: u32, y: u32, sz: u32) -> Pixel {
        if (x >= self.width) || (y >= self.height) || (sz == 0) {
            return Pixel::default(); // nothing to compute
        }
        let e_x = (x + sz).clamp(0, self.width);
        let e_y = (y + sz).clamp(0, self.height);
        let actual_width = (e_x - x) as usize;
        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;
        let mut sum_a = 0u32;
        let mut pos = (y as usize) * (self.width as usize) + (x as usize);
        let mut p_y = y;
        while p_y < e_y {
            for px in &self.pixels[pos..(pos + actual_width)] {
                sum_r += px.red as u32;
                sum_g += px.green as u32;
                sum_b += px.blue as u32;
                sum_a += px.alpha as u32;
            }
            pos += self.width as usize;
            p_y += 1;
        }
        let nr_pixels = sz * sz;
        Pixel::new(
            (sum_r / nr_pixels) as u8,
            (sum_g / nr_pixels) as u8,
            (sum_b / nr_pixels) as u8,
            (sum_a / nr_pixels) as u8,
        )
    }
}

/// Creates an image from a string representation.
///
/// The string format uses pipe characters `|` to delimit rows, and single characters
/// to represent different colored pixels.
///
/// # Color Codes
///
/// | Color       | Character Codes                |
/// |-------------|--------------------------------|
/// | Black       | '0', ' ' (space), '.'         |
/// | Dark Blue   | 'B', '1'                      |
/// | Dark Green  | 'G', '2'                      |
/// | Teal        | 'T', '3'                      |
/// | Dark Red    | 'R', '4'                      |
/// | Magenta     | 'M', 'm', '5'                 |
/// | Olive       | '6', 'o', 'O'                 |
/// | Silver      | 'S', '7'                      |
/// | Gray        | 's', '8'                      |
/// | Blue        | 'b', '9'                      |
/// | Green       | 'g'                           |
/// | Aqua        | 'A', 'a', 't'                 |
/// | Red         | 'r'                           |
/// | Pink        | 'P', 'p'                      |
/// | Yellow      | 'Y', 'y'                      |
/// | White       | 'W', 'w'                      |
///
/// # Example
///
/// ```rust
/// use appcui::prelude::*;
/// use std::str::FromStr;
///
/// // Create a 3x2 image with specific colors
/// let image_str = "|RGB| |YWr|";
/// let img = Image::from_str(image_str).unwrap();
/// assert_eq!(img.width(), 3);
/// assert_eq!(img.height(), 2);
/// ```
///
/// # Arguments
///
/// * `image` - String representation of the image
///
/// # Returns
///
/// * `Some(Image)` - If the string represents a valid image
/// * `None` - If the format is invalid
impl FromStr for Image {
    type Err = StringFormatError;

    fn from_str(image: &str) -> Result<Self, Self::Err> {
        let mut f = StringFormatParser::new(image);
        let size = f.size()?;
        if (size.width > 0xF000) || (size.height > 0xF000) {
            return Err(StringFormatError::ImageTooLarge);
        }
        let sz = (size.width as usize) * (size.height as usize);
        let mut img = Image {
            width: size.width,
            height: size.height,
            pixels: Vec::with_capacity(sz),
        };
        while let Some(line) = f.next_line() {
            for b in line {
                match *b {
                    b'0' | b' ' | b'.' => img.pixels.push(Pixel::with_color(Color::Black)),
                    b'B' | b'1' => img.pixels.push(Pixel::with_color(Color::DarkBlue)),
                    b'G' | b'2' => img.pixels.push(Pixel::with_color(Color::DarkGreen)),
                    b'T' | b'3' => img.pixels.push(Pixel::with_color(Color::Teal)),
                    b'R' | b'4' => img.pixels.push(Pixel::with_color(Color::DarkRed)),
                    b'M' | b'm' | b'5' => img.pixels.push(Pixel::with_color(Color::Magenta)),
                    b'6' | b'o' | b'O' => img.pixels.push(Pixel::with_color(Color::Olive)),
                    b'S' | b'7' => img.pixels.push(Pixel::with_color(Color::Silver)),
                    b's' | b'8' => img.pixels.push(Pixel::with_color(Color::Gray)),
                    b'b' | b'9' => img.pixels.push(Pixel::with_color(Color::Blue)),
                    b'g' => img.pixels.push(Pixel::with_color(Color::Green)),
                    b'A' | b'a' | b't' => img.pixels.push(Pixel::with_color(Color::Aqua)),
                    b'r' => img.pixels.push(Pixel::with_color(Color::Red)),
                    b'P' | b'p' => img.pixels.push(Pixel::with_color(Color::Pink)),
                    b'Y' | b'y' => img.pixels.push(Pixel::with_color(Color::Yellow)),
                    b'W' | b'w' => img.pixels.push(Pixel::with_color(Color::White)),
                    _ => img.pixels.push(Pixel::with_color(Color::Transparent)),
                }
            }
        }
        Ok(img)
    }
}

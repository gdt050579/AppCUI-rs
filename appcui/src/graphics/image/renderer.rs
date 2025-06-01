use super::{
    super::{Character, Color, Image, SpecialChar, Surface},
    Pixel,
};

/// The type of renderer to use for rendering the image.
///
/// This enum defines the different rendering methods available for images.
/// Each variant represents a specific rendering style:
///
/// - `SmallBlocks`: Renders the image using small blocks.
/// - `LargeBlocks64Colors`: Renders the image using large blocks with 64 colors.
/// - `GrayScale`: Renders the image using a grayscale palette.
/// - `AsciiArt`: Renders the image using ASCII characters.
///
/// The `RendererType` enum is used to select the rendering method for an image.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RendererType {
    SmallBlocks,
    LargeBlocks64Colors,
    GrayScale,
    AsciiArt,
}
pub struct Renderer {}
impl Renderer {
    pub(crate) fn render_with_small_blocks(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32) {
        let w = img.width();
        let h = img.height();
        let x_step = rap;
        let y_step = rap * 2;
        let mut cp = Character::default();
        let mut py = y;
        let mut img_y = 0;
        while img_y < h {
            let mut px = x;
            let mut img_x = 0u32;
            while img_x < w {
                if rap == 1 {
                    cp.foreground = img.pixel(img_x, img_y).unwrap_or_default().as_color();
                    cp.background = img.pixel(img_x, img_y + 1).unwrap_or_default().as_color();
                } else {
                    cp.foreground = img.compute_square_average_color(img_x, img_y, rap).as_color();
                    cp.background = img.compute_square_average_color(img_x, img_y + rap, rap).as_color();
                }

                if cp.background == cp.foreground {
                    if cp.background == Color::Black {
                        cp.code = ' ';
                    } else {
                        cp.code = char::from(SpecialChar::Block100);
                    }
                } else {
                    cp.code = char::from(SpecialChar::BlockUpperHalf);
                }
                surface.write_char(px, py, cp);
                img_x += x_step;
                px += 1;
            }
            py += 1;
            img_y += y_step;
        }
    }

    fn render_with_large_block(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: fn(p: Pixel) -> Character) {
        let w = img.width();
        let h = img.height();
        let mut img_y = 0u32;
        let mut p_y = y;
        while img_y < h {
            let mut p_x = x;
            let mut img_x = 0u32;
            while img_x < w {
                if rap == 1 {
                    surface.fill_horizontal_line(p_x, p_y, p_x + 1, f(img.pixel(img_x, img_y).unwrap_or_default()));
                } else {
                    surface.fill_horizontal_line(p_x, p_y, p_x + 1, f(img.compute_square_average_color(img_x, img_y, rap)));
                }
                img_x += rap;
                p_x += 2;
            }
            img_y += rap;
            p_y += 1;
        }
    }

    pub(crate) fn render_with_large_blocks_64(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32) {
        Renderer::render_with_large_block(surface, img, x, y, rap, |p| p.as_character());
    }
    pub(crate) fn render_with_gray_scale(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32) {
        Renderer::render_with_large_block(surface, img, x, y, rap, |p| p.as_gray_scale_character());
    }
    pub(crate) fn render_ascii_art(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32) {
        Renderer::render_with_large_block(surface, img, x, y, rap, |p| p.as_ascii_art());
    }
}

use crate::backend::utils::SixelEncoder;
use crate::graphics::{CellSize, Image, Size, Surface};
use crate::prelude::RenderOptions;

pub fn size(image: &Image) -> Size {
    size_with_cell(image, CellSize::default())
}

pub fn size_with_cell(image: &Image, cell_size: CellSize) -> Size {
    let cell_width = cell_size.width as u32;
    let cell_height = cell_size.height as u32;

    // Calculate how many characters are needed to display the image
    let char_width = image.width().div_ceil(cell_width);
    let char_height = image.height().div_ceil(cell_height);
    Size::new(char_width.max(1), char_height.max(1))
}

pub fn paint(surface: &mut Surface, image: &Image, x: i32, y: i32, render_options: &RenderOptions) {
    let cell_size = surface.cell_size();
    let cell_width_px = cell_size.width as u32;
    let cell_height_px = cell_size.height as u32;

    let scale = render_options.scale() as u32;
    let scaled_width = if scale > 1 { image.width() / scale } else { image.width() };
    let scaled_height = if scale > 1 { image.height() / scale } else { image.height() };

    // Calculate character dimensions based on actual cell size
    let char_width = scaled_width.div_ceil(cell_width_px) as i32;
    let char_height = scaled_height.div_ceil(cell_height_px) as i32;

    // Fill the character area with placeholder spaces
    use crate::graphics::{CharAttribute, Character, Color};
    let placeholder = Character::with_attributes(' ', CharAttribute::with_color(Color::Transparent, Color::Transparent));

    for cy in 0..char_height {
        for cx in 0..char_width {
            surface.write_char(x + cx, y + cy, placeholder);
        }
    }

    surface.set_sixel_region(x, y, char_width, char_height, image, render_options);
}

pub fn generate_sixel(image: &Image, render_options: &RenderOptions) -> String {
    let mut encoder = SixelEncoder::new();

    let scale = render_options.scale() as u32;

    if scale <= 1 {
        let pixels = image_to_pixels(image);
        encoder.encode(&pixels, image.width(), image.height());
    } else {
        // Scale down the image
        let new_width = image.width().div_ceil(scale);
        let new_height = image.height().div_ceil(scale);
        let pixels = image_to_scaled_pixels(image, new_width, new_height, scale);
        encoder.encode(&pixels, new_width, new_height);
    }

    encoder.into_output()
}

/// Convert Image pixels to (r, g, b, a) tuples
fn image_to_pixels(image: &Image) -> Vec<(u8, u8, u8, u8)> {
    let mut pixels = Vec::with_capacity((image.width() * image.height()) as usize);

    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some(pixel) = image.pixel(x, y) {
                pixels.push((pixel.red, pixel.green, pixel.blue, pixel.alpha));
            } else {
                pixels.push((0, 0, 0, 0));
            }
        }
    }

    pixels
}

/// Convert Image pixels to scaled (r, g, b, a) tuples using area averaging
fn image_to_scaled_pixels(image: &Image, new_width: u32, new_height: u32, scale: u32) -> Vec<(u8, u8, u8, u8)> {
    let mut pixels = Vec::with_capacity((new_width * new_height) as usize);

    for y in 0..new_height {
        for x in 0..new_width {
            // Average pixels in the scale x scale block
            let src_x = x * scale;
            let src_y = y * scale;

            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            for dy in 0..scale {
                for dx in 0..scale {
                    let px = src_x + dx;
                    let py = src_y + dy;
                    if px < image.width() && py < image.height() {
                        if let Some(pixel) = image.pixel(px, py) {
                            r_sum += pixel.red as u32;
                            g_sum += pixel.green as u32;
                            b_sum += pixel.blue as u32;
                            a_sum += pixel.alpha as u32;
                            count += 1;
                        }
                    }
                }
            }

            if count > 0 {
                pixels.push(((r_sum / count) as u8, (g_sum / count) as u8, (b_sum / count) as u8, (a_sum / count) as u8));
            } else {
                pixels.push((0, 0, 0, 0));
            }
        }
    }

    pixels
}

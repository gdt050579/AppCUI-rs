use crate::graphics::*;

fn render(surface: &mut Surface, img: &Image, x: i32, y: i32, lmin: u8, rap: u32, f: fn(p: Pixel) -> Color) {
    let w = img.width();
    let h = img.height();
    let mut img_y = 0u32;
    let mut p_y = y;

    while img_y < h {
        let mut p_x = x;
        let mut img_x = 0u32;

        while img_x < w {
            // Each braille character represents 2x4 pixels
            let mut pattern = 0u8;
            let mut total_r = 0u32;
            let mut total_g = 0u32;
            let mut total_b = 0u32;
            let mut total_a = 0u32;
            let mut pixel_count = 0u32;

            // Sample 8 pixels in a 2x4 grid pattern
            for (i, (dx, dy)) in [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2), (0, 3), (1, 3)].iter().enumerate() {
                let sample_x = img_x + dx * rap;
                let sample_y = img_y + dy * rap;

                if sample_x < w && sample_y < h {
                    let pixel = if rap == 1 {
                        img.pixel(sample_x, sample_y).unwrap_or_default()
                    } else {
                        img.compute_square_average_color(sample_x, sample_y, rap)
                    };

                    // Accumulate RGBA values for averaging
                    total_r += pixel.red as u32;
                    total_g += pixel.green as u32;
                    total_b += pixel.blue as u32;
                    total_a += pixel.alpha as u32;
                    pixel_count += 1;

                    // Determine if this dot should be visible based on luminance
                    let luminance = pixel.luminance();
                    if luminance > lmin {
                        pattern |= 1 << i;
                    }
                }
            }

            // Create braille character
            let braille_char = char::from_u32(0x2800 + pattern as u32).unwrap_or('\u{2800}');

            // Compute average color and apply color function
            let foreground = if pixel_count > 0 {
                let avg_r = (total_r / pixel_count) as u8;
                let avg_g = (total_g / pixel_count) as u8;
                let avg_b = (total_b / pixel_count) as u8;
                let avg_a = (total_a / pixel_count) as u8;
                let avg_pixel = Pixel::new(avg_r, avg_g, avg_b, avg_a);
                f(avg_pixel)
            } else {
                Color::Black
            };

            let ch = Character::new(braille_char, foreground, Color::Black, CharFlags::None);
            surface.write_char(p_x, p_y, ch);

            img_x += 2 * rap; // Move 2 pixels horizontally
            p_x += 1;
        }

        img_y += 4 * rap; // Move 4 pixels vertically
        p_y += 1;
    }
}

#[inline(always)]
pub(crate) fn size(img: &Image) -> Size {
    Size::new((img.width() + 1) >> 1, (img.height() + 3) >> 2)
}

#[inline(always)]
pub(crate) fn paint(surface: &mut Surface, img: &Image, x: i32, y: i32, render_options: &RenderOptions) {
    let rap = render_options.scale as u32;
    let lmin = render_options.luminance_threshold;
    match render_options.color_schema {
        ColorSchema::Auto => {
            #[cfg(feature = "TRUE_COLORS")]
            {
                render(surface, img, x, y, lmin, rap, |p| p.as_rgb_color())
            }
            #[cfg(not(feature = "TRUE_COLORS"))]
            {
                render(surface, img, x, y, rap, |p| p.as_color16())
            }
        }
        ColorSchema::Color16 => render(surface, img, x, y, lmin, rap, |p| p.as_color16()),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::TrueColors => render(surface, img, x, y, lmin, rap, |p| p.as_rgb_color()),
        ColorSchema::GrayScale4 => render(surface, img, x, y, lmin, rap, |p| p.as_grayscale4()),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::GrayScaleTrueColors => render(surface, img, x, y, lmin, rap, |p| p.as_grayscale()),
        ColorSchema::BlackAndWhite => render(surface, img, x, y, lmin, rap, |_| Color::White),
    }
}

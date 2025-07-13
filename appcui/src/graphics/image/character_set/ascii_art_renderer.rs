use crate::graphics::*;

const ASCII_ART_CHARSET: &[char] = &[
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|',
    '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k',
    'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];

fn render<T>(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: T)
where
    T: Fn(Pixel) -> Color,
{
    let w = img.width();
    let h = img.height();
    let mut img_y = 0u32;
    let mut p_y = y;

    while img_y < h {
        let mut p_x = x;
        let mut img_x = 0u32;

        while img_x < w {
            let pixel = if rap == 1 {
                img.pixel(img_x, img_y).unwrap_or_default()
            } else {
                img.compute_square_average_color(img_x, img_y, rap)
            };
            let ascii_char = ASCII_ART_CHARSET[(pixel.luminance() as usize * (ASCII_ART_CHARSET.len() - 1)) / 255];
            let ch = Character::new(ascii_char, f(pixel), Color::Black, CharFlags::None);
            surface.write_char(p_x, p_y, ch);
            surface.write_char(p_x + 1, p_y, ch);

            img_x += rap;
            p_x += 2;
        }

        img_y += rap;
        p_y += 1;
    }
}

#[inline(always)]
pub(crate) fn size(img: &Image) -> Size {
    Size::new(img.width() * 2, img.height())
}

#[inline(always)]
pub(crate) fn paint(surface: &mut Surface, img: &Image, x: i32, y: i32, render_options: &RenderOptions) {
    let rap = render_options.scale as u32;
    match render_options.color_schema {
        ColorSchema::Auto => {
            #[cfg(feature = "TRUE_COLORS")]
            {
                render(surface, img, x, y, rap, |p| p.as_rgb_color())
            }
            #[cfg(not(feature = "TRUE_COLORS"))]
            {
                render(surface, img, x, y, rap, |p| p.as_color16())
            }
        }
        ColorSchema::Color16 => render(surface, img, x, y, rap, |p| p.as_color16()),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::TrueColors => render(surface, img, x, y, rap, |p| p.as_rgb_color()),
        ColorSchema::GrayScale4 => render(surface, img, x, y, rap, |p| p.as_grayscale4()),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::GrayScaleTrueColors => render(surface, img, x, y, rap, |p| p.as_grayscale()),
        ColorSchema::BlackAndWhite => render(surface, img, x, y, rap, |p| p.as_blackwhite(render_options.luminance_threshold)),
    }
}

use crate::graphics::*;

fn render(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: fn(p: Pixel) -> Color) {
    let w = img.width();
    let h = img.height();
    let mut img_y = 0u32;
    let mut p_y = y;
    let mut ch = Character::new(' ', Color::Black, Color::Black, CharFlags::None);
    while img_y < h {
        let mut p_x = x;
        let mut img_x = 0u32;
        while img_x < w {
            ch.background = if rap == 1 {
                f(img.pixel(img_x, img_y).unwrap_or_default())
            } else {
                f(img.compute_square_average_color(img_x, img_y, rap))
            };
            surface.fill_horizontal_line(p_x, p_y, p_x + 1, ch);
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
        ColorSchema::BlackAndWhite => render(surface, img, x, y, rap, |p| p.as_blackwhite()),
    }
}

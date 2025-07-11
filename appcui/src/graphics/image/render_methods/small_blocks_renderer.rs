use crate::graphics::*;

fn render(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: fn(p: Pixel) -> Color) {
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
                cp.foreground = f(img.pixel(img_x, img_y).unwrap_or_default());
                cp.background = f(img.pixel(img_x, img_y + 1).unwrap_or_default());
                // cp.foreground = img.pixel(img_x, img_y).unwrap_or_default().as_color();
                // cp.background = img.pixel(img_x, img_y + 1).unwrap_or_default().as_color();
            } else {
                // cp.foreground = img.compute_square_average_color(img_x, img_y, rap).as_color();
                // cp.background = img.compute_square_average_color(img_x, img_y + rap, rap).as_color();
                cp.foreground = f(img.compute_square_average_color(img_x, img_y, rap));
                cp.background = f(img.compute_square_average_color(img_x, img_y + rap, rap));
            }

            cp.code = if cp.background == cp.foreground {
                ' '
            } else {
                char::from(SpecialChar::BlockUpperHalf)
            };
            surface.write_char(px, py, cp);
            img_x += x_step;
            px += 1;
        }
        py += 1;
        img_y += y_step;
    }
}

#[inline(always)]
pub(crate) fn size(img: &Image) -> Size {
    Size::new(img.width(), img.height() / 2)
}

#[inline(always)]
pub(crate) fn paint(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, color_schema: ColorSchema) {
    match color_schema {
        ColorSchema::Auto => todo!(),
        ColorSchema::Color16 => render(surface, img, x, y, rap, |p| p.as_color()),
        ColorSchema::TrueColors => todo!(),
        ColorSchema::GrayScale => todo!(),
        ColorSchema::BlackAndWhite => todo!(),
    }
}

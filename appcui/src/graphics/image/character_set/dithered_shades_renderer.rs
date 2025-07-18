use crate::graphics::*;

const COLORMAP_64_COLORS: [Color; 125] = [
    Color::Black,
    Color::Blue,
    Color::Blue,
    Color::Blue,
    Color::Blue,
    Color::Green,
    Color::Aqua,
    Color::Blue,
    Color::Blue,
    Color::Blue,
    Color::Green,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Green,
    Color::Green,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Green,
    Color::Green,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Red,
    Color::Pink,
    Color::Blue,
    Color::Blue,
    Color::Blue,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::Blue,
    Color::Blue,
    Color::Green,
    Color::White,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Green,
    Color::Green,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Green,
    Color::Green,
    Color::Aqua,
    Color::Aqua,
    Color::Aqua,
    Color::Red,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Yellow,
    Color::White,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::Aqua,
    Color::Red,
    Color::Red,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Red,
    Color::Red,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Red,
    Color::Red,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Red,
    Color::Red,
    Color::Pink,
    Color::Pink,
    Color::Pink,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::Pink,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
    Color::White,
    Color::Yellow,
    Color::Yellow,
    Color::Yellow,
    Color::White,
    Color::White,
];

const COLORMAP_64_COLORS_PROC: [u8; 125] = [
    0, 25, 50, 75, 100, 25, 25, 50, 75, 100, 50, 25, 50, 50, 75, 75, 75, 50, 75, 75, 100, 100, 75, 75, 100, 25, 25, 50, 75, 100, 25, 25, 25, 75, 100,
    50, 25, 50, 50, 75, 75, 75, 50, 75, 75, 100, 100, 75, 75, 100, 50, 25, 50, 50, 75, 25, 25, 50, 50, 75, 50, 50, 50, 50, 50, 50, 50, 50, 75, 75,
    75, 75, 50, 75, 100, 75, 75, 50, 75, 75, 75, 75, 50, 75, 75, 50, 50, 50, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 100, 100, 100, 75, 75, 100,
    100, 100, 75, 75, 100, 75, 75, 50, 75, 100, 75, 75, 75, 75, 100, 100, 100, 100, 100, 100,
];

const GRAY_CHARS: [(char, Color); 11] = [
    (' ', Color::Black),
    ('░', Color::Gray),
    ('▒', Color::Gray),
    ('░', Color::Silver),
    ('░', Color::White),
    ('▒', Color::Silver),
    ('▓', Color::Gray),
    ('▒', Color::White),
    ('▓', Color::Silver),
    ('▓', Color::White),
    ('█', Color::White),
];

fn pixel_to_16color_character(p: Pixel) -> Character {
    let (p_r, p_g, p_b) = p.blend_alpha();
    let r = ((p_r as u32) + 32) / 64;
    let g = ((p_g as u32) + 32) / 64;
    let b = ((p_b as u32) + 32) / 64;
    let idx = (r * 25 + g * 5 + b) as usize;
    let col = COLORMAP_64_COLORS[idx];
    let proc = COLORMAP_64_COLORS_PROC[idx];
    match proc {
        0 => Character::new(' ', Color::Black, Color::Black, CharFlags::None),
        25 => Character::new(SpecialChar::Block25, col, Color::Black, CharFlags::None),
        50 => Character::new(SpecialChar::Block50, col, Color::Black, CharFlags::None),
        75 => Character::new(SpecialChar::Block75, col, Color::Black, CharFlags::None),
        100 => Character::new(' ', col, col, CharFlags::None),
        _ => Character::default(),
    }
}

fn pixel_to_black_and_white(p: Pixel) -> Character {
    let proc = ((p.luminance() as u32) + 32) >> 6;
    match proc {
        0 => Character::new(' ', Color::Black, Color::Black, CharFlags::None),
        1 => Character::new(SpecialChar::Block25, Color::White, Color::Black, CharFlags::None),
        2 => Character::new(SpecialChar::Block50, Color::White, Color::Black, CharFlags::None),
        3 => Character::new(SpecialChar::Block75, Color::White, Color::Black, CharFlags::None),
        _ => Character::new(' ', Color::White, Color::White, CharFlags::None),
    }
}
fn pixel_to_gray(p: Pixel) -> Character {
    let index = ((p.luminance() as usize) / 24).min(GRAY_CHARS.len() - 1);
    let (ch,fore) = GRAY_CHARS[index];
    Character::new(ch, fore, Color::Black, CharFlags::None)
}

#[cfg(feature = "TRUE_COLORS")]
fn render_large_blocks<T>(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: T)
where
    T: Fn(Pixel) -> Color,
{
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

fn render_dithered<T>(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: T)
where
    T: Fn(Pixel) -> Character,
{
    let w = img.width();
    let h = img.height();
    let mut img_y = 0u32;
    let mut p_y = y;

    while img_y < h {
        let mut p_x = x;
        let mut img_x = 0u32;

        while img_x < w {
            let ch = if rap == 1 {
                f(img.pixel(img_x, img_y).unwrap_or_default())
            } else {
                f(img.compute_square_average_color(img_x, img_y, rap))
            };
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
                render_large_blocks(surface, img, x, y, rap, |p| p.as_rgb_color())
            }
            #[cfg(not(feature = "TRUE_COLORS"))]
            {
                render_dithered(surface, img, x, y, rap, pixel_to_16color_character)
            }
        }
        ColorSchema::Color16 => render_dithered(surface, img, x, y, rap, pixel_to_16color_character),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::TrueColors => render_large_blocks(surface, img, x, y, rap, |p| p.as_rgb_color()),
        ColorSchema::GrayScale4 => render_dithered(surface, img, x, y, rap, pixel_to_gray),
        #[cfg(feature = "TRUE_COLORS")]
        ColorSchema::GrayScaleTrueColors => render_large_blocks(surface, img, x, y, rap, |p| p.as_grayscale()),
        ColorSchema::BlackAndWhite => render_dithered(surface, img, x, y, rap, pixel_to_black_and_white),
    }
}

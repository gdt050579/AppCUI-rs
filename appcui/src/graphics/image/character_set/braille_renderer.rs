use crate::graphics::*;

// Braille pattern unicode characters
// Each braille character represents 8 dots in a 2x4 grid
// The dots are numbered as follows:
// 1 4
// 2 5
// 3 6
// 7 8
const BRAILLE_PATTERNS: [char; 256] = [
    '\u{2800}', '\u{2801}', '\u{2802}', '\u{2803}', '\u{2804}', '\u{2805}', '\u{2806}', '\u{2807}',
    '\u{2808}', '\u{2809}', '\u{280A}', '\u{280B}', '\u{280C}', '\u{280D}', '\u{280E}', '\u{280F}',
    '\u{2810}', '\u{2811}', '\u{2812}', '\u{2813}', '\u{2814}', '\u{2815}', '\u{2816}', '\u{2817}',
    '\u{2818}', '\u{2819}', '\u{281A}', '\u{281B}', '\u{281C}', '\u{281D}', '\u{281E}', '\u{281F}',
    '\u{2820}', '\u{2821}', '\u{2822}', '\u{2823}', '\u{2824}', '\u{2825}', '\u{2826}', '\u{2827}',
    '\u{2828}', '\u{2829}', '\u{282A}', '\u{282B}', '\u{282C}', '\u{282D}', '\u{282E}', '\u{282F}',
    '\u{2830}', '\u{2831}', '\u{2832}', '\u{2833}', '\u{2834}', '\u{2835}', '\u{2836}', '\u{2837}',
    '\u{2838}', '\u{2839}', '\u{283A}', '\u{283B}', '\u{283C}', '\u{283D}', '\u{283E}', '\u{283F}',
    '\u{2840}', '\u{2841}', '\u{2842}', '\u{2843}', '\u{2844}', '\u{2845}', '\u{2846}', '\u{2847}',
    '\u{2848}', '\u{2849}', '\u{284A}', '\u{284B}', '\u{284C}', '\u{284D}', '\u{284E}', '\u{284F}',
    '\u{2850}', '\u{2851}', '\u{2852}', '\u{2853}', '\u{2854}', '\u{2855}', '\u{2856}', '\u{2857}',
    '\u{2858}', '\u{2859}', '\u{285A}', '\u{285B}', '\u{285C}', '\u{285D}', '\u{285E}', '\u{285F}',
    '\u{2860}', '\u{2861}', '\u{2862}', '\u{2863}', '\u{2864}', '\u{2865}', '\u{2866}', '\u{2867}',
    '\u{2868}', '\u{2869}', '\u{286A}', '\u{286B}', '\u{286C}', '\u{286D}', '\u{286E}', '\u{286F}',
    '\u{2870}', '\u{2871}', '\u{2872}', '\u{2873}', '\u{2874}', '\u{2875}', '\u{2876}', '\u{2877}',
    '\u{2878}', '\u{2879}', '\u{287A}', '\u{287B}', '\u{287C}', '\u{287D}', '\u{287E}', '\u{287F}',
    '\u{2880}', '\u{2881}', '\u{2882}', '\u{2883}', '\u{2884}', '\u{2885}', '\u{2886}', '\u{2887}',
    '\u{2888}', '\u{2889}', '\u{288A}', '\u{288B}', '\u{288C}', '\u{288D}', '\u{288E}', '\u{288F}',
    '\u{2890}', '\u{2891}', '\u{2892}', '\u{2893}', '\u{2894}', '\u{2895}', '\u{2896}', '\u{2897}',
    '\u{2898}', '\u{2899}', '\u{289A}', '\u{289B}', '\u{289C}', '\u{289D}', '\u{289E}', '\u{289F}',
    '\u{28A0}', '\u{28A1}', '\u{28A2}', '\u{28A3}', '\u{28A4}', '\u{28A5}', '\u{28A6}', '\u{28A7}',
    '\u{28A8}', '\u{28A9}', '\u{28AA}', '\u{28AB}', '\u{28AC}', '\u{28AD}', '\u{28AE}', '\u{28AF}',
    '\u{28B0}', '\u{28B1}', '\u{28B2}', '\u{28B3}', '\u{28B4}', '\u{28B5}', '\u{28B6}', '\u{28B7}',
    '\u{28B8}', '\u{28B9}', '\u{28BA}', '\u{28BB}', '\u{28BC}', '\u{28BD}', '\u{28BE}', '\u{28BF}',
    '\u{28C0}', '\u{28C1}', '\u{28C2}', '\u{28C3}', '\u{28C4}', '\u{28C5}', '\u{28C6}', '\u{28C7}',
    '\u{28C8}', '\u{28C9}', '\u{28CA}', '\u{28CB}', '\u{28CC}', '\u{28CD}', '\u{28CE}', '\u{28CF}',
    '\u{28D0}', '\u{28D1}', '\u{28D2}', '\u{28D3}', '\u{28D4}', '\u{28D5}', '\u{28D6}', '\u{28D7}',
    '\u{28D8}', '\u{28D9}', '\u{28DA}', '\u{28DB}', '\u{28DC}', '\u{28DD}', '\u{28DE}', '\u{28DF}',
    '\u{28E0}', '\u{28E1}', '\u{28E2}', '\u{28E3}', '\u{28E4}', '\u{28E5}', '\u{28E6}', '\u{28E7}',
    '\u{28E8}', '\u{28E9}', '\u{28EA}', '\u{28EB}', '\u{28EC}', '\u{28ED}', '\u{28EE}', '\u{28EF}',
    '\u{28F0}', '\u{28F1}', '\u{28F2}', '\u{28F3}', '\u{28F4}', '\u{28F5}', '\u{28F6}', '\u{28F7}',
    '\u{28F8}', '\u{28F9}', '\u{28FA}', '\u{28FB}', '\u{28FC}', '\u{28FD}', '\u{28FE}', '\u{28FF}',
];

fn get_braille_pattern(pattern: u8) -> char {
    BRAILLE_PATTERNS[pattern as usize]
}

fn render(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, f: fn(p: Pixel) -> Color) {
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
                    
                    // Accumulate RGB values for averaging
                    total_r += pixel.red as u32;
                    total_g += pixel.green as u32;
                    total_b += pixel.blue as u32;
                    pixel_count += 1;
                    
                    // Determine if this dot should be visible based on luminance
                    let luminance = pixel.luminance();
                    if luminance > 128 { // Threshold for visibility
                        pattern |= 1 << i;
                    }
                }
            }
            
            // Create braille character
            let braille_char = get_braille_pattern(pattern);
            
            // Compute average color and apply color function
            let foreground = if pixel_count > 0 {
                let avg_r = (total_r / pixel_count) as u8;
                let avg_g = (total_g / pixel_count) as u8;
                let avg_b = (total_b / pixel_count) as u8;
                let avg_pixel = Pixel::with_rgb(avg_r, avg_g, avg_b);
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
    Size::new(img.width() / 2, img.height() / 4)
}

#[inline(always)]
pub(crate) fn paint(surface: &mut Surface, img: &Image, x: i32, y: i32, rap: u32, color_schema: ColorSchema) {
    match color_schema {
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
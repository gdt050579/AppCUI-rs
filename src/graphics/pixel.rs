use super::Color;

const COLOR_TO_PIXEL: [u32; 16] = [
    0x000000, // Black
    0x000080, // DarkBlue
    0x008000, // DarkGreen
    0x008080, // Teal
    0x800000, // DarkRed
    0x800080, // Purple
    0x808000, // Brown
    0xC0C0C0, // LightGray
    0x808080, // LightGray
    0x0000FF, // Blue
    0x00FF00, // Green
    0x00FFFF, // Aqua
    0xFF0000, // Red
    0xFF00FF, // Pink
    0xFFFF00, // Yellow
    0xFFFFFF, // White
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Pixel {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Pixel {
            red,
            green,
            blue,
            alpha: 255,
        }
    }
    pub fn from_color(color: Color) -> Self {
        let idx = color as u8;
        if idx > 16 {
            Pixel {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            }
        } else {
            let u = COLOR_TO_PIXEL[idx as usize];
            Pixel {
                red: ((u >> 24) & 0xFF) as u8,
                green: ((u >> 16) & 0xFF) as u8,
                blue: ((u & 0xFF) as u8),
                alpha: 255,
            }
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { red: 0, green: 0, blue: 0, alpha: 0 }
    }
}
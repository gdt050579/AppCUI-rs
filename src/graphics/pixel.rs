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

const COLORMAP_16_COLORS: [Color; 27] = [
    /* 0*/ Color::Black, // (0, 0, 0)
    /* 1*/ Color::DarkBlue, // (0, 0, 1)
    /* 2*/ Color::Blue, // (0, 0, 2)
    /* 3*/ Color::DarkGreen, // (0, 1, 0)
    /* 4*/ Color::Teal, // (0, 1, 1)
    /* 5*/ Color::Teal, // (0, 1, 2) [Aprox]
    /* 6*/ Color::Green, // (0, 2, 0)
    /* 7*/ Color::Teal, // (0, 2, 1) [Aprox]
    /* 8*/ Color::Aqua, // (0, 2, 2)
    /* 9*/ Color::DarkRed, // (1, 0, 0)
    /*10*/ Color::Magenta, // (1, 0, 1)
    /*11*/ Color::Magenta, // (1, 0, 2) [Aprox]
    /*12*/ Color::Olive, // (1, 1, 0)
    /*13*/ Color::Gray, // (1, 1, 1)
    /*14*/ Color::Gray, // (1, 1, 2) [Aprox]
    /*15*/ Color::Olive, // (1, 2, 0) [Aprox]
    /*16*/ Color::Gray, // (1, 2, 1) [Aprox]
    /*17*/ Color::Silver, // (1, 2, 2) [Aprox]
    /*18*/ Color::Red, // (2, 0, 0)
    /*19*/ Color::Magenta, // (2, 0, 1) [Aprox]
    /*20*/ Color::Pink, // (2, 0, 2)
    /*21*/ Color::Olive, // (2, 1, 0) [Aprox]
    /*22*/ Color::Gray, // (2, 1, 1) [Aprox]
    /*23*/ Color::Silver, // (2, 1, 2) [Aprox]
    /*24*/ Color::Yellow, // (2, 2, 0)
    /*25*/ Color::Silver, // (2, 2, 1) [Aprox]
    /*26*/ Color::White, // (2, 2, 2)
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
        if idx >= 16 {
            Pixel {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            }
        } else {
            let u = COLOR_TO_PIXEL[idx as usize];
            Pixel {
                red: ((u >> 16) & 0xFF) as u8,
                green: ((u >> 8) & 0xFF) as u8,
                blue: ((u & 0xFF) as u8),
                alpha: 255,
            }
        }
    }
    pub fn to_color(&self) -> Color {
        let b = if self.blue <= 16 {
            0u32
        } else {
            if self.blue < 192 {
                1u32
            } else {
                2u32
            }
        };
        let r = if self.red <= 16 {
            0u32
        } else {
            if self.red < 192 {
                1u32
            } else {
                2u32
            }
        };
        let g = if self.green <= 16 {
            0u32
        } else {
            if self.green < 192 {
                1u32
            } else {
                2u32
            }
        };
        return COLORMAP_16_COLORS[(r * 9 + g * 3 + b) as usize];
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
}

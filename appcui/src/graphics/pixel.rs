use super::CharFlags;
use super::Character;
use super::Color;
use super::SpecialChar;

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

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Pixel { red, green, blue, alpha }
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
    pub(super) fn as_color(&self) -> Color {
        let b = if self.blue <= 16 {
            0u32
        } else if self.blue < 192 {
            1u32
        } else {
            2u32
        };
        let r = if self.red <= 16 {
            0u32
        } else if self.red < 192 {
            1u32
        } else {
            2u32
        };
        let g = if self.green <= 16 {
            0u32
        } else if self.green < 192 {
            1u32
        } else {
            2u32
        };
        COLORMAP_16_COLORS[(r * 9 + g * 3 + b) as usize]
    }
    pub(super) fn as_character(&self) -> Character {
        let r = ((self.red as u32) + 32) / 64;
        let g = ((self.green as u32) + 32) / 64;
        let b = ((self.blue as u32) + 32) / 64;
        let idx = (r * 25 + g * 5 + b) as usize;
        let col = COLORMAP_64_COLORS[idx];
        let proc = COLORMAP_64_COLORS_PROC[idx];
        match proc {
            0 => {
                Character::new(' ', Color::Black, Color::Black, CharFlags::None)
            }
            25 => {
                Character::new(SpecialChar::Block25, col, Color::Black, CharFlags::None)
            }
            50 => {
                Character::new(SpecialChar::Block50, col, Color::Black, CharFlags::None)
            }
            75 => {
                Character::new(SpecialChar::Block75, col, Color::Black, CharFlags::None)
            }
            100 => {
                Character::new(' ', col, col, CharFlags::None)
            }
            _ => {
                Character::default()
            }
        }
    }
    pub(super) fn as_gray_scale_character(&self) -> Character {
        let val = ((self.blue as u32) + (self.red as u32) + (self.green as u32)) / 3;
        if val < 32 {
            return Character::new(' ', Color::Black, Color::Black, CharFlags::None);
        }
        if val < 96 {
            return Character::new(SpecialChar::Block25, Color::White, Color::Black, CharFlags::None);
        }
        if val < 160 {
            return Character::new(SpecialChar::Block50, Color::White, Color::Black, CharFlags::None);
        }
        if val < 224 {
            Character::new(SpecialChar::Block75, Color::White, Color::Black, CharFlags::None)
        } else {
            Character::new(' ', Color::White, Color::White, CharFlags::None)
        }
    }
}
impl From<u32> for Pixel {
    fn from(value: u32) -> Self {
        Self {
            blue: (value & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8, 
            red: ((value >> 16) & 0xFF) as u8,
            alpha: ((value >> 24) & 0xFF) as u8,
        }
    }
}
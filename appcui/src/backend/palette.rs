use crate::graphics::Color;

#[derive(Debug, Clone, Copy)]
pub enum Palette {
    Terminal,
    BuiltIn,
    Custom(ColorPalette),
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub(crate) colors: [(u8, u8, u8); 16],
}

impl ColorPalette {
    pub const fn built_in() -> Self {
        Self {
            colors: [
                (0, 0, 0),
                (0, 0, 128),
                (0, 128, 0),
                (0, 128, 128),
                (128, 0, 0),
                (128, 0, 128),
                (128, 128, 0),
                (192, 192, 192),
                (128, 128, 128),
                (0, 0, 255),
                (0, 255, 0),
                (0, 255, 255),
                (255, 0, 0),
                (255, 0, 255),
                (255, 255, 0),
                (255, 255, 255),
            ],
        }
    }
    pub fn from_pairs(pairs: &[(Color, u8, u8, u8)]) -> Self {
        let mut colors = [(0, 0, 0); 16];
        for (color, r, g, b) in pairs {
            let index = color.as_color_index();
            assert!(index < 16, "You can only use the first 16 colors in a palette (Black, DarkBlue, DarkGreen, Teal, DarkRed, Magenta, Olive, Silver, Gray, Blue, Green, Aqua, Red, Pink, Yellow, White) !");
            colors[index as usize] = (*r, *g, *b);
        }
        Self { colors }
    }
}

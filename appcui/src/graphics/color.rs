/// Represents an enum with variants that can be used to describe the foreground and background colors of a character in a terminal or console application.
#[cfg(not(feature = "TRUE_COLORS"))]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Color {
    /// <table><tr><td style="background-color: #000000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    #[default]
    Black = 0x00,

    /// <table><tr><td style="background-color: #000080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkBlue = 0x01,

    /// <table><tr><td style="background-color: #008000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkGreen = 0x02,

    /// <table><tr><td style="background-color: #008080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Teal = 0x03,

    /// <table><tr><td style="background-color: #800000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkRed = 0x04,

    /// <table><tr><td style="background-color: #800080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Magenta = 0x05,

    /// <table><tr><td style="background-color: #808000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Olive = 0x06,

    /// <table><tr><td style="background-color: #C0C0C0; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Silver = 0x07,

    /// <table><tr><td style="background-color: #808080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Gray = 0x08,

    /// <table><tr><td style="background-color: #0000FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Blue = 0x09,

    /// <table><tr><td style="background-color: #00FF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Green = 0x0A,

    /// <table><tr><td style="background-color: #00FFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Aqua = 0x0B,

    /// <table><tr><td style="background-color: #FF0000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Red = 0x0C,

    /// <table><tr><td style="background-color: #FF00FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Pink = 0x0D,

    /// <table><tr><td style="background-color: #FFFF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Yellow = 0x0E,

    /// <table><tr><td style="background-color: #FFFFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    White = 0x0F,

    /// a marker for transparent color
    Transparent = 0x10,
}

/// Represents an enum with variants that can be used to describe the foreground and background colors of a character in a terminal or console application.
#[cfg(feature = "TRUE_COLORS")]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Color {
    /// <table><tr><td style="background-color: #000000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    #[default]
    Black = 0x00,

    /// <table><tr><td style="background-color: #000080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkBlue = 0x01,

    /// <table><tr><td style="background-color: #008000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkGreen = 0x02,

    /// <table><tr><td style="background-color: #008080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Teal = 0x03,

    /// <table><tr><td style="background-color: #800000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkRed = 0x04,

    /// <table><tr><td style="background-color: #800080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Magenta = 0x05,

    /// <table><tr><td style="background-color: #808000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Olive = 0x06,

    /// <table><tr><td style="background-color: #C0C0C0; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Silver = 0x07,

    /// <table><tr><td style="background-color: #808080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Gray = 0x08,

    /// <table><tr><td style="background-color: #0000FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Blue = 0x09,

    /// <table><tr><td style="background-color: #00FF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Green = 0x0A,

    /// <table><tr><td style="background-color: #00FFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Aqua = 0x0B,

    /// <table><tr><td style="background-color: #FF0000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Red = 0x0C,

    /// <table><tr><td style="background-color: #FF00FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Pink = 0x0D,

    /// <table><tr><td style="background-color: #FFFF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Yellow = 0x0E,

    /// <table><tr><td style="background-color: #FFFFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    White = 0x0F,

    /// a marker for transparent color
    Transparent = 0x10,

    /// A 24-bit RGB color, for example `(255, 128, 0)`.
    RGB(u8, u8, u8),
}
impl Color {
    /// Returns a high-contrast color (`Black` or `White`) for drawing over this color.
    ///
    /// Dark palette colors map to [`White`](Self::White), light ones to [`Black`](Self::Black).
    /// [`Transparent`](Self::Transparent) is unchanged. With the `TRUE_COLORS` feature, RGB values
    /// use luminance: light colors return black, dark colors return white.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// assert_eq!(Color::Black.contrast_color(), Color::White);
    /// assert_eq!(Color::Yellow.contrast_color(), Color::Black);
    /// ```
    #[inline(always)]
    pub fn contrast_color(&self) -> Self {
        match self {
            Color::Black | Color::DarkBlue | Color::DarkGreen | Color::Teal | Color::DarkRed | Color::Magenta | Color::Olive | Color::Gray | Color::Blue | Color::Red => Color::White,
            Color::Silver | Color::Green | Color::Aqua | Color::Pink | Color::Yellow | Color::White => Color::Black,
            Color::Transparent => Color::Transparent,
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                if (2126 * (*r as i32) + 7152 * (*g as i32) + 722 * (*b as i32)) > 1275000 {
                    Color::Black
                } else {
                    Color::White
                }
            }
        }
    }
    /// Returns the complementary (inverted) color.
    ///
    /// Each of the 16 palette colors maps to its opposite (for example [`Black`](Self::Black) ↔
    /// [`White`](Self::White), [`Red`](Self::Red) ↔ [`Teal`](Self::Teal)).
    /// [`Transparent`](Self::Transparent) is unchanged. With the `TRUE_COLORS` feature, RGB channels
    /// are inverted as `255 - channel`.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// assert_eq!(Color::Black.inverse_color(), Color::White);
    /// assert_eq!(Color::Red.inverse_color(), Color::Teal);
    /// ```
    #[inline(always)]
    pub fn inverse_color(&self) -> Self {
        match self {
            Color::Black => Color::White,     // #000000 → #FFFFFF
            Color::DarkBlue => Color::Yellow, // #000080 → #FFFF7F
            Color::DarkGreen => Color::Pink,  // #008000 → #FF7FFF
            Color::Teal => Color::Red,        // #008080 → #FF7F7F
            Color::DarkRed => Color::Aqua,    // #800000 → #7FFFFF
            Color::Magenta => Color::Green,   // #800080 → #7FFF7F
            Color::Olive => Color::Blue,      // #808000 → #7F7FFF
            Color::Silver => Color::Gray,     // #C0C0C0 → #3F3F3F
            Color::Gray => Color::Silver,     // #808080 → #7F7F7F
            Color::Blue => Color::Olive,      // #0000FF → #FFFF00
            Color::Green => Color::Magenta,   // #00FF00 → #FF00FF
            Color::Aqua => Color::DarkRed,    // #00FFFF → #FF0000
            Color::Red => Color::Teal,        // #FF0000 → #00FFFF
            Color::Pink => Color::DarkGreen,  // #FF00FF → #00FF00
            Color::Yellow => Color::DarkBlue, // #FFFF00 → #0000FF
            Color::White => Color::Black,     // #FFFFFF → #000000
            Color::Transparent => Color::Transparent,
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => Color::from_rgb(255 - *r, 255 - *g, 255 - *b),
        }
    }
    /// Builds a palette color from its numeric index (`0`–`16`).
    ///
    /// Indexes match the variant discriminants: `0` is [`Black`](Self::Black), `15` is
    /// [`White`](Self::White), and `16` is [`Transparent`](Self::Transparent). Any other value
    /// returns `None`. RGB colors cannot be constructed this way.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// assert_eq!(Color::from_value(12), Some(Color::Red));
    /// assert_eq!(Color::from_value(99), None);
    /// ```
    pub fn from_value(value: i32) -> Option<Color> {
        match value {
            0 => Some(Color::Black),
            1 => Some(Color::DarkBlue),
            2 => Some(Color::DarkGreen),
            3 => Some(Color::Teal),
            4 => Some(Color::DarkRed),
            5 => Some(Color::Magenta),
            6 => Some(Color::Olive),
            7 => Some(Color::Silver),
            8 => Some(Color::Gray),
            9 => Some(Color::Blue),
            10 => Some(Color::Green),
            11 => Some(Color::Aqua),
            12 => Some(Color::Red),
            13 => Some(Color::Pink),
            14 => Some(Color::Yellow),
            15 => Some(Color::White),
            16 => Some(Color::Transparent),
            _ => None,
        }
    }
    /// Returns the variant name, such as `"Red"` or `"Transparent"`.
    ///
    /// With the `TRUE_COLORS` feature, any 24-bit RGB value returns `"RGB"`.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// assert_eq!(Color::Aqua.name(), "Aqua");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Color::Black => "Black",
            Color::DarkBlue => "DarkBlue",
            Color::DarkGreen => "DarkGreen",
            Color::Teal => "Teal",
            Color::DarkRed => "DarkRed",
            Color::Magenta => "Magenta",
            Color::Olive => "Olive",
            Color::Silver => "Silver",
            Color::Gray => "Gray",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Aqua => "Aqua",
            Color::Red => "Red",
            Color::Pink => "Pink",
            Color::Yellow => "Yellow",
            Color::White => "White",
            Color::Transparent => "Transparent",
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(_, _, _) => "RGB",
        }
    }
    #[inline(always)]
    pub(crate) fn as_color_index(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::DarkBlue => 1,
            Color::DarkGreen => 2,
            Color::Teal => 3,
            Color::DarkRed => 4,
            Color::Magenta => 5,
            Color::Olive => 6,
            Color::Silver => 7,
            Color::Gray => 8,
            Color::Blue => 9,
            Color::Green => 10,
            Color::Aqua => 11,
            Color::Red => 12,
            Color::Pink => 13,
            Color::Yellow => 14,
            Color::White => 15,
            Color::Transparent => 16,
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                let mut index = 0;
                if *r > 64 {
                    index |= 0b100;
                }
                if *g > 64 {
                    index |= 0b010;
                }
                if *b > 64 {
                    index |= 0b001;
                }
                // 192 (0xc0) should remain like this so that we obtain th Silver
                if *r >= 196 || *g >= 196 || *b >= 196 {
                    index |= 0b1000;
                }
                index
            }
        }
    }

    // #[cfg(feature = "TRUE_COLORS")]
    // #[inline(always)]
    // pub(crate) fn is_rgb(&self) -> bool {
    //     matches!(self, Color::RGB(_, _, _))
    // }
    // #[cfg(not(feature = "TRUE_COLORS"))]
    // #[inline(always)]
    // pub(crate) fn is_rgb(&self) -> bool {
    //     false
    // }
    #[cfg(feature = "TRUE_COLORS")]
    #[inline(always)]
    pub(crate) fn rgb(&self) -> Option<(u8, u8, u8)> {
        match self {
            Color::RGB(r, g, b) => Some((*r, *g, *b)),
            _ => None,
        }
    }
    #[cfg(not(feature = "TRUE_COLORS"))]
    #[inline(always)]
    pub(crate) fn rgb(&self) -> Option<(u8, u8, u8)> {
        None
    }

    /// Builds a color from 8-bit red, green, and blue components.
    ///
    /// With the `TRUE_COLORS` feature this stores a 24-bit RGB color. Without it, the value is
    /// quantized to the nearest of the 16 palette colors.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let white = Color::from_rgb(255, 255, 255);
    /// assert_eq!(white.contrast_color(), Color::Black);
    /// ```
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        #[cfg(feature = "TRUE_COLORS")]
        return Color::RGB(r, g, b);
        #[cfg(not(feature = "TRUE_COLORS"))]
        {
            let mut index = 0;
            if r > 64 {
                index |= 0b100;
            }
            if g > 64 {
                index |= 0b010;
            }
            if b > 64 {
                index |= 0b001;
            }
            // 192 (0xc0) should remain like this so that we obtain th Silver
            if r >= 196 || g >= 196 || b >= 196 {
                index |= 0b1000;
            }
            Color::from_value(index).unwrap()
        }
    }
}

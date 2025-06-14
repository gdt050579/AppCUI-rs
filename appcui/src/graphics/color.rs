/// Represents am enum with varians that can be used to describe the foreground and background colors of a character in a terminal or console application.
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

    RGB(u8, u8, u8),
}
impl Color {
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
    #[cfg(feature = "TRUE_COLORS")]
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
    #[cfg(not(feature = "TRUE_COLORS"))]
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
    pub(crate) fn rgb(&self) -> Option<(u8,u8,u8)> {
        match self {
            Color::RGB(r, g, b) => Some((*r,*g,*b)),
            _ => None
        }
    }
    #[cfg(not(feature = "TRUE_COLORS"))]
    #[inline(always)]
    pub(crate) fn rgb(&self) -> Option<(u8,u8,u8)> {
        None
    }

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

use super::Color;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
pub enum CharFlags {
    Bold = 0x0001,
    Italic = 0x0002,
    Underline = 0x0004,
}

/// Represents attributes of a character such as foreground color, background color, and flags.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CharAttribute {
    pub foreground: Color,
    pub background: Color,
    pub flags: CharFlags,
}

impl CharAttribute {
    /// Creates a new `CharAttribute` with the specified foreground [Color], background [Color], and flags.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let attr = CharAttribute::new(Color::Red, Color::Black, CharFlags::Bold);
    /// ```
    pub fn new(fore: Color, back: Color, flags: CharFlags) -> CharAttribute {
        CharAttribute {
            foreground: fore,
            background: back,
            flags,
        }
    }

    /// Creates a new `CharAttribute` with the specified foreground and background colors.
    /// The flags are set to `CharFlags::None`.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let attr = CharAttribute::with_color(Color::Red, Color::Black);
    /// ```
    pub fn with_color(fore: Color, back: Color) -> CharAttribute {
        CharAttribute {
            foreground: fore,
            background: back,
            flags: CharFlags::None,
        }
    }

    /// Creates a new `CharAttribute` with the specified foreground color.
    /// The background color is set to `Color::Transparent` and the flags are set to `CharFlags::None`.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let attr = CharAttribute::with_fore_color(Color::Red);
    /// ```
    pub fn with_fore_color(fore: Color) -> CharAttribute {
        CharAttribute {
            foreground: fore,
            background: Color::Transparent,
            flags: CharFlags::None,
        }
    }

    /// Creates a new `CharAttribute` with the specified background color.
    /// The foreground color is set to `Color::Transparent` and the flags are set to `CharFlags::None`.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let attr = CharAttribute::with_back_color(Color::Black);
    /// ```
    pub fn with_back_color(back: Color) -> CharAttribute {
        CharAttribute {
            foreground: Color::Transparent,
            background: back,
            flags: CharFlags::None,
        }
    }
}
impl Default for CharAttribute {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            flags: CharFlags::None,
        }
    }
}

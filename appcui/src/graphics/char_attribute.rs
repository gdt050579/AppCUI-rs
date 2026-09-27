use super::Color;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
/// Text style bits stored in a [`CharAttribute`].
///
/// Combine values with `|`. `CharFlags::None` means no extra styling (plain glyphs).
/// Support depends on the terminal backend.
pub enum CharFlags {
    /// Bold / increased intensity.
    Bold = 0x0001,
    /// Italic slant.
    Italic = 0x0002,
    /// Single underline.
    Underline = 0x0004,
    /// Double underline.
    DoubleUnderline = 0x0008,
    /// Wavy / curly underline.
    CurlyUnderline = 0x0010,
    /// Dotted underline.
    DottedUnderline = 0x0020,
    /// Strike-through (a line through the glyph).
    StrikeThrough = 0x0040, 
}

/// Represents attributes of a character such as foreground color, background color, and flags.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CharAttribute {
    /// Foreground (text) color.
    pub foreground: Color,
    /// Background color.
    pub background: Color,
    /// Style flags such as bold or underline.
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

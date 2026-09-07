use super::CharAttribute;
use super::CharFlags;
use super::Color;

static UNICODE_CODES: [char; 48] = [
    '\u{2554}', '\u{2557}', '\u{255D}', '\u{255A}', '\u{2550}', '\u{2551}', '\u{256C}', // double line box
    '\u{250C}', '\u{2510}', '\u{2518}', '\u{2514}', '\u{2500}', '\u{2502}', '\u{253C}', // single line box
    '\u{2191}', '\u{2193}', '\u{2190}', '\u{2192}', '\u{2195}', '\u{2194}', // arrows
    '\u{20}', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2588}', '\u{2580}', '\u{2584}', '\u{258C}', '\u{2590}', '\u{25A0}', // blocks
    '\u{25B2}', '\u{25BC}', '\u{25C4}', '\u{25BA}', // Trangles
    '\u{25CF}', '\u{25CB}', '\u{221A}', '\u{2261}', '\u{205E}', '\u{2026}', // symbols
    '\u{251C}', '\u{252C}', '\u{2524}', '\u{2534}', // middle single line box
    '\u{2594}', '\u{2587}', '\u{2595}', '\u{2581}', // middle single line box
    
];

/// A special character set used for drawing boxes, arrows, and other graphical elements in a terminal or console application.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum SpecialChar {
    /// Double-line top-left corner: `╔`.
    BoxTopLeftCornerDoubleLine = 0,
    /// Double-line top-right corner: `╗`.
    BoxTopRightCornerDoubleLine,
    /// Double-line bottom-right corner: `╝`.
    BoxBottomRightCornerDoubleLine,
    /// Double-line bottom-left corner: `╚`.
    BoxBottomLeftCornerDoubleLine,
    /// Double-line horizontal: `═`.
    BoxHorizontalDoubleLine,
    /// Double-line vertical: `║`.
    BoxVerticalDoubleLine,
    /// Double-line cross: `╬`.
    BoxCrossDoubleLine,

    /// Single-line top-left corner: `┌`.
    BoxTopLeftCornerSingleLine,
    /// Single-line top-right corner: `┐`.
    BoxTopRightCornerSingleLine,
    /// Single-line bottom-right corner: `┘`.
    BoxBottomRightCornerSingleLine,
    /// Single-line bottom-left corner: `└`.
    BoxBottomLeftCornerSingleLine,
    /// Single-line horizontal: `─`.
    BoxHorizontalSingleLine,
    /// Single-line vertical: `│`.
    BoxVerticalSingleLine,
    /// Single-line cross: `┼`.
    BoxCrossSingleLine,

    /// Up arrow: `↑`.
    ArrowUp,
    /// Down arrow: `↓`.
    ArrowDown,
    /// Left arrow: `←`.
    ArrowLeft,
    /// Right arrow: `→`.
    ArrowRight,
    /// Vertical double arrow: `↕`.
    ArrowUpDown,
    /// Horizontal double arrow: `↔`.
    ArrowLeftRight,

    /// Empty block (space), 0% fill.
    Block0,
    /// Light shade: `░` (25%).
    Block25,
    /// Medium shade: `▒` (50%).
    Block50,
    /// Dark shade: `▓` (75%).
    Block75,
    /// Full block: `█` (100%).
    Block100,
    /// Upper half block: `▀`.
    BlockUpperHalf,
    /// Lower half block: `▄`.
    BlockLowerHalf,
    /// Left half block: `▌`.
    BlockLeftHalf,
    /// Right half block: `▐`.
    BlockRightHalf,
    /// Centered square: `■`.
    BlockCentered,

    /// Up-pointing triangle: `▲`.
    TriangleUp,
    /// Down-pointing triangle: `▼`.
    TriangleDown,
    /// Left-pointing triangle: `◄`.
    TriangleLeft,
    /// Right-pointing triangle: `►`.
    TriangleRight,

    /// Filled circle: `●`.
    CircleFilled,
    /// Empty circle: `○`.
    CircleEmpty,
    /// Check mark: `√`.
    CheckMark,
    /// Menu/hamburger sign: `≡`.
    MenuSign,
    /// Four vertical dots: `⁞`.
    FourPoints,
    /// Horizontal ellipsis: `…`.
    ThreePointsHorizontal,

    /// Single-line T-junction on the left: `├`.
    BoxMidleLeft,
    /// Single-line T-junction on the top: `┬`.
    BoxMidleTop,
    /// Single-line T-junction on the right: `┤`.
    BoxMidleRight,
    /// Single-line T-junction on the bottom: `┴`.
    BoxMidleBottom,

    /// A line along the top of the cell: `▔`.
    LineOnTop,
    /// A thick bar on the left of the cell: `▇`.
    LineOnLeft,
    /// A line along the right of the cell: `▕`.
    LineOnRight,
    /// A line along the bottom of the cell: `▁`.
    LineOnBottom,
}

impl From<SpecialChar> for char {
    fn from(value: SpecialChar) -> Self {
        UNICODE_CODES[value as usize]
    }
}


/// Represents a character with its attributes, including foreground and background colors, and flags.
/// The `Character` struct is used to define a character that can be displayed on the screen.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Character {
    pub code: char,
    pub foreground: Color,
    pub background: Color,
    pub flags: CharFlags,
}
impl Character {
    /// Creates a new character with the specified code, foreground color, background color, and flags.
    /// The code could be of type `char` or a SpecialChar.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// let ch = Character::new('A', Color::Red, Color::Black, CharFlags::None);
    /// let special_ch = Character::new(SpecialChar::BoxTopLeftCornerDoubleLine, 
    ///                                 Color::Green, 
    ///                                 Color::Black, 
    ///                                 CharFlags::None);
    /// ```
    #[inline(always)]
    pub fn new<T>(code: T, fore: Color, back: Color, flags: CharFlags) -> Self
    where
        char: From<T>,
    {
        Character {
            code: char::from(code),
            foreground: fore,
            background: back,
            flags,
        }
    }

    /// Creates a new character from a specified code (a char or a SpecialChar).
    /// The foreground and background colors are set to transparent and th e flags are set to None.
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// let ch = Character::with_char('A');
    /// let special_ch = Character::with_char(SpecialChar::BoxTopLeftCornerDoubleLine);
    /// ```
    #[inline(always)]
    pub fn with_char<T>(code: T) -> Self
    where
        char: From<T>,
    {
        Character {
            code: char::from(code),
            foreground: Color::Transparent,
            background: Color::Transparent,
            flags: CharFlags::None,
        }
    }

    /// Creates a new character with the specified foreground and background colors.
    /// The code is set to the null character ('\0') - meaning that it will not overwrite the existing character when displayed on the surface.
    /// The flags are set to None.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let ch = Character::with_color(Color::Red, Color::Black);
    /// ```
    #[inline(always)]
    pub fn with_color(fore: Color, back: Color) -> Self {
        Character {
            code: 0 as char,
            foreground: fore,
            background: back,
            flags: CharFlags::None,
        }
    }

    /// Creates a new character with the specified code (a char or a SpecialChar) and attributes.
    /// The foreground and background colors are set to the values specified in the `CharAttribute` struct.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let attr = CharAttribute::new(Color::Red, Color::Black, CharFlags::None);
    /// let ch = Character::with_attributes('A', attr);
    /// let special_ch = Character::with_attributes(SpecialChar::BoxTopLeftCornerDoubleLine, attr);
    /// ```
    #[inline(always)]
    pub fn with_attributes<T>(code: T, attr: CharAttribute) -> Self
    where
        char: From<T>,
    {
        Character {
            code: char::from(code),
            foreground: attr.foreground,
            background: attr.background,
            flags: attr.flags,
        }
    }

    /// Sets an character based on the provided `Character` instance.
    /// The code, foreground color, and background color are updated only if they are not set to the default values 
    /// Foe example, the character code is only set if the **ch** argument is not 0.
    /// Similarly, if the background or foreground colors are not set to `Color::Transparent`, they will be updated.
    /// The flags are always updated.
    #[inline(always)]
    pub fn set(&mut self, ch: Character) {
        if ch.code != (0 as char) {
            self.code = ch.code;
        }
        if ch.foreground != Color::Transparent {
            self.foreground = ch.foreground;
        }
        if ch.background != Color::Transparent {
            self.background = ch.background;
        }
        self.flags = ch.flags;
    }
}

impl Default for Character {
    /// Creates a new character with default values.
    /// The code is set to a space character (' '), the foreground color is set to `Color::White`,
    /// the background color is set to `Color::Black`, and the flags are set to `CharFlags::None`.
    fn default() -> Self {
        Self {
            code: ' ',
            foreground: Color::White,
            background: Color::Black,
            flags: CharFlags::None,
        }
    }
}

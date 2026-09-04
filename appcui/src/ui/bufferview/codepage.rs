use flat_string::FlatString;

// CP437
static CP437: [char; 256] = [
    ' ', '☺', '☻', '♥', '♦', '♣', '♠', '•', // 0x00
    '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', // 0x08
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', // 0x10
    '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', // 0x18
    ' ', '!', '"', '#', '$', '%', '&', '\'', // 0x20
    '(', ')', '*', '+', ',', '-', '.', '/', // 0x28
    '0', '1', '2', '3', '4', '5', '6', '7', // 0x30
    '8', '9', ':', ';', '<', '=', '>', '?', // 0x38
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', // 0x40
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 0x48
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', // 0x50
    'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 0x58
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', // 0x60
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 0x68
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', // 0x70
    'x', 'y', 'z', '{', '|', '}', '~', '⌂', // 0x78
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', // 0x80
    'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', // 0x88
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', // 0x90
    'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', // 0x98
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', // 0xA0
    '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', // 0xA8
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', // 0xB0
    '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', // 0xB8
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', // 0xC0
    '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', // 0xC8
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', // 0xD0
    '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', // 0xD8
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', // 0xE0
    'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', // 0xE8
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', // 0xF0
    '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ', // 0xF8
];

// WINDOWS_1252
static WINDOWS_1252: [char; 256] = [
    ' ', '☺', '☻', '♥', '♦', '♣', '♠', '•', // 0x00
    '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', // 0x08
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', // 0x10
    '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', // 0x18
    ' ', '!', '"', '#', '$', '%', '&', '\'', // 0x20
    '(', ')', '*', '+', ',', '-', '.', '/', // 0x28
    '0', '1', '2', '3', '4', '5', '6', '7', // 0x30
    '8', '9', ':', ';', '<', '=', '>', '?', // 0x38
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', // 0x40
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 0x48
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', // 0x50
    'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 0x58
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', // 0x60
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 0x68
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', // 0x70
    'x', 'y', 'z', '{', '|', '}', '~', '⌂', // 0x78
    '€', ' ', '‚', 'ƒ', '„', '…', '†', '‡', // 0x80
    'ˆ', '‰', 'Š', '‹', 'Œ', ' ', 'Ž', ' ', // 0x88
    ' ', '‘', '’', '“', '”', '•', '–', '—', // 0x90
    '˜', '™', 'š', '›', 'œ', ' ', 'ž', 'Ÿ', // 0x98
    ' ', '¡', '¢', '£', '¤', '¥', '¦', '§', // 0xA0
    '¨', '©', 'ª', '«', '¬', ' ', '®', '¯', // 0xA8
    '°', '±', '²', '³', '´', 'µ', '¶', '·', // 0xB0
    '¸', '¹', 'º', '»', '¼', '½', '¾', '¿', // 0xB8
    'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', // 0xC0
    'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', // 0xC8
    'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', // 0xD0
    'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', // 0xD8
    'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', // 0xE0
    'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', // 0xE8
    'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', // 0xF0
    'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ', // 0xF8
];

// ASCII
static ASCII: [char; 256] = [
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x00
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x08
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x10
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x18
    ' ', '!', '"', '#', '$', '%', '&', '\'', // 0x20
    '(', ')', '*', '+', ',', '-', '.', '/', // 0x28
    '0', '1', '2', '3', '4', '5', '6', '7', // 0x30
    '8', '9', ':', ';', '<', '=', '>', '?', // 0x38
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', // 0x40
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 0x48
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', // 0x50
    'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 0x58
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', // 0x60
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 0x68
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', // 0x70
    'x', 'y', 'z', '{', '|', '}', '~', ' ', // 0x78
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x80
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x88
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x90
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0x98
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xA0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xA8
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xB0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xB8
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xC0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xC8
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xD0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xD8
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xE0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xE8
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xF0
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 0xF8
];

/// A 256-entry mapping from byte values to display characters.
///
/// Used by [`super::BufferView`] to render the character panel for non-UTF-8 data.
#[derive(Clone, Copy)]
pub struct Codepage {
    pub(super) map: [char; 256],
    pub(super) name: FlatString<22>,
}
impl Codepage {
    /// IBM PC code page 437.
    pub const CP437: Self = Self::from_parts(&CP437, "CP437");
    /// Windows-1252 (Western European) code page.
    pub const WINDOWS_1252: Self = Self::from_parts(&WINDOWS_1252, "WINDOWS_1252");
    /// 7-bit ASCII; control characters and bytes above `0x7F` are shown as spaces.
    pub const ASCII: Self = Self::from_parts(&ASCII, "ASCII");
    const fn from_parts(map: &'static [char; 256], name: &'static str) -> Self {
        Self {
            map: *map,
            name: FlatString::from_str(name),
        }
    }
    /// Creates a custom code page initialized with printable ASCII (`0x20`..`0x7F`)
    /// and `'?'` for all other entries.
    pub fn new(name: &str) -> Self {
        let mut me = Self {
            map: ['?'; 256],
            name: FlatString::from_str(name),
        };
        for i in 32..127u8 {
            me.map[i as usize] = i as char;
        }
        me
    }
    /// Returns the display name of this code page.
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// Returns the full byte-to-character translation table.
    #[inline(always)]
    pub fn map(&self) -> &[char; 256] {
        &self.map
    }
    /// Replaces the entire translation table.
    #[inline(always)]
    pub fn set_map(&mut self, map: [char; 256]) {
        self.map = map;
    }
    /// Sets the display name of this code page.
    #[inline(always)]
    pub fn set_name(&mut self, name: &str) {
        self.name = FlatString::from_str(name);
    }
    /// Maps a single byte value to a display character.
    #[inline(always)]
    pub fn set(&mut self, index: u8, ch: char) {
        self.map[index as usize] = ch;
    }
    /// Returns the display character for a byte value.
    #[inline(always)]
    pub fn get(&self, index: u8) -> char {
        self.map[index as usize]
    }
    /// Sets every entry in the translation table to the same character.
    pub fn fill(&mut self, ch: char) {
        for i in 0..256 {
            self.map[i] = ch;
        }
    }
}

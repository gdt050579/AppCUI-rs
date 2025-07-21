use crate::utils::Caption;

use super::CharAttribute;
use EnumBitFlags::EnumBitFlags;

/// Represents the alignment of the text. It can be Left, Center or Right.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Default, Eq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}


/// Represents the wrap type of the text. It can be WordWrap, CharacterWrap, MultiLine, SingleLine or SingleLineWrap.
/// - `WordWrap`: The text will be wrapped to the next line if it exceeds the width.
/// - `CharacterWrap`: The text will be wrapped to the next line if it exceeds the width, but it will not break words.
/// - `MultiLine`: The text will be wrapped to the next line if a line break is encountered.
/// - `SingleLine`: The text will not be wrapped (line break is ignored). This is the default value.
/// - `SingleLineWrap`: The text will be clipped to a specific width.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum WrapType {
    WordWrap(u16),
    CharacterWrap(u16),
    MultiLine,
    #[default]
    SingleLine,
    SingleLineWrap(u16),
}

#[EnumBitFlags(bits = 8)]
enum TextFormatFlags {
    Hotkey = 0x01,
    CharsCount = 0x02,
}
//  X,Y, char_attr, align are left pub(crate) to allow direct access to those filed within the crate
// only members that don't have an associated flag are public(crate)

/// A structure that contains information about how a text should be displayed on the screen.
pub struct TextFormat {
    flags: TextFormatFlags,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) char_attr: CharAttribute,
    pub(super) hotkey_attr: CharAttribute,
    pub(super) hotkey_pos: u32,
    pub(super) chars_count: u16,
    pub(crate) align: TextAlignment,
    pub(crate) wrap_type: WrapType,
}

impl TextFormat {
    #[inline(always)]
    pub(super) fn has_hotkey(&self) -> bool {
        self.flags.contains(TextFormatFlags::Hotkey)
    }
    #[inline(always)]
    pub(super) fn has_chars_count(&self) -> bool {
        self.flags.contains(TextFormatFlags::CharsCount)
    }
    
    /// Sets the alignment of the text. It can be Left, Center or Right.
    #[inline(always)]
    pub fn set_align(&mut self, align: TextAlignment) {
        self.align = align;
    }

    /// Sets the attribute of the text (foreground , background color and flags such as Bold, Italic, etc).
    #[inline(always)]
    pub fn set_attribute(&mut self, attr: CharAttribute) {
        self.char_attr = attr;
    }

    /// Sets the position of the text on the screen.
    #[inline(always)]
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Sets the hotkey attribute and position (index) of the hotkey in the text buffer.
    #[inline(always)]
    pub fn set_hotkey(&mut self, attr: CharAttribute, pos: u32) {
        self.hotkey_attr = attr;
        self.hotkey_pos = pos;
        self.flags.set(TextFormatFlags::Hotkey);
    }

    /// Clears the hotkey attribute and position.
    #[inline(always)]
    pub fn clear_hotkey(&mut self) {
        self.flags.remove(TextFormatFlags::Hotkey);
    }

    /// Sets the wrap mode of the text. It can be `Character`` or `Word``. If the wrap mode is set, the width of the text should be set as well.
    /// If the wrap mode is set, the text will be wrapped to the next line if it exceeds the width.
    /// Setting the wrap mode implies that the text is multi-line ('\n' characters are interpreted as new lines).   
    #[inline(always)]
    pub fn set_wrap_type(&mut self, wrap_type: WrapType) {
        self.wrap_type = wrap_type;
    }

    /// Sets the number of characters in the text buffer. This is usefull to perform some optimizations in particular for unicode characters.
    #[inline(always)]
    pub fn set_chars_count(&mut self, value: u16) {
        self.chars_count = value;
        self.flags.set(TextFormatFlags::CharsCount);
    }

    // inner methods
    #[inline(always)]
    pub(crate) fn width(&self) -> u16 {
        match self.wrap_type {
            WrapType::WordWrap(width) | WrapType::CharacterWrap(width) | WrapType::SingleLineWrap(width) => width,
            _ => 0,
        }
    }
    #[inline(always)]
    pub(crate) fn set_hotkey_from_caption(&mut self, attr: CharAttribute, caption: &Caption) {
        if caption.has_hotkey() {
            self.set_hotkey(attr, caption.hotkey_pos().unwrap() as u32);
        } else {
            self.clear_hotkey();
        }
    }
}

impl Default for TextFormat {
    /// Creates a new instance of the TextFormat with default values.
    /// The default values are:
    /// - `flags`: None
    /// - `x`: 0
    /// - `y`: 0
    /// - `char_attr`: Default::default()
    /// - `hotkey_attr`: Default::default()
    /// - `hotkey_pos`: 0
    /// - `chars_count`: 0
    /// - `align`: TextAlignment::Left
    /// - `wrap_type`: WrapType::SingleLine 
    fn default() -> Self {
        Self {
            flags: TextFormatFlags::None,
            x: 0,
            y: 0,
            char_attr: Default::default(),
            hotkey_attr: Default::default(),
            hotkey_pos: 0,
            chars_count: 0,
            align: TextAlignment::Left,
            wrap_type: WrapType::SingleLine,
        }
    }
}

/// A builder for the [TextFormat] struct.
/// This builder allows you to create a TextFormat instance starting from the default values and modify it step by step.
/// 
/// # Example
/// ```rust
/// use appcui::prelude::*;
/// 
/// let format = TextFormatBuilder::new()
///                     .position(10, 20)
///                     .attribute(CharAttribute::with_color(Color::White, Color::Black))
///                     .align(TextAlignment::Center)
///                     .wrap_type(WrapType::WordWrap(50))
///                     .build();
/// ```
pub struct TextFormatBuilder {
    format: TextFormat,
}

impl TextFormatBuilder {
    /// Creates a new instance of the TextFormatBuilder.
    #[inline(always)]
    pub fn new() -> Self {
        Self { format: Default::default() }
    }

    /// Sets the position of the text on the screen.
    #[inline(always)]
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.format.x = x;
        self.format.y = y;
        self
    }

    /// Sets the attribute of the text.
    #[inline(always)]
    pub fn attribute(mut self, attr: CharAttribute) -> Self {
        self.format.char_attr = attr;
        self
    }

    /// Sets the hotkey attribute and position (index) of the hotkey in the text buffer.
    #[inline(always)]
    pub fn hotkey(mut self, attr: CharAttribute, pos: u32) -> Self {
        self.format.set_hotkey(attr, pos);
        self
    }

    /// Sets the alignment of the text. It can be Left, Center or Right.
    #[inline(always)]
    pub fn align(mut self, align: TextAlignment) -> Self {
        self.format.align = align;
        self
    }

    /// Sets the number of characters in the text buffer. This is usefull to perform some optimizations in particular for unicode characters.
    #[inline(always)]
    pub fn chars_count(mut self, value: u16) -> Self {
        self.format.set_chars_count(value);
        self
    }

    /// Sets the wrap mode of the text. It can be Character or Word. If the wrap mode is set, the width of the text should be set as well.
    /// If the wrap mode is set, the text will be wrapped to the next line if it exceeds the width.
    #[inline(always)]
    pub fn wrap_type(mut self, wrap_type: WrapType) -> Self {
        self.format.set_wrap_type(wrap_type);
        self
    }

    /// Builds the TextFormat instance.
    #[inline(always)]
    pub fn build(self) -> TextFormat {
        self.format
    }
}

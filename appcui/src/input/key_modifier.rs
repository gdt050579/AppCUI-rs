use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Modifier keys that can be combined with a [`KeyCode`](super::KeyCode) in a [`Key`](super::Key).
///
/// Combine values with `|`. `KeyModifier::None` means no Alt, Ctrl, or Shift.
/// Names are written with a trailing `+`, for example `Ctrl+Alt+`.
pub enum KeyModifier {
    /// The Alt key, shown as `Alt+` in key names.
    Alt = 0x01,
    /// The Control key, shown as `Ctrl+` in key names.
    Ctrl = 0x02,
    /// The Shift key, shown as `Shift+` in key names.
    Shift = 0x04,
}

static KEY_NAME: [&str; 8] = [
    /* 0 */ "",
    /* 1 */ "Alt+",
    /* 2 */ "Ctrl+",
    /* 3 */ "Ctrl+Alt+",
    /* 4 */ "Shift+",
    /* 5 */ "Alt+Shift+",
    /* 6 */ "Ctrl+Shift+",
    /* 7 */ "Ctrl+Alt+Shift+",
];

impl KeyModifier {
    /// Returns the name of the key modifier.
    /// 
    /// # Returns
    /// The name of the key modifier.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let modifier = KeyModifier::Alt;
    /// let name = modifier.name();
    /// ```
    pub fn name(&self) -> &'static str {
        if self.value < 8 {
            return KEY_NAME[self.value as usize];
        }
        ""
    }
    pub(crate) fn name_from_index(index: usize) -> &'static str {
        if index < 8 { KEY_NAME[index] } else { "" }
    }
}

impl From<u8> for KeyModifier {
    /// Creates a new key modifier from a u8.
    /// 
    /// # Arguments
    /// * `value` - The u8 to create the key modifier from.
    /// 
    /// # Returns
    /// A new key modifier created from the u8. If the u8 is not a valid key modifier, the function will return `KeyModifier::None`.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let modifier = KeyModifier::from(0x01);
    /// ```         
    fn from(value: u8) -> Self {
        if value < 8 {
            let mut result = KeyModifier::None;
            if (value & 1) != 0 {
                result |= KeyModifier::Alt;
            }
            if (value & 2) != 0 {
                result |= KeyModifier::Ctrl;
            }
            if (value & 4) != 0 {
                result |= KeyModifier::Shift;
            }
            return result;
        }
        KeyModifier::None
    }
}

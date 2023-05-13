use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum KeyModifier {
    Alt = 0x01,
    Ctrl = 0x02,
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
    pub fn get_name(&self) -> &'static str {
        if self.value < 8 {
            return KEY_NAME[self.value as usize];
        }
        return "";
    }
    pub(crate) fn get_name_from_index(index: usize) -> &'static str {
        return if index < 8 { KEY_NAME[index] } else { "" };
    }
}

impl From<u8> for KeyModifier {
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
        return KeyModifier::None;
    }
}

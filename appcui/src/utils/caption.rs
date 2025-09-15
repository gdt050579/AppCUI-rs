use crate::input::*;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum ExtractHotKeyMethod {
    NoHotKey,
    Key,
    AltPlusKey,
    CtrlPlusKey,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) struct Caption {
    text: String,
    chars_count: usize,
    hotkey_pos: usize,
    hotkey: Key,
}
impl Caption {
    pub(crate) fn new(text: &str, extract_hotkey_method: ExtractHotKeyMethod) -> Self {
        let mut s = Caption {
            text: String::with_capacity(text.len() + 16),
            chars_count: 0,
            hotkey_pos: 0,
            hotkey: Key::default(),
        };
        s.set_text(text, extract_hotkey_method);
        s
    }
    pub(crate) fn set_text(&mut self, text: &str, extract_hotkey_method: ExtractHotKeyMethod) {
        if (extract_hotkey_method != ExtractHotKeyMethod::NoHotKey) && (!text.is_empty()) {
            // search for &<char>
            let buf = text.as_bytes();
            let len = buf.len() - 1;
            let mut pos = 0;
            self.hotkey = Key::default();
            while pos < len {
                if buf[pos] == b'&' {
                    self.hotkey = Key::create_hotkey(buf[pos + 1] as char, KeyModifier::from(extract_hotkey_method));
                    if self.hotkey.code != KeyCode::None {
                        self.hotkey_pos = pos;
                        break;
                    }
                }
                pos += 1;
            }
            if self.hotkey.code != KeyCode::None {
                self.text.clear();
                self.text.push_str(&text[..pos]);
                self.text.push_str(&text[pos + 1..]);
            } else {
                self.text.clear();
                self.hotkey_pos = 0;
                self.text.push_str(text);
            }
        } else {
            self.text.clear();
            self.text.push_str(text);
            self.hotkey_pos = 0;
            self.hotkey = Key::default();
        }
        self.chars_count = self.text.chars().count();
    }
    #[inline(always)]
    pub(crate) fn text(&self) -> &str {
        self.text.as_str()
    }
    #[inline(always)]
    pub(crate) fn chars_count(&self) -> usize {
        self.chars_count
    }
    #[inline(always)]
    pub(crate) fn hotkey_pos(&self) -> Option<usize> {
        if self.hotkey.code == KeyCode::None {
            None
        } else {
            Some(self.hotkey_pos)
        }
    }
    #[inline(always)]
    pub(crate) fn hotkey(&self) -> Key {
        self.hotkey
    }
    #[inline(always)]
    pub(crate) fn has_hotkey(&self) -> bool {
        self.hotkey.code != KeyCode::None
    }
    // pub(crate) fn copy_from(&mut self, caption: &Caption) {
    //     self.chars_count = caption.chars_count;
    //     self.hotkey = caption.hotkey;
    //     self.hotkey_pos = caption.hotkey_pos;
    //     self.text.clear();
    //     self.text.push_str(caption.text.as_str());
    // }
    #[inline(always)]
    pub(crate) fn clear_hotkey_modifier(&mut self) {
        self.hotkey.modifier = KeyModifier::None;
    }
}

impl From<ExtractHotKeyMethod> for KeyModifier {
    fn from(value: ExtractHotKeyMethod) -> Self {
        match value {
            ExtractHotKeyMethod::NoHotKey => KeyModifier::None,
            ExtractHotKeyMethod::Key => KeyModifier::None,
            ExtractHotKeyMethod::AltPlusKey => KeyModifier::Alt,
            ExtractHotKeyMethod::CtrlPlusKey => KeyModifier::Ctrl,
        }
    }
}

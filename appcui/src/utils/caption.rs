use crate::input::*;

#[derive(Clone)]
pub(crate) struct Caption {
    text: String,
    chars_count: usize,
    hotkey_pos: usize,
    hotkey: Key,
}
impl Caption {
    pub(crate) fn new(text: &str, process_hotkey: bool) -> Self {
        let mut s = Caption {
            text: String::with_capacity(text.len() + 16),
            chars_count: 0,
            hotkey_pos: 0,
            hotkey: Key::default(),
        };
        s.set_text(text, process_hotkey);
        s
    }
    pub(crate) fn set_text(&mut self, text: &str, process_hotkey: bool) {
        if (process_hotkey) && (text.len() > 0) {
            // search for &<char>
            let buf = text.as_bytes();
            let len = buf.len() - 1;
            let mut pos = 0;
            self.hotkey = Key::default();
            while pos < len {
                if buf[pos] == b'&' {
                    self.hotkey = Key::from_char(buf[pos + 1] as char, KeyModifier::Alt);
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
    #[inline]
    pub(crate) fn get_text(&self) -> &str {
        self.text.as_str()
    }
    #[inline]
    pub(crate) fn get_chars_count(&self) -> usize {
        self.chars_count
    }
    #[inline]
    pub(crate) fn get_hotkey_pos(&self) -> Option<usize> {
        if self.hotkey.code == KeyCode::None {
            None
        } else {
            Some(self.hotkey_pos)
        }
    }
    #[inline]
    pub(crate) fn get_hotkey(&self) -> Key {
        self.hotkey
    }
    #[inline]
    pub(crate) fn has_hotkey(&self) -> bool {
        self.hotkey.code != KeyCode::None
    }
    pub(crate) fn copy_from(&mut self, caption: &Caption) {
        self.chars_count = caption.chars_count;
        self.hotkey = caption.hotkey;
        self.hotkey_pos = caption.hotkey_pos;
        self.text.clear();
        self.text.push_str(caption.text.as_str());
    }
}

impl Default for Caption {
    fn default() -> Self {
        Self {
            text: String::new(),
            chars_count: 0,
            hotkey_pos: 0,
            hotkey: Key::default(),
        }
    }
}

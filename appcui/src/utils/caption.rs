use crate::input::*;

pub (crate) struct Caption
{
    text: String,
    chars_count: u32,
    hotkey_pos: u32,
    hotkey: Key
}
impl Caption {
    fn new(text: &str, has_hotkey: bool)->Self {
        let mut s = Caption {
            text: String::with_capacity(text.len()),
            chars_count: 0,
            hotkey_pos: 0,
            hotkey: Key::default()
        };
        s
    }
    fn set_text(&mut self, text: &str, has_hotkey: bool) {

    }
    #[inline]
    fn get_text(&self)->&str {
        self.text.as_str()
    }
    #[inline]
    fn get_chars_count(&self)->u32 {
        self.chars_count
    }
    #[inline]
    fn get_hotkey_pos(&self)->Option<u32> {
        if self.hotkey.code == KeyCode::None {
            None
        } else {
            Some(self.hotkey_pos)
        }
    }
    #[inline]
    fn get_hotkey(&self)->Key {
        self.hotkey
    }
    #[inline]
    fn has_hotkey(&self)->bool {
        self.hotkey.code != KeyCode::None
    }
}
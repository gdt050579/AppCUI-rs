use super::KeyCode;
use super::KeyModifier;

#[derive(Copy,Clone,PartialEq,Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifier: KeyModifier,
    pub character: char,
}

impl Key {
    pub fn new(code: KeyCode, modifier:KeyModifier, character: char) -> Key {
        Key {
            code: code,
            modifier: modifier,
            character: character
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self { code: KeyCode::None, modifier: KeyModifier::None, character: 0 as char }
    }
}
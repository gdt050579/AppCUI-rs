use super::KeyCode;
use super::KeyModifier;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifier: KeyModifier,
}

impl Key {
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Key {
        Key {
            code: code,
            modifier: modifier,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            code: KeyCode::None,
            modifier: KeyModifier::None,
        }
    }
}

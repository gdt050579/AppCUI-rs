use std::fmt::Display;

use super::KeyCode;
use super::KeyModifier;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifier: KeyModifier,
}

impl Key {
    #[allow(non_upper_case_globals)]
    pub const None: Key = Key {
        code: KeyCode::None,
        modifier: KeyModifier::None,
    };
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Key {
        Key {
            code: code,
            modifier: modifier,
        }
    }
    pub fn create_hotkey(character: char, modifier: KeyModifier) -> Key {
        let code = match character {
            'a' | 'A' => KeyCode::A,
            'b' | 'B' => KeyCode::B,
            'c' | 'C' => KeyCode::C,
            'd' | 'D' => KeyCode::D,
            'e' | 'E' => KeyCode::E,
            'f' | 'F' => KeyCode::F,
            'g' | 'G' => KeyCode::G,
            'h' | 'H' => KeyCode::H,
            'i' | 'I' => KeyCode::I,
            'j' | 'J' => KeyCode::J,
            'k' | 'K' => KeyCode::K,
            'l' | 'L' => KeyCode::L,
            'm' | 'M' => KeyCode::M,
            'n' | 'N' => KeyCode::N,
            'o' | 'O' => KeyCode::O,
            'p' | 'P' => KeyCode::P,
            'q' | 'Q' => KeyCode::Q,
            'r' | 'R' => KeyCode::R,
            's' | 'S' => KeyCode::S,
            't' | 'T' => KeyCode::T,
            'u' | 'U' => KeyCode::U,
            'v' | 'V' => KeyCode::V,
            'w' | 'W' => KeyCode::W,
            'x' | 'X' => KeyCode::X,
            'y' | 'Y' => KeyCode::Y,
            'z' | 'Z' => KeyCode::Z,
            '0' => KeyCode::N0,
            '1' => KeyCode::N1,
            '2' => KeyCode::N2,
            '3' => KeyCode::N3,
            '4' => KeyCode::N4,
            '5' => KeyCode::N5,
            '6' => KeyCode::N6,
            '7' => KeyCode::N7,
            '8' => KeyCode::N8,
            '9' => KeyCode::N9,
            _ => KeyCode::None,
        };
        if code == KeyCode::None {
            return Key {
                code: KeyCode::None,
                modifier: KeyModifier::None,
            };
        } else {
            return Key { code, modifier };
        }
    }
    pub fn get_compact_code(&self) -> u16 {
        ((self.code as u8) as u16) | ((self.modifier.get_value() as u16) << 8)
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
impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        Self {
            code: value,
            modifier: KeyModifier::None,
        }
    }
}
impl From<u16> for Key {
    fn from(value: u16) -> Self {
        let k = (value & 0xFF) as u8;
        let m = (value >> 8) as u8;
        if (k >= 64) || (m >= 8) {
            Key::None
        } else {
            Self {
                code: k.into(),
                modifier: m.into(),
            }
        }
    }
}
impl From<char> for Key {
    fn from(value: char) -> Self {
        match value {
            // capital
            'A' => return Key::new(KeyCode::A, KeyModifier::None),
            'B' => return Key::new(KeyCode::B, KeyModifier::None),
            'C' => return Key::new(KeyCode::C, KeyModifier::None),
            'D' => return Key::new(KeyCode::D, KeyModifier::None),
            'E' => return Key::new(KeyCode::E, KeyModifier::None),
            'F' => return Key::new(KeyCode::F, KeyModifier::None),
            'G' => return Key::new(KeyCode::G, KeyModifier::None),
            'H' => return Key::new(KeyCode::H, KeyModifier::None),
            'I' => return Key::new(KeyCode::I, KeyModifier::None),
            'J' => return Key::new(KeyCode::J, KeyModifier::None),
            'K' => return Key::new(KeyCode::K, KeyModifier::None),
            'L' => return Key::new(KeyCode::L, KeyModifier::None),
            'M' => return Key::new(KeyCode::M, KeyModifier::None),
            'N' => return Key::new(KeyCode::N, KeyModifier::None),
            'O' => return Key::new(KeyCode::O, KeyModifier::None),
            'P' => return Key::new(KeyCode::P, KeyModifier::None),
            'Q' => return Key::new(KeyCode::Q, KeyModifier::None),
            'R' => return Key::new(KeyCode::R, KeyModifier::None),
            'S' => return Key::new(KeyCode::S, KeyModifier::None),
            'T' => return Key::new(KeyCode::T, KeyModifier::None),
            'U' => return Key::new(KeyCode::U, KeyModifier::None),
            'V' => return Key::new(KeyCode::V, KeyModifier::None),
            'W' => return Key::new(KeyCode::W, KeyModifier::None),
            'X' => return Key::new(KeyCode::X, KeyModifier::None),
            'Y' => return Key::new(KeyCode::Y, KeyModifier::None),
            'Z' => return Key::new(KeyCode::Z, KeyModifier::None),
            // lower case letters
            'a' => return Key::new(KeyCode::A, KeyModifier::Shift),
            'b' => return Key::new(KeyCode::B, KeyModifier::Shift),
            'c' => return Key::new(KeyCode::C, KeyModifier::Shift),
            'd' => return Key::new(KeyCode::D, KeyModifier::Shift),
            'e' => return Key::new(KeyCode::E, KeyModifier::Shift),
            'f' => return Key::new(KeyCode::F, KeyModifier::Shift),
            'g' => return Key::new(KeyCode::G, KeyModifier::Shift),
            'h' => return Key::new(KeyCode::H, KeyModifier::Shift),
            'i' => return Key::new(KeyCode::I, KeyModifier::Shift),
            'j' => return Key::new(KeyCode::J, KeyModifier::Shift),
            'k' => return Key::new(KeyCode::K, KeyModifier::Shift),
            'l' => return Key::new(KeyCode::L, KeyModifier::Shift),
            'm' => return Key::new(KeyCode::M, KeyModifier::Shift),
            'n' => return Key::new(KeyCode::N, KeyModifier::Shift),
            'o' => return Key::new(KeyCode::O, KeyModifier::Shift),
            'p' => return Key::new(KeyCode::P, KeyModifier::Shift),
            'q' => return Key::new(KeyCode::Q, KeyModifier::Shift),
            'r' => return Key::new(KeyCode::R, KeyModifier::Shift),
            's' => return Key::new(KeyCode::S, KeyModifier::Shift),
            't' => return Key::new(KeyCode::T, KeyModifier::Shift),
            'u' => return Key::new(KeyCode::U, KeyModifier::Shift),
            'v' => return Key::new(KeyCode::V, KeyModifier::Shift),
            'w' => return Key::new(KeyCode::W, KeyModifier::Shift),
            'x' => return Key::new(KeyCode::X, KeyModifier::Shift),
            'y' => return Key::new(KeyCode::Y, KeyModifier::Shift),
            'z' => return Key::new(KeyCode::Z, KeyModifier::Shift),

            // numbers
            '0' => return Key::new(KeyCode::N0, KeyModifier::None),
            '1' => return Key::new(KeyCode::N1, KeyModifier::None),
            '2' => return Key::new(KeyCode::N2, KeyModifier::None),
            '3' => return Key::new(KeyCode::N3, KeyModifier::None),
            '4' => return Key::new(KeyCode::N4, KeyModifier::None),
            '5' => return Key::new(KeyCode::N5, KeyModifier::None),
            '6' => return Key::new(KeyCode::N6, KeyModifier::None),
            '7' => return Key::new(KeyCode::N7, KeyModifier::None),
            '8' => return Key::new(KeyCode::N8, KeyModifier::None),
            '9' => return Key::new(KeyCode::N9, KeyModifier::None),

            // simbols over bnumbers
            ')' => return Key::new(KeyCode::N0, KeyModifier::Shift),
            '!' => return Key::new(KeyCode::N1, KeyModifier::Shift),
            '@' => return Key::new(KeyCode::N2, KeyModifier::Shift),
            '#' => return Key::new(KeyCode::N3, KeyModifier::Shift),
            '$' => return Key::new(KeyCode::N4, KeyModifier::Shift),
            '%' => return Key::new(KeyCode::N5, KeyModifier::Shift),
            '^' => return Key::new(KeyCode::N6, KeyModifier::Shift),
            '&' => return Key::new(KeyCode::N7, KeyModifier::Shift),
            '*' => return Key::new(KeyCode::N8, KeyModifier::Shift),
            '(' => return Key::new(KeyCode::N9, KeyModifier::Shift),

            // other combinations
            ' ' => return Key::new(KeyCode::Space, KeyModifier::None),
            '\n' => return Key::new(KeyCode::Enter, KeyModifier::None),
            '\t' => return Key::new(KeyCode::Tab, KeyModifier::None),

            // default
            _ => { return Key::None; }
        };        
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.modifier.name(), self.code.name())
    }
}

use proc_macro::*;
use std::str::FromStr;

use crate::token_stream_to_string::TokenStreamToString;

use super::{KeyCode, KeyModifier};

struct Key {
    alt: bool,
    ctrl: bool,
    shift: bool,
    code: KeyCode,
}
impl Key {
    fn new(string: &str) -> Key {
        let mut k = Key {
            alt: false,
            ctrl: false,
            shift: false,
            code: KeyCode::None,
        };
        for text in string.split('+') {
            let hash = crate::utils::compute_hash(text);
            let code = KeyCode::from_hash(hash);
            if let Some(c) = code {
                if c == KeyCode::None {
                    return Key {
                        alt: false,
                        ctrl: false,
                        shift: false,
                        code: KeyCode::None,
                    };
                }
                if k.code != KeyCode::None {
                    panic!(
                        "You can not set two keys at the same time: '{}' and '{}'",
                        c.get_name(),
                        k.code.get_name()
                    );
                }
                k.code = c;
                continue;
            }
            let modif = KeyModifier::from_hash(hash);
            if let Some(m) = modif {
                match m {
                    KeyModifier::None => { /* do nothing */ }
                    KeyModifier::Alt => {
                        if k.alt {
                            panic!("You have already added `Alt` as a modifier !");
                        }
                        k.alt = true;
                    }
                    KeyModifier::Shift => {
                        if k.shift {
                            panic!("You have already added `Shift` as a modifier !");
                        }
                        k.shift = true;
                    }
                    KeyModifier::Ctrl => {
                        if k.ctrl {
                            panic!("You have already added `Ctrl` as a modifier !");
                        }
                        k.ctrl = true;
                    }
                }
            } else {
                panic!("Unknwon key or modifiert: '{}'", text);
            }
        }
        k
    }
    fn has_modifier(&self) -> bool {
        self.alt | self.shift | self.ctrl
    }
}

pub(crate) fn create_string(string: &str) -> String {
    let key = Key::new(string);
    let mut s = String::from("Key::new(KeyCode::");
    s.push_str(key.code.get_name());
    s.push_str(", ");
    if key.has_modifier() {
        if key.alt {
            s.push_str("KeyModifier::Alt|");
        }
        if key.shift {
            s.push_str("KeyModifier::Shift|");
        }
        if key.ctrl {
            s.push_str("KeyModifier::Ctrl|");
        }
        // remove last character
        s.pop();
        s.push(')');
    } else {
        s.push_str("KeyModifier::None )");
    }
    s
}
pub(crate) fn create_u16(string: &str) -> u16 {
    let key = Key::new(string);
    let mut value: u16 = 0;
    if key.alt {
        value |= 0x100;
    }
    if key.ctrl {
        value |= 0x200;
    }
    if key.shift {
        value |= 0x400;
    }
    value |= (key.code as u8) as u16;
    value
}
pub fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("key");
    let value = create_u16(&s);
    let mut string_repr = value.to_string();
    string_repr.push_str("u16");
    TokenStream::from_str(&string_repr).expect("Fail to convert key to token stream")
}

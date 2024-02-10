use proc_macro::*;
use std::str::FromStr;

use crate::token_stream_to_string::TokenStreamToString;

pub fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("key");
    let value = parse_string_key_representation(&s);
    let mut string_repr = value.to_string();
    string_repr.push_str("u16");
    TokenStream::from_str(&string_repr).expect("Fail to convert key to token stream")
}

pub(crate) fn parse_string_key_representation(string: &str) -> u16 {
    let mut key_value = 0u16;
    let mut modifiers = 0u16;

    for key in string.split('+') {
        let modifier = match key {
            "Ctrl" => 0x200,
            "Alt" => 0x100,
            "Shift" => 0x400,
            _ => 0,
        };

        let key_code = if modifier != 0 { 0 } else { parse_key_name(key) };
        if (modifier == 0) && (key_code == 0) {
            panic!("Unknown key or modifier: {}", key);
        }
        if ((modifiers & modifier) != 0) && (modifier != 0) {
            panic!("Duplicate modifier: {}", key);
        }
        if (key_value != 0) && (key_code != 0) {
            panic!("A key can only be added once: {}", key);
        }
        modifiers |= modifier;
        key_value = key_code;
    }
    if (modifiers == 0) && (key_value == 0) {
        panic!("Invalid key combination: {}", string);
    }
    modifiers | key_value
}

fn parse_key_name(key: &str) -> u16 {
    match key {
        "F1" => 1,
        "F2" => 2,
        "F3" => 3,
        "F4" => 4,
        "F5" => 5,
        "F6" => 6,
        "F7" => 7,
        "F8" => 8,
        "F9" => 9,
        "F10" => 10,
        "F11" => 11,
        "F12" => 12,
        "Enter" => 13,
        "Escape" => 14,
        "Insert" => 15,
        "Delete" => 16,
        "Backspace" => 17,
        "Tab" => 18,
        "Left" => 19,
        "Up" => 20,
        "Down" => 21,
        "Right" => 22,
        "PageUp" => 23,
        "PageDown" => 24,
        "Home" => 25,
        "End" => 26,
        "Space" => 27,
        "A" => 28,
        "B" => 29,
        "C" => 30,
        "D" => 31,
        "E" => 32,
        "F" => 33,
        "G" => 34,
        "H" => 35,
        "I" => 36,
        "J" => 37,
        "K" => 38,
        "L" => 39,
        "M" => 40,
        "N" => 41,
        "O" => 42,
        "P" => 43,
        "Q" => 44,
        "R" => 45,
        "S" => 46,
        "T" => 47,
        "U" => 48,
        "V" => 49,
        "W" => 50,
        "X" => 51,
        "Y" => 52,
        "Z" => 53,
        "0" => 54,
        "1" => 55,
        "2" => 56,
        "3" => 57,
        "4" => 58,
        "5" => 59,
        "6" => 60,
        "7" => 61,
        "8" => 62,
        "9" => 63,
        _ => 0,
    }
}
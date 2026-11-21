use super::io::{add_alt, decode_csi, decode_plain_key, decode_sgr_mouse, AnsiKeyCode, Letter, MouseButtonEvent};
use super::TermiosError;
use crate::input::{KeyModifier, MouseButton};

fn decode(payload: &str) -> Result<AnsiKeyCode, TermiosError> {
    decode_sgr_mouse(payload.as_bytes()).map(|key| key.code())
}

fn mouse(button: MouseButton, x: i32, y: i32) -> MouseButtonEvent {
    MouseButtonEvent { button, x, y }
}

#[test]
fn sgr_left_press_is_zero_based() {
    assert_eq!(decode("0;1;1M").unwrap(), AnsiKeyCode::MouseButton(mouse(MouseButton::Left, 0, 0)));
}

#[test]
fn sgr_release_is_button_up() {
    assert_eq!(decode("0;80;24m").unwrap(), AnsiKeyCode::MouseButton(mouse(MouseButton::None, 79, 23)));
}

#[test]
fn sgr_drag_and_hover_keep_the_cell() {
    assert_eq!(decode("32;15;7M").unwrap(), AnsiKeyCode::MouseMove(mouse(MouseButton::Left, 14, 6)));
    assert_eq!(decode("35;15;7M").unwrap(), AnsiKeyCode::MouseMove(mouse(MouseButton::None, 14, 6)));
    // Windows Terminal reports the same hover with a trailing `m`.
    assert_eq!(decode("35;15;7m").unwrap(), AnsiKeyCode::MouseMove(mouse(MouseButton::None, 14, 6)));
}

#[test]
fn sgr_middle_and_right_follow_xterm_numbering() {
    assert_eq!(decode("1;3;4M").unwrap(), AnsiKeyCode::MouseButton(mouse(MouseButton::Center, 2, 3)));
    assert_eq!(decode("2;3;4M").unwrap(), AnsiKeyCode::MouseButton(mouse(MouseButton::Right, 2, 3)));
}

#[test]
fn sgr_position_is_not_limited_to_one_byte() {
    assert_eq!(
        decode("0;400;250M").unwrap(),
        AnsiKeyCode::MouseButton(mouse(MouseButton::Left, 399, 249))
    );
}

#[test]
fn sgr_modifiers_stay_on_the_key() {
    let key = decode_sgr_mouse(b"28;2;3M").unwrap();
    assert_eq!(key.code(), AnsiKeyCode::MouseButton(mouse(MouseButton::Left, 1, 2)));
    assert_eq!(key.modifier(), KeyModifier::Ctrl | KeyModifier::Alt | KeyModifier::Shift);
}

#[test]
fn sgr_wheel_and_truncated_reports_are_rejected() {
    assert!(decode("64;1;1M").is_err());
    assert!(decode("0;1M").is_err());
    assert!(decode("0;1;1;2M").is_err());
}

fn csi(payload: &str) -> (AnsiKeyCode, KeyModifier) {
    let key = decode_csi(payload.as_bytes()).unwrap();
    (key.code(), key.modifier())
}

#[test]
fn csi_arrows_carry_xterm_modifiers() {
    assert_eq!(csi("A"), (AnsiKeyCode::_Up, KeyModifier::None));
    assert_eq!(csi("1;2A"), (AnsiKeyCode::_Up, KeyModifier::Shift));
    assert_eq!(csi("1;3B"), (AnsiKeyCode::_Down, KeyModifier::Alt));
    assert_eq!(csi("1;5C"), (AnsiKeyCode::_Right, KeyModifier::Ctrl));
    assert_eq!(csi("1;6D"), (AnsiKeyCode::_Left, KeyModifier::Ctrl | KeyModifier::Shift));
    assert_eq!(csi("1;8A"), (AnsiKeyCode::_Up, KeyModifier::Ctrl | KeyModifier::Alt | KeyModifier::Shift));
}

#[test]
fn csi_paging_home_end_insert_delete() {
    assert_eq!(csi("5~"), (AnsiKeyCode::_PageUp, KeyModifier::None));
    assert_eq!(csi("5;2~"), (AnsiKeyCode::_PageUp, KeyModifier::Shift));
    assert_eq!(csi("6;5~"), (AnsiKeyCode::_PageDown, KeyModifier::Ctrl));
    assert_eq!(csi("H"), (AnsiKeyCode::_Home, KeyModifier::None));
    assert_eq!(csi("1;2H"), (AnsiKeyCode::_Home, KeyModifier::Shift));
    assert_eq!(csi("1~"), (AnsiKeyCode::_Home, KeyModifier::None));
    assert_eq!(csi("F"), (AnsiKeyCode::_End, KeyModifier::None));
    assert_eq!(csi("4~"), (AnsiKeyCode::_End, KeyModifier::None));
    assert_eq!(csi("4;3~"), (AnsiKeyCode::_End, KeyModifier::Alt));
    assert_eq!(csi("2~"), (AnsiKeyCode::_Insert, KeyModifier::None));
    assert_eq!(csi("2;2~"), (AnsiKeyCode::_Insert, KeyModifier::Shift));
    assert_eq!(csi("3~"), (AnsiKeyCode::Delete, KeyModifier::None));
    assert_eq!(csi("3;5~"), (AnsiKeyCode::Delete, KeyModifier::Ctrl));
}

#[test]
fn csi_function_keys_and_shifted_tab() {
    assert_eq!(csi("15~"), (AnsiKeyCode::F5, KeyModifier::None));
    assert_eq!(csi("15;2~"), (AnsiKeyCode::F5, KeyModifier::Shift));
    assert_eq!(csi("23~"), (AnsiKeyCode::_F11, KeyModifier::None));
    assert_eq!(csi("24;6~"), (AnsiKeyCode::F12, KeyModifier::Ctrl | KeyModifier::Shift));
    assert_eq!(csi("1;2P"), (AnsiKeyCode::F1, KeyModifier::Shift));
    assert_eq!(csi("1;5Q"), (AnsiKeyCode::F2, KeyModifier::Ctrl));
    assert_eq!(csi("Z"), (AnsiKeyCode::_Tab, KeyModifier::Shift));
    assert_eq!(csi("1;5Z"), (AnsiKeyCode::_Tab, KeyModifier::Ctrl | KeyModifier::Shift));
}

#[test]
fn escape_prefix_adds_alt_to_a_plain_key() {
    let alt_a = add_alt(decode_plain_key(b'a').unwrap());
    assert_eq!(alt_a.code(), AnsiKeyCode::Letter(Letter::A));
    assert_eq!(alt_a.modifier(), KeyModifier::Alt);

    let ctrl_alt_a = add_alt(decode_plain_key(1).unwrap());
    assert_eq!(ctrl_alt_a.code(), AnsiKeyCode::Letter(Letter::A));
    assert_eq!(ctrl_alt_a.modifier(), KeyModifier::Ctrl | KeyModifier::Alt);

    let alt_shift_a = add_alt(decode_plain_key(b'A').unwrap());
    assert_eq!(alt_shift_a.code(), AnsiKeyCode::Letter(Letter::A));
    assert_eq!(alt_shift_a.modifier(), KeyModifier::Alt | KeyModifier::Shift);

    assert_eq!(add_alt(decode_plain_key(13).unwrap()).code(), AnsiKeyCode::_Enter);
    assert_eq!(add_alt(decode_plain_key(13).unwrap()).modifier(), KeyModifier::Alt);
    assert_eq!(add_alt(decode_plain_key(127).unwrap()).code(), AnsiKeyCode::_Backspace);
    assert_eq!(add_alt(decode_plain_key(127).unwrap()).modifier(), KeyModifier::Alt);
}

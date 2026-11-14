use super::io::{decode_sgr_mouse, AnsiKeyCode, MouseButtonEvent};
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

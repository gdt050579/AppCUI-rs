use crate::input::MouseButton;

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseHoldCommand {
    x: i32,
    y: i32,
    button: MouseButton
}

impl MouseHoldCommand {
    fn new(parser: &CommandParser)->Result<Self,ParserError> {
        if parser.get_params_count()!=3 {
            return Err(ParserError::new("Mouse.Hold command requires 3 parameters"));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        let b = parser.get_mouse_button(2);
        if x.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Hold command should an integer (x value)"));
        }
        if y.is_none() {
            return Err(ParserError::new("Seconf parameter for Mouse.Hold command should an integer (y value)"));
        }
        if b.is_none() {
            return Err(ParserError::new("Third parameter for Mouse.Hold command should an mouse button (left, right or cente)"));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
            button: b.unwrap()
        })
    }
}
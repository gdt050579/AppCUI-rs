use crate::input::MouseButton;

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseReleaseCommand {
    x: i32,
    y: i32,
}

impl MouseReleaseCommand {
    pub(super) fn new(parser: &CommandParser)->Result<Self,ParserError> {
        if parser.get_params_count()!=2 {
            return Err(ParserError::new("Mouse.Release command requires 2 parameters"));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        if x.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Release command should an integer (x value)"));
        }
        if y.is_none() {
            return Err(ParserError::new("Second parameter for Mouse.Release command should an integer (y value)"));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}
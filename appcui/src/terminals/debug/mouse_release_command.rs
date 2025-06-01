use std::collections::VecDeque;

use crate::{
    graphics::Point,
    input::MouseButton,
    terminals::{MouseButtonUpEvent, MouseMoveEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseReleaseCommand {
    x: i32,
    y: i32,
    button: MouseButton,
}

impl MouseReleaseCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 3 {
            return Err(ParserError::new("Mouse.Release command requires 3 parameters"));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        let b = parser.get_mouse_button(2);
        if x.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Release command should an integer (x value)"));
        }
        if y.is_none() {
            return Err(ParserError::new("Second parameter for Mouse.Release command should an integer (y value)"));
        }
        if b.is_none() {
            return Err(ParserError::new(
                "Third parameter for Mouse.Hold command should an mouse button (left, right or center)",
            ));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
            button: b.unwrap(),
        })
    }
    pub(super) fn generate_event(&self, mouse_pos: Point, sys_events: &mut VecDeque<SystemEvent>) {
        //GDT: not sure it is completely correct --> the actual state is that a mouse button is clicked
        if (mouse_pos.x != self.x) || (mouse_pos.y != self.y) {
            sys_events.push_back(SystemEvent::MouseMove(MouseMoveEvent {
                x: self.x,
                y: self.y,
                button: self.button,
            }));
        }
        sys_events.push_back(SystemEvent::MouseButtonUp(MouseButtonUpEvent {
            x: self.x,
            y: self.y,
            button: MouseButton::None,
        }));
    }
}

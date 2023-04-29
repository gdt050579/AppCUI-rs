use std::collections::VecDeque;

use crate::{
    input::{MouseButton, MouseWheelDirection},
    terminals::{MouseWheelEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseWheelCommand {
    x: i32,
    y: i32,
    direction: MouseWheelDirection,
    times: u32,
}

impl MouseWheelCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 4 {
            return Err(ParserError::new(
                "Mouse.Wheel command requires 4 parameters",
            ));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        let d = parser.get_mouse_wheel(2);
        let t = parser.get_i32(3);
        if x.is_none() {
            return Err(ParserError::new(
                "First parameter for Mouse.Wheel command should an integer (x value)",
            ));
        }
        if y.is_none() {
            return Err(ParserError::new(
                "Second parameter for Mouse.Wheel command should an integer (y value)",
            ));
        }
        if d.is_none() {
            return Err(ParserError::new("Third parameter for Mouse.Wheel command should a direction (one of left,right,up,down)"));
        }
        if t.is_none() {
            return Err(ParserError::new("Fourth parameter for Mouse.Wheel should be a positive number (bigger than 0) - number of times"));
        }
        if t.unwrap() < 1 {
            return Err(ParserError::new("Fourth parameter for Mouse.Wheel should be a positive number (bigger than 0) - number of times"));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
            direction: d.unwrap(),
            times: t.unwrap() as u32,
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        for _ in 0..self.times {
            sys_events.push_back(SystemEvent::MouseWheel(MouseWheelEvent {
                x: self.x,
                y: self.y,
                direction: self.direction,
            }));
        }
    }
}

use std::collections::VecDeque;

use crate::terminals::SystemEvent;

use super::{
    command_parser::{CommandParser, ParserError},
    mouse_click_command::MouseClickCommand,
    mouse_drag_command::MouseDragCommand,
    mouse_hold_command::MouseHoldCommand,
    mouse_move_command::MouseMoveCommand,
    mouse_release_command::MouseReleaseCommand,
    mouse_wheel_command::MouseWheelCommand,
    paint_command::PaintCommand,
};

pub(super) enum Command {
    MouseHold(MouseHoldCommand),
    MouseRelease(MouseReleaseCommand),
    MouseClick(MouseClickCommand),
    MouseMove(MouseMoveCommand),
    MouseDrag(MouseDragCommand),
    MouseWheel(MouseWheelCommand),
    Paint(PaintCommand),
}
impl Command {
    pub(super) fn new(text: &str) -> Result<Command, ParserError> {
        let cp = CommandParser::new(text)?;
        match cp.get_command() {
            "Mouse.Hold" => {
                let variant = MouseHoldCommand::new(&cp)?;
                return Ok(Command::MouseHold(variant));
            }
            "Mouse.Release" => {
                let variant = MouseReleaseCommand::new(&cp)?;
                return Ok(Command::MouseRelease(variant));
            }
            "Mouse.Click" => {
                let variant = MouseClickCommand::new(&cp)?;
                return Ok(Command::MouseClick(variant));
            }
            "Mouse.Move" => {
                let variant = MouseMoveCommand::new(&cp)?;
                return Ok(Command::MouseMove(variant));
            }
            "Mouse.Drag" => {
                let variant = MouseDragCommand::new(&cp)?;
                return Ok(Command::MouseDrag(variant));
            }
            "Mouse.Wheel" => {
                let variant = MouseWheelCommand::new(&cp)?;
                return Ok(Command::MouseWheel(variant));
            }
            "Paint" => {
                let variant = PaintCommand::new(&cp)?;
                return Ok(Command::Paint(variant));
            }
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                return Err(ParserError::new(&s));
            }
        }
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        match self {
            Command::MouseHold(cmd) => cmd.generate_event(sys_events),
            Command::MouseRelease(cmd) => cmd.generate_event(sys_events),
            Command::MouseClick(cmd) => cmd.generate_event(sys_events),
            Command::MouseMove(cmd) => cmd.generate_event(sys_events),
            Command::MouseDrag(cmd) => cmd.generate_event(sys_events),
            Command::MouseWheel(cmd) => cmd.generate_event(sys_events),
            Command::Paint(_) => {}
        }
    }
    pub(super) fn get_paint_command_title(&self) -> Option<String> {
        match self {
            Command::Paint(cmd) => Some(String::from(cmd.get_title())),
            _ => None,
        }
    }
}

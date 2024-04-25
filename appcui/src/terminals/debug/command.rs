use std::collections::VecDeque;

use crate::graphics::Point;
use crate::terminals::SystemEvent;

use super::{
    check_cursor_command::CheckCursorCommand,
    check_hash_command::CheckHashCommand,
    command_parser::{CommandParser, ParserError},
    keypress_command::KeyPressedCommand,
    keytypetext_command::KeyTypeTextCommand,
    mouse_click_command::MouseClickCommand,
    mouse_doubleclick_command::MouseDoubleClickCommand,
    mouse_drag_command::MouseDragCommand,
    mouse_hold_command::MouseHoldCommand,
    mouse_move_command::MouseMoveCommand,
    mouse_release_command::MouseReleaseCommand,
    mouse_wheel_command::MouseWheelCommand,
    paint_command::PaintCommand,
    paint_enable_command::PaintEnableCommand,
    error_disable_command::ErrorDisableCommand,
    resize_command::ResizeCommand,
};

pub(super) enum Command {
    MouseHold(MouseHoldCommand),
    MouseRelease(MouseReleaseCommand),
    MouseClick(MouseClickCommand),
    MouseDoubleClick(MouseDoubleClickCommand),
    MouseMove(MouseMoveCommand),
    MouseDrag(MouseDragCommand),
    MouseWheel(MouseWheelCommand),
    Paint(PaintCommand),
    PaintEnable(PaintEnableCommand),
    ErrorDisable(ErrorDisableCommand),
    CheckHash(CheckHashCommand),
    CheckCursor(CheckCursorCommand),
    Resize(ResizeCommand),
    KeyPresed(KeyPressedCommand),
    KeyTypeText(KeyTypeTextCommand),
}
impl Command {
    pub(super) fn new(text: &str) -> Result<Command, ParserError> {
        let cp = CommandParser::new(text)?;
        match cp.get_command() {
            "Mouse.Hold" => {
                let variant = MouseHoldCommand::new(&cp)?;
                Ok(Command::MouseHold(variant))
            }
            "Mouse.Release" => {
                let variant = MouseReleaseCommand::new(&cp)?;
                Ok(Command::MouseRelease(variant))
            }
            "Mouse.Click" => {
                let variant = MouseClickCommand::new(&cp)?;
                Ok(Command::MouseClick(variant))
            }
            "Mouse.DoubleClick" => {
                let variant = MouseDoubleClickCommand::new(&cp)?;
                Ok(Command::MouseDoubleClick(variant))
            }
            "Mouse.Move" => {
                let variant = MouseMoveCommand::new(&cp)?;
                Ok(Command::MouseMove(variant))
            }
            "Mouse.Drag" => {
                let variant = MouseDragCommand::new(&cp)?;
                Ok(Command::MouseDrag(variant))
            }
            "Mouse.Wheel" => {
                let variant = MouseWheelCommand::new(&cp)?;
                Ok(Command::MouseWheel(variant))
            }
            "Paint" => {
                let variant = PaintCommand::new(&cp)?;
                Ok(Command::Paint(variant))
            }
            "Paint.Enable" => {
                let variant = PaintEnableCommand::new(&cp)?;
                Ok(Command::PaintEnable(variant))
            }
            "CheckHash" => {
                let variant = CheckHashCommand::new(&cp)?;
                Ok(Command::CheckHash(variant))
            }
            "CheckCursor" => {
                let variant = CheckCursorCommand::new(&cp)?;
                Ok(Command::CheckCursor(variant))
            }
            "Error.Disable" => {
                let variant = ErrorDisableCommand::new(&cp)?;
                Ok(Command::ErrorDisable(variant))
            }
            "Resize" => {
                let variant = ResizeCommand::new(&cp)?;
                Ok(Command::Resize(variant))
            }
            "Key.Pressed" => {
                let variant = KeyPressedCommand::new(&cp)?;
                Ok(Command::KeyPresed(variant))
            }
            "Key.TypeText" => {
                let variant = KeyTypeTextCommand::new(&cp)?;
                Ok(Command::KeyTypeText(variant))
            }
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                Err(ParserError::new(&s))
            }
        }
    }
    pub(super) fn generate_event(&self, mouse_pos: Point, sys_events: &mut VecDeque<SystemEvent>) {
        match self {
            Command::MouseHold(cmd) => cmd.generate_event(mouse_pos, sys_events),
            Command::MouseRelease(cmd) => cmd.generate_event(mouse_pos, sys_events),
            Command::MouseClick(cmd) => cmd.generate_event(mouse_pos, sys_events),
            Command::MouseDoubleClick(cmd) => cmd.generate_event(mouse_pos, sys_events),
            Command::MouseMove(cmd) => cmd.generate_event(sys_events),
            Command::MouseDrag(cmd) => cmd.generate_event(sys_events),
            Command::MouseWheel(cmd) => cmd.generate_event(mouse_pos, sys_events),
            Command::Resize(cmd) => cmd.generate_event(sys_events),
            Command::KeyPresed(cmd) => cmd.generate_event(sys_events),
            Command::KeyTypeText(cmd) => cmd.generate_event(sys_events),
            Command::Paint(_) => {}
            Command::PaintEnable(_) => {}
            Command::ErrorDisable(_) => {}
            Command::CheckHash(_) => {}
            Command::CheckCursor(_) => {}
        }
    }
    pub(super) fn get_paint_command_title(&self) -> Option<String> {
        match self {
            Command::Paint(cmd) => Some(String::from(cmd.get_title())),
            _ => None,
        }
    }
    pub(super) fn get_screen_hash(&self) -> Option<u64> {
        match self {
            Command::CheckHash(cmd) => Some(cmd.get_hash()),
            _ => None,
        }
    }
    pub(super) fn get_cursor_pos(&self) -> Option<Point> {
        match self {
            Command::CheckCursor(cmd) => Some(cmd.get_point()),
            _ => None,
        }
    }
    pub(super) fn get_paint_enable_status(&self) -> Option<bool> {
        match self {
            Command::PaintEnable(cmd) => Some(cmd.is_paint_enabled()),
            _ => None,
        }
    }
    pub(super) fn get_error_disable_status(&self) -> Option<bool> {
        match self {
            Command::ErrorDisable(cmd) => Some(cmd.is_error_disabled()),
            _ => None,
        }
    }
}

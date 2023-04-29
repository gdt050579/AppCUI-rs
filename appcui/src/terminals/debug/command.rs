use super::{mouse_hold_command::MouseHoldCommand, command_parser::{CommandParser, ParserError}};

pub(super) enum Command {
    MouseHold(MouseHoldCommand)
}
impl Command {
    pub(super) fn new(text: &str) -> Result<Command,ParserError> {
        let cp = CommandParser::new(text)?;
        match cp.get_command() {
            "Mouse.Hold" => {
                let variant = MouseHoldCommand::new(&cp)?;
                return Ok(Command::MouseHold(variant));
            }
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                return Err(ParserError::new(&s));
            }
        }
    }    
}
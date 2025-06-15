use super::command_parser::{CommandParser, ParserError};

pub(super) struct PaintEnableCommand {
    enabled: bool,
}

impl PaintEnableCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 1 {
            return Err(ParserError::new("Paint.Enable command has parameter (a boolean value that enables or disables painted for the entire script) !"));
        }
        if let Some(value) = parser.get_bool(0) {
            return Ok(Self { enabled: value });
        }
        Err(ParserError::new("Expecting a boolean value as the first parameter for Paint.Enable command  !"))

    }
    pub(super) fn is_paint_enabled(&self) -> bool {
        self.enabled
    }
}

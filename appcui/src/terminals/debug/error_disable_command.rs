use super::command_parser::{CommandParser, ParserError};

pub(super) struct ErrorDisableCommand {
    disabled: bool,
}

impl ErrorDisableCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 1 {
            return Err(ParserError::new("Error.Disable command has parameter (a boolean value that enables or disables errors) !"));
        }
        if let Some(value) = parser.get_bool(0) {
            return Ok(Self { disabled: value });
        }
        Err(ParserError::new("Expecting a boolean value as the first parameter for Error.Disable command  !"))

    }
    pub(super) fn is_error_disabled(&self) -> bool {
        self.disabled
    }
}

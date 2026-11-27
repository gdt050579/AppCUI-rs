use super::command_parser::{CommandParser, ParserError};

pub(super) struct ForceRepaint;

impl ForceRepaint {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() > 0 {
            return Err(ParserError::new("ForceRepaint command has no parameters !"));
        }
        Ok(Self)
    }
}

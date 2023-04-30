use super::command_parser::{CommandParser, ParserError};

pub(super) struct CheckHashCommand {
    value: u64,
}

impl CheckHashCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 1 {
            return Err(ParserError::new("CheckHash command has one parameters !"));
        }
        if let Some(hash) = parser.get_hash(0) {
            return Ok(Self { value: hash });
        } else {
            return Err(ParserError::new(
                "CheckHash (invalid hash) --> use a hexadecimal format 0x....",
            ));
        }
    }
    pub(super) fn get_hash(&self) -> u64 {
        self.value
    }
}

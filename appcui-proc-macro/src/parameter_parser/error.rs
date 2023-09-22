use super::Token;

#[derive(Debug)]
pub(crate) struct Error {
    data: String,
    start: usize,
    end: usize,
    message: String
}
impl Error {
    pub(super) fn new(param_list: &str, message: &str, start: usize, end: usize) -> Self {
        Self {
            data: String::from(param_list),
            start,
            end,
            message: String::from(message)
        }
    }
    pub(super) fn with_token(param_list: &str, message: &str, tok: &Token)->Self {
        Self {
            data: String::from(param_list),
            start: tok.get_start(),
            end: tok.get_end(),
            message: String::from(message)
        }
    }
}
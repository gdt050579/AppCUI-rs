#[derive(Debug)]
pub(crate) struct Error {
    data: String,
    start: usize,
    end: usize,
    message: &'static str
}
impl Error {
    pub(super) fn new(param_list: &str, message: &'static str, start: usize, end: usize) -> Self {
        Self {
            data: String::from(param_list),
            start,
            end,
            message
        }
    }
}
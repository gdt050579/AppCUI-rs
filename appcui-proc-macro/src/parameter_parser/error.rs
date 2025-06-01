use std::fmt::Debug;

use super::Token;

pub(crate) struct Error {
    data: String,
    start: usize,
    end: usize,
    message: String,
}
impl Error {
    pub(crate) fn new(param_list: &str, message: &str, start: usize, end: usize) -> Self {
        Self {
            data: String::from(param_list),
            start,
            end,
            message: String::from(message),
        }
    }
    pub(super) fn with_token(param_list: &str, message: &str, tok: &Token) -> Self {
        Self {
            data: String::from(param_list),
            start: tok.get_start(),
            end: tok.get_end(),
            message: String::from(message),
        }
    }
    pub(super) fn get_description(&self) -> String {
        let mut s = String::with_capacity(512);
        s.push_str("\nError: ");
        s.push_str(&self.message);
        s.push_str("\nCode : ");
        // compute intervale
        // sanity check
        if (self.start < self.end) && (self.end <= self.data.len()) {
            let mut start = self.start;
            let mut end = self.end;
            let mut count_on_left = 0;
            let mut count_on_right = 0;
            let buf = self.data.as_bytes();
            while (start > 0) && (count_on_left < 20) && (buf[start] != b'\n') && (buf[start] != b'\r') {
                start -= 1;
                while (start > 0) && (buf[start] >= 128/* skip utf-8 */) {
                    start -= 1;
                }
                count_on_left += 1;
            }
            while (end < buf.len()) && (count_on_right < 20) && (buf[end] != b'\n') && (buf[end] != b'\r') {
                end += 1;
                while (end < buf.len()) && (buf[end] >= 128/* skip utf-8 */) {
                    end += 1;
                }
                count_on_right += 1;
            }
            s.push_str(&self.data[start..end]);
            s.push_str("\n     : ");
            for _ in 0..count_on_left {
                s.push(' ');
            }
            for _ in self.start..self.end {
                s.push('^');
            }
            s.push('\n');
        } else {
            s.push_str(
                format!(
                    "Internal error (invalid start/end parameter)\n - start={}\n = end={}\n - len={}\n - text='{}'\n ",
                    self.start,
                    self.end,
                    self.data.len(),
                    self.data
                )
                .as_str(),
            );
        }
        s
    }
    pub(crate) fn panic(self) {
        panic!("{}", self.get_description().as_str());
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_description().as_str())
    }
}

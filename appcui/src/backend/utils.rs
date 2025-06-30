mod ansi_formatter;
#[cfg(target_os = "windows")]
pub(crate) mod win32;
#[cfg(test)]
mod tests;

pub(crate) use ansi_formatter::AnsiFormatter;
pub(crate) use ansi_formatter::AnsiFlags;

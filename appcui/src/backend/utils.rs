mod ansi_formatter;
#[cfg(target_os = "windows")]
pub(crate) mod win32;

pub(crate) use ansi_formatter::AnsiFormatter;
pub(crate) use ansi_formatter::AnsiFlags;

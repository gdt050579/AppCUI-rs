mod ansi_formatter;
mod sixel_encoder;
#[cfg(test)]
mod tests;
#[cfg(target_family = "unix")]
pub(crate) mod unix;
#[cfg(target_os = "windows")]
pub(crate) mod win32;

pub(crate) use ansi_formatter::AnsiFlags;
pub(crate) use ansi_formatter::AnsiFormatter;
pub(crate) use sixel_encoder::SixelEncoder;

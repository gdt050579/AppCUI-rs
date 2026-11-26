pub(crate) mod api;
pub(crate) mod constants;
pub(crate) mod structs;
mod clipboard;
mod console;
mod console_restorer;
mod original_screen;

#[cfg(test)]
mod tests;


pub(crate)use clipboard::Clipboard;
pub(crate)use console::Console;
pub(crate)use console_restorer::ConsoleRestorer;
use original_screen::OriginalScreen;





//! A command bar UI control for displaying available commands and shortcuts.
//!
//! The CommandBar control provides a bar that shows available actions and their keyboard shortcuts.
//! It typically appears at the bottom of the screen and updates based on the current context.

mod command_bar;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::command_bar::CommandBar;
